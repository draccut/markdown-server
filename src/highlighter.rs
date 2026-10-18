use crate::keywords::{
    ASM_8051_KEYWORDS, ASM_X86_KEYWORDS, KEYWORDS_C, DOCKER_KEYWORDS, JS_KEYWORDS,
    PYTHON_KEYWORDS, RUST_KEYWORDS, SHELL_KEYWORDS, SQL_KEYWORDS, Keyword, 
    DOCKER_COMPOSE_KEYWORDS
};

use crate::language::Language;
use crate::col::HighlightConfig;

#[derive(Debug, Clone)]
struct ReplaceMap<'a> {
    pub to_replace: char,
    pub replacement: &'a str,
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '$'
}

fn is_keyword(word: &str, keywords: &[Keyword]) -> bool {
    keywords.iter().any(|kw| kw.0 == word)
}

fn append_line_number(line_nr: &mut u64, result: &mut String, con: &HighlightConfig) {
    *line_nr += 1;
    if *line_nr < 10 {
        result.push_str(&format!("{}{}   {}", con.color_numbers, *line_nr, con.color_reset));
    } else if *line_nr < 100 {
        result.push_str(&format!("{}{}  {}",  con.color_numbers, *line_nr, con.color_reset));
    } else {
        result.push_str(&format!("{}{} {}",   con.color_numbers, *line_nr, con.color_reset));
    }
}

static RULES: [ReplaceMap; 2] = [
    ReplaceMap {
        to_replace: '<',
        replacement: "&lt;",
    },
    ReplaceMap {
        to_replace: '>',
        replacement: "&gt;",
    }
];

