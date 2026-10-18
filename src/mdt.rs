#[derive(Clone, Copy)]
pub struct MdMapping {
    pub md_start: &'static str,
    pub md_end: &'static str,
    pub html_start: &'static str,
    pub html_end: &'static str,
}

static MD_MAP: &[MdMapping] = &[
    MdMapping { md_start: "# ",      md_end: "",   html_start: "<h1>",     html_end: "</h1>",      },
    MdMapping { md_start: "## ",     md_end: "",   html_start: "<h2>",     html_end: "</h2>",      },
    MdMapping { md_start: "### ",    md_end: "",   html_start: "<h3>",     html_end: "</h3>",      },
    MdMapping { md_start: "#### ",   md_end: "",   html_start: "<h4>",     html_end: "</h4>",      },
    MdMapping { md_start: "##### ",  md_end: "",   html_start: "<h5>",     html_end: "</h5>",      },
    MdMapping { md_start: "###### ", md_end: "",   html_start: "<h6>",     html_end: "</h6>",      },
    MdMapping { md_start: "**",      md_end: "**", html_start: "<b>",      html_end: "</b>",       },
    MdMapping { md_start: "__",      md_end: "__", html_start: "<u>",      html_end: "</u>",       },
    MdMapping { md_start: "*",       md_end: "*",  html_start: "<i>",      html_end: "</i>",       },
    MdMapping { md_start: "_",       md_end: "_",  html_start: "<em>",     html_end: "</em>",      },
    MdMapping { md_start: "~~",      md_end: "~~", html_start: "<del>",    html_end: "</del>",     },
    MdMapping { md_start: "`",       md_end: "`",  html_start: "<code>",   html_end: "</code>",    },
];

static TABLE_CSS_STYLE: &str = "border: 2px solid black; text-align: left; min-width: 70px; padding: 8px;";

fn process_inline(text: &str) -> String {
    let mut buf = text.to_string(); 
    for m in MD_MAP {
        if m.md_end.is_empty() {
            continue;
        }
        while let Some(start_idx) = buf.find(m.md_start) {
            if let Some(end_idx) = buf[start_idx + m.md_start.len()..].find(m.md_end) {
                let actual_end_idx = start_idx + m.md_start.len() + end_idx;
                let inner_text = &buf[start_idx + m.md_start.len()..actual_end_idx];
                let replacement = format!("{}{}{}", m.html_start, inner_text, m.html_end);
                buf.replace_range(start_idx..actual_end_idx + m.md_end.len(), &replacement);
            } else {
                break;
            }
        }
    }
    buf
}

pub fn is_mdt(lines: &[&str], i: usize) -> bool {
    if i + 1 >= lines.len() {
        return false;
    }
    let line = lines[i].trim();
    let next = lines[i + 1].trim();
    if line.starts_with('|') && line.contains("---") {
        return true;
    }
    if next.starts_with('|') && next.contains("---") {
        return true;
    }
    return false;
}

fn process_table<F>(lines: &[&str], process_inline: &F) -> String
where
    F: Fn(&str) -> String,
{
    if lines.is_empty() {
        return String::new();
    }
    let mut html = String::new();
    html.push_str("<table>\n");
    let mut rows: Vec<Vec<&str>> = Vec::with_capacity(lines.len());
    for &ln in lines {
        let cells: Vec<&str> = ln
            .split('|')
            .map(|c| c.trim())
            .filter(|c| !c.is_empty())
            .collect();
        rows.push(cells);
    }
    let has_separator = rows.len() >= 2 && rows[1].iter().any(|c| c.contains("---") || c.contains(":-") || c.contains("-:"));
    if has_separator {
        let header = &rows[0];
        html.push_str("  <thead>\n    <tr>");
        for cell in header {
            html.push_str(&format!("<th>{}</th>", process_inline(cell)));
        }
        html.push_str("</tr>\n  </thead>\n");
    }
    if has_separator {
        html.push_str("  <tbody>\n");
    }
    for (ridx, row) in rows.iter().enumerate() {
        if has_separator && ridx == 1 {
            continue;
        }
        if !has_separator && ridx == 0 {
            println!("no-op, first row is body");
        }
        html.push_str("    <tr>");
        for cell in row {
            html.push_str(&format!("<td style=\"{}\">{}</td>", TABLE_CSS_STYLE, process_inline(cell)));
        }
        html.push_str("</tr>\n");
    }
    if has_separator {
        html.push_str("  </tbody>\n");
    }
    html.push_str("</table>\n");
    html
}

pub fn mdt_from_str(md_table: &str) -> String {
    if md_table.trim().is_empty() {
        return String::new();
    }
    let lines: Vec<&str> = md_table.lines().collect();
    process_table(&lines, &|s: &str| process_inline(s))
}

