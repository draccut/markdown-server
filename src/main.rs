mod my_theme;
mod style;
mod my_page;
mod highlighter;
mod keywords;
mod col;
mod codestyle;
mod language;
mod mdt;

use crate::my_page::{page_template, get_subcontent_tags};
use crate::my_theme::ACTIVE_THEME;
use crate::mdt::{mdt_from_str, is_mdt};
use crate::codestyle::code_style;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use pulldown_cmark::{html, Options, Parser};
use std::fs;
use std::path::{Path, Component};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use html_escape::encode_text;
use mime::TEXT_HTML_UTF_8;

const MD_ROOT: &str = "md";
const BIND_ADDR: &str = "127.0.0.1:3034";

#[derive(Clone)]
struct FileEntry {
    name: String,
    mtime: SystemTime,
    content: String,
    is_markdown: bool,
}

type FileStore = Arc<Mutex<Vec<FileEntry>>>;

fn encode_filename(name: &str) -> String {
    utf8_percent_encode(name, NON_ALPHANUMERIC).to_string()
}

fn is_safe_relative_path(rel: &str) -> bool {
    if rel.is_empty() || rel.starts_with('/') || rel.starts_with('\\') {
        return false;
    }
    for comp in Path::new(rel).components() {
        match comp {
            Component::Normal(_) => {}
            _ => return false,
        }
    }
    true
}

enum Segment {
    Text(String),
    Code { lang: Option<String>, text: String },
    Table(String),
}

fn split_markdown_sections(md: &str) -> Vec<Segment> {
    let lines: Vec<&str> = md.lines().collect();
    let mut segments = Vec::new();
    let mut normal_buf = String::new();
    let mut code_buf = String::new();
    let mut i = 0;
    fn flush_normal(segments: &mut Vec<Segment>, buf: &mut String) {
        if !buf.is_empty() {
            segments.push(Segment::Text(buf.clone()));
            buf.clear();
        }
    }
    fn collect_table(lines: &[&str], i: &mut usize) -> String {
        let mut table_lines = Vec::new();
        while *i < lines.len() && lines[*i].trim().starts_with('|') {
            table_lines.push(lines[*i]);
            *i += 1;
        }
        table_lines.join("\n")
    }
    fn collect_fenced_code(lines: &[&str], i: &mut usize, code_buf: &mut String) {
        code_buf.clear();
        while *i < lines.len() {
            let line = lines[*i];
            if line.starts_with("```") {
                *i += 1;
                break;
            }
            code_buf.push_str(line);
            code_buf.push('\n');
            *i += 1;
        }
    }
    fn collect_indented_code(lines: &[&str], i: &mut usize, code_buf: &mut String) {
        code_buf.clear();
        while *i < lines.len() {
            let line = lines[*i];
            let is_indented = line.starts_with('\t') || line.starts_with("    ");
            if !is_indented {
                break;
            }
            code_buf.push_str(line.trim_start());
            code_buf.push('\n');
            *i += 1;
        }
    }
    while i < lines.len() {
        let line = lines[i];
        if is_mdt(&lines, i) {
            flush_normal(&mut segments, &mut normal_buf);
            let table_text = collect_table(&lines, &mut i);
            segments.push(Segment::Table(table_text));
            continue;
        }
        if line.starts_with("```") {
            flush_normal(&mut segments, &mut normal_buf);
            let lang = line.trim_start_matches("```").trim();
            let fenced_lang = if lang.is_empty() {
                None
            } else {
                Some(lang.to_string())
            };
            i += 1;
            collect_fenced_code(&lines, &mut i, &mut code_buf);
            segments.push(Segment::Code {
                lang: fenced_lang,
                text: code_buf.clone(),
            });
            continue;
        }
        let is_indented = line.starts_with('\t') || line.starts_with("    ");
        if is_indented {
            flush_normal(&mut segments, &mut normal_buf);
            collect_indented_code(&lines, &mut i, &mut code_buf);
            segments.push(Segment::Code {
                lang: None,
                text: code_buf.clone(),
            });
            continue;
        }
        normal_buf.push_str(line);
        normal_buf.push('\n');
        i += 1;
    }
    flush_normal(&mut segments, &mut normal_buf);
    segments
}

fn render_code_block(code: &str, _lang: Option<&str>) -> String {
    code_style(code)
}

fn render_markdown(md: &str) -> String {
    let sections = split_markdown_sections(md);
    let mut options = Options::empty();
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    let mut html_output = String::new();
    for seg in sections {
        match seg {
            Segment::Text(text_md) => {
                let parser = Parser::new_ext(&text_md, options);
                html::push_html(&mut html_output, parser);
            }
            Segment::Code { lang, text } => {
                html_output.push_str(&render_code_block(&text, lang.as_deref()));
            }
            Segment::Table(table) => {
                html_output.push_str(&mdt_from_str(&table));
            }
        }
    }
    get_subcontent_tags(&html_output)
}

