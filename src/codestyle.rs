use std::collections::{HashMap, HashSet};

use crate::keywords::{
    ASM_8051_KEYWORDS, ASM_X86_KEYWORDS, KEYWORDS_C, DOCKER_KEYWORDS, JS_KEYWORDS,
    PYTHON_KEYWORDS, RUST_KEYWORDS, SHELL_KEYWORDS, SQL_KEYWORDS, Keyword,
    DOCKER_COMPOSE_KEYWORDS,
};

use crate::highlighter::highlight_line;
use crate::col::HighlightConfig;
use crate::language::Language;

fn normalize_token(token: &str) -> String {
    let lower = token.to_lowercase();
    lower
        .trim_matches(|c: char| !c.is_alphanumeric())
        .to_string()
}

fn build_keyword_sets() -> Vec<(&'static str, HashSet<String>)> {
    let mut v: Vec<(&'static str, HashSet<String>)> = Vec::new();
    macro_rules! add {
        ($name:expr, $slice:expr) => {{
            let set: HashSet<String> = $slice.iter()
                .map(|k: &Keyword| normalize_token(k.0))
                .collect();
            v.push(($name, set));
        }};
    }
    add!("C", KEYWORDS_C);
    add!("Rust", RUST_KEYWORDS);
    add!("Python", PYTHON_KEYWORDS);
    add!("JavaScript", JS_KEYWORDS);
    add!("Shell", SHELL_KEYWORDS);
    add!("SQL", SQL_KEYWORDS);
    add!("Docker", DOCKER_KEYWORDS);
    add!("Docker Compose", DOCKER_COMPOSE_KEYWORDS);
    add!("ASM x86", ASM_X86_KEYWORDS);
    add!("ASM 8051", ASM_8051_KEYWORDS);
    v
}

pub fn count_keywords_per_language(text: &str, normalize: bool) -> HashMap<&'static str, usize> {
    let keyword_sets = build_keyword_sets();
    let mut counts: HashMap<&'static str, usize> = HashMap::new();
    for (name, _) in &keyword_sets {
        counts.insert(*name, 0usize);
    }
    for raw_token in text.split_whitespace() {
        let token = if normalize {
            normalize_token(raw_token)
        } else {
            raw_token.to_string()
        };
        if token.is_empty() {
            continue;
        }
        for (name, set) in &keyword_sets {
            if set.contains(&token) {
                *counts.get_mut(name).unwrap() += 1;
            }
        }
    }
    counts
}

fn language_from_str(name: &str) -> Language {
    match name {
        "Python" => Language::Python,
        "JavaScript" => Language::JavaScript,
        "C" => Language::Clang,
        "Markdown" => Language::Markdown,
        "Html" | "HTML" => Language::Html,
        "Docker" => Language::Docker,
        "Shell" => Language::Shell,
        "ASM x86" => Language::AsmX86,
        "ASM 8051" => Language::Asm8051,
        "SQL" => Language::Sql,
        "Rust" => Language::Rust,
        "Docker Compose" => Language::LangDockerCompose,
        _ => Language::Plain,
    }
}

fn count_language_keywords(text: &str) -> Language {
    let counts = count_keywords_per_language(text, true);
    let mut selected_lang: Language = Language::Plain;
    let mut score: usize = 0;
    for (lang, cnt) in counts {
        if score < cnt {
            selected_lang = language_from_str(lang);
            score = cnt;
        }
    }
    selected_lang
}

pub fn code_style(source: &str) -> String {
    let lang = count_language_keywords(source);
    let mut output: String = String::with_capacity(source.len() * 2);
    let config = HighlightConfig::html_colors();
    let mut line_number: u64 = 0;
    for line in source.lines() {
        highlight_line(&mut line_number, line, lang, &config, &mut output);
    }
    let result = format!("{}{}{}", "<pre>", output, "</pre>");
    result
}