pub fn highlight_line(line_nr: &mut u64, raw_line: &str, lang: Language, config: &HighlightConfig, result: &mut String) {
    let mut line = String::with_capacity(raw_line.len() * 2);
    for current_char in raw_line.chars() {
        let mut replaced = false; 
        for rule in &RULES {
            if current_char == rule.to_replace {
                line.push_str(rule.replacement);
                replaced = true;
                break;
            }
        }
        if !replaced {
            line.push(current_char);
        }
    }
    append_line_number(line_nr, result, config);
    let mut p = line.chars().peekable();
    while let Some(mut ch) = p.next() {
        if lang == Language::Clang {
            if ch == '#' {
                let mut index: u32 = 0;
                result.push_str(config.color_header);
                result.push('#');
                while let Some(&c) = p.peek() {
                    index += 1;
                    if c == ' ' {
                        break;
                    }
                    if index > 70 {
                        return;
                    }
                    result.push(p.next().unwrap());
                }
                result.push_str(config.color_reset);
            }
        }
        match lang {
            Language::AsmX86 | Language::Asm8051 => {
                if ch == ';' {
                    result.push_str(config.color_comment);
                    result.push(';');
                    for c in p { result.push(c); }
                    result.push_str(config.color_reset);
                    break;
                }
            }
            Language::Python | Language::Shell => {
                if ch == '#' {
                    result.push_str(config.color_comment);
                    result.push('#');
                    for c in p { result.push(c); }
                    result.push_str(config.color_reset);
                    break;
                }
            }
            Language::JavaScript | Language::Docker => {
                if ch == '/' && p.peek() == Some(&'/') {
                    result.push_str(config.color_comment);
                    result.push_str("//");
                    p.next();
                    for c in p {
                        result.push(c);
                    }
                    result.push_str(config.color_reset);
                    break;
                }
            }
            Language::Clang => {
                if ch == '/' {
                    match p.peek() {
                        Some(&'/') => {
                            result.push_str(config.color_comment);
                            result.push_str("//");
                            p.next();
                            for c in p {
                                result.push(c);
                            }
                            result.push_str(config.color_reset);
                            break;
                        }
                        Some(&'*') => {
                            result.push_str(config.color_comment);
                            result.push_str("/*");
                            p.next();
                            while let Some(c) = p.next() {
                                if c == '*' && p.peek() == Some(&'/') {
                                    result.push('*');
                                    result.push('/');
                                    p.next();
                                    result.push_str(config.color_reset);
                                    break;
                                }
                                result.push(c);
                            }
                            continue;
                        }
                        _ => {}
                    }
                }
            }
            Language::Sql => {
                if ch == '/' && p.peek() == Some(&'*') {
                    result.push_str(config.color_comment);
                    result.push_str("/*");
                    p.next();
                    while let Some(c) = p.next() {
                        if c == '*' && p.peek() == Some(&'/') {
                            result.push('*');
                            result.push('/');
                            p.next();
                            result.push_str(config.color_reset);
                            break;
                        }
                        result.push(c);
                    }
                    continue;
                }
            }
            _ => {}
        }
        if ch == '"' || ch == '\'' {
            let quote = ch;
            result.push_str(config.color_string);
            result.push(quote);
            ch = match p.next() {
                Some(c) => c,
                None => { result.push_str(config.color_reset); break; }
            };
            while ch != quote {
                if ch == '\\' {
                    result.push('\\');
                    if let Some(next) = p.next() {
                        result.push(next);
                        ch = match p.next() { Some(c) => c, None => break };
                        continue;
                    }
                } else {
                    result.push(ch);
                }
                ch = match p.next() { Some(c) => c, None => break };
            }
            if ch == quote {
                result.push(quote);
            }
            result.push_str(config.color_reset);
            continue;
        }
        if lang == Language::Html && ch == '<' {
            result.push_str(config.color_tag);
            result.push('<');
            ch = match p.next() { Some(c) => c, None => { result.push_str(config.color_reset); break; } };
            while ch != '>' {
                if ch == '"' {
                    let quote = ch;
                    result.push_str(config.color_string);
                    result.push(quote);
                    ch = match p.next() { Some(c) => c, None => break };
                    while ch != quote {
                        if ch == '\\' {
                            result.push('\\');
                            if let Some(n) = p.next() { result.push(n); ch = match p.next() { Some(c)=>c, None=>break }; continue; }
                        } else {
                            result.push(ch);
                        }
                        ch = match p.next() { Some(c) => c, None => break };
                    }
                    if ch == quote {
                        result.push(quote);
                    }
                    result.push_str(config.color_tag);
                } else {
                    result.push(ch);
                }
                ch = match p.next() { Some(c) => c, None => break };
            }
            if ch == '>' {
                result.push('>');
            }
            result.push_str(config.color_reset);
            continue;
        }
        if is_word_char(ch) {
            let mut buf = String::new();
            buf.push(ch);
            while let Some(&next) = p.peek() {
                if is_word_char(next) && buf.len() < 250 {
                    buf.push(next);
                    p.next();
                } else {
                    break;
                }
            }
            let is_kw = match lang {
                Language::Python          => is_keyword(&buf, PYTHON_KEYWORDS),
                Language::JavaScript      => is_keyword(&buf, JS_KEYWORDS),
                Language::Clang          => is_keyword(&buf, KEYWORDS_C),
                Language::Shell           => is_keyword(&buf, SHELL_KEYWORDS),
                Language::Docker          => is_keyword(&buf, DOCKER_KEYWORDS),
                Language::AsmX86          => is_keyword(&buf, ASM_X86_KEYWORDS),
                Language::Asm8051         => is_keyword(&buf, ASM_8051_KEYWORDS),
                Language::Sql             => is_keyword(&buf, SQL_KEYWORDS),
                Language::Rust            => is_keyword(&buf, RUST_KEYWORDS),
                Language::LangDockerCompose   => is_keyword(&buf, DOCKER_COMPOSE_KEYWORDS),
                _                         => false,
            };
            if is_kw {
                result.push_str(config.color_keyword);
                result.push_str(&buf);
                result.push_str(config.color_reset);
            } else {
                result.push_str(&buf);
            }
            continue;
        }
        result.push(ch);
    }
    result.push_str(config.new_line);
}