fn load_all_files() -> std::io::Result<Vec<FileEntry>> {
    let root = Path::new(MD_ROOT);
    let mut entries = Vec::new();
    collect_files(root, root, &mut entries)?;
    entries.sort_by(|a, b| b.mtime.cmp(&a.mtime));
    Ok(entries)
}

fn collect_files(root: &Path, dir: &Path, out: &mut Vec<FileEntry>) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_files(root, &path, out)?;
            continue;
        }
        if !path.is_file() {
            continue;
        }
        let rel = match path.strip_prefix(root) {
            Ok(r) => r,
            Err(_) => continue,
        };
        let name = match rel.to_str() {
            Some(s) => s.replace('\\', "/"),
            None => continue,
        };
        if !is_safe_relative_path(&name) {
            println!("Überspringe unsicheren Pfad: {}", &name);
            continue;
        }
        let meta = fs::metadata(&path)?;
        let mtime = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        let content = fs::read_to_string(&path).unwrap_or_default();
        let lower = name.to_ascii_lowercase();
        let is_markdown = lower.ends_with(".md") || lower.ends_with(".markdown");
        if is_markdown {
            out.push(FileEntry {
                name,
                mtime,
                content,
                is_markdown,
            });
        }
    }
    Ok(())
}

#[get("/")]
async fn index(store: web::Data<FileStore>) -> Result<impl Responder> {
    let files = store.lock().unwrap();
    let mut list_html = String::from(
        "<h1>Dateien (neueste zuerst)</h1>\n<ul class=\"file-list\">\n",
    );
    for f in files.iter() {
        let enc = encode_filename(&f.name);
        list_html.push_str(&format!(
            r#"<li><a href="/{enc}">{name}</a></li>"#,
            enc = enc,
            name = encode_text(&f.name)
        ));
    }
    list_html.push_str("</ul>\n");

    let html = page_template("Index", &list_html, ACTIVE_THEME);
    Ok(HttpResponse::Ok()
    .content_type(TEXT_HTML_UTF_8.to_string())
    .body(html))
}

#[get("/{filename:.*}")]
async fn serve_file(
    path: web::Path<(String,)>,
    store: web::Data<FileStore>,
) -> Result<impl Responder> {
    let filename = &path.0;
    if !is_safe_relative_path(filename) {
        let html = page_template(
            "403",
            "<h1>403 – Ungültiger Pfad</h1>",
            ACTIVE_THEME,
        );
        return Ok(HttpResponse::Forbidden()
        .content_type(TEXT_HTML_UTF_8.to_string())
        .body(html));
    }
    let files = store.lock().unwrap();
    let entry = files.iter().find(|e| e.name == *filename);
    match entry {
        Some(f) if f.is_markdown => {
            let html_body = render_markdown(&f.content);
            let html = page_template(
                &format!("View – {}", encode_text(filename)),
                                     &html_body,
                                     ACTIVE_THEME,
            );
            Ok(HttpResponse::Ok()
            .content_type(TEXT_HTML_UTF_8.to_string())
            .body(html))
        }
        Some(f) if !f.content.is_empty() => Ok(HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(f.content.clone())),
        Some(_) => Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(format!("Binary: {}", encode_text(filename)))),
        None => {
            let html = page_template(
                "404",
                "<h1>404 – Datei nicht gefunden</h1>",
                ACTIVE_THEME,
            );
            Ok(HttpResponse::NotFound()
            .content_type(TEXT_HTML_UTF_8.to_string())
            .body(html))
        }
    }
}

#[get("/_internal/reload")]
async fn internal_reload(store: web::Data<FileStore>) -> Result<impl Responder> {
    match load_all_files() {
        Ok(new_files) => {
            let count = new_files.len();
            let mut guard = store.lock().unwrap();
            *guard = new_files;
            Ok(HttpResponse::Ok().body(format!("reloaded ({} files)", count)))
        }
        Err(e) => {
            let msg = format!("Reload-Fehler: {}", e);
            println!("{}", &msg);
            Ok(HttpResponse::InternalServerError().body(msg))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let root = Path::new(MD_ROOT);
    if !root.exists() || !root.is_dir() {
        let error_msg = format!(
            "Fehler: MD_ROOT '{}' existiert nicht oder ist kein Verzeichnis.",
            MD_ROOT
        );
        eprintln!("{}", error_msg);
        println!("{}", &error_msg);
        std::process::exit(1);
    }
    let files = match load_all_files() {
        Ok(f) => f,
        Err(e) => {
            let msg = format!("Konnte Dateien nicht laden: {}", e);
            eprintln!("{}", msg);
            println!("{}", &msg);
            std::process::exit(1);
        }
    };
    println!("Starte Server auf http://{}/", BIND_ADDR);
    println!("{} Dateien geladen (path-safe).", files.len());
    for f in &files {
        println!(" --> /{}", f.name);
    }
    let store: FileStore = Arc::new(Mutex::new(files));
    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(store.clone()))
        .service(index)
        .service(serve_file)
        .service(internal_reload)
    })
    .bind(BIND_ADDR)?
    .run()
    .await
}
