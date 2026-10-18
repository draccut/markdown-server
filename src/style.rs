use std::fmt::Write;

use crate::my_theme::{
    self, Theme, ACCENT, BG_1, BG_2, BORDER, BUTTON_BG, BUTTON_FG, CODE_BG, FONT_FAMILY, FONT_MONO,
    FONT_SIZE_BTN_MOBILE, FONT_SIZE_CODE, FONT_SIZE_FOOTER, FONT_SIZE_SM, LINE_HEIGHT, LINK_BG,
    MAX_WIDTH, MUTED, PANEL, RADIUS, TEXT,
};

pub struct StyleBuilder {
    css: String,
}

impl StyleBuilder {
    pub fn new() -> Self {
        Self { css: String::new() }
    }

    pub fn add_rule(&mut self, selector: &str, properties: &[(&str, &str)]) {
        let _ = writeln!(self.css, "{} {{", selector);
        for (prop, value) in properties {
            let _ = writeln!(self.css, "  {}: {};", prop, value);
        }
        let _ = writeln!(self.css, "}}");
    }

    pub fn add_media_query(&mut self, query: &str, rules: &[(&str, &[(&str, &str)])]) {
        let _ = writeln!(self.css, "@media {} {{", query);
        for (selector, properties) in rules {
            let _ = writeln!(self.css, "  {} {{", selector);
            for (prop, value) in *properties {
                let _ = writeln!(self.css, "    {}: {};", prop, value);
            }
            let _ = writeln!(self.css, "  }}");
        }
        let _ = writeln!(self.css, "}}");
    }

    pub fn build(self) -> String {
        self.css
    }
}

impl Default for StyleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub fn generate_css(theme: Theme) -> String {
    let t = theme as usize;
    let mut b = StyleBuilder::new();
    let bg1 = my_theme::hex(BG_1[t]);
    let bg2 = my_theme::hex(BG_2[t]);
    let panel = my_theme::hex(PANEL[t]);
    let text = my_theme::hex(TEXT[t]);
    let muted = my_theme::hex(MUTED[t]);
    let accent = my_theme::hex(ACCENT[t]);
    let link_bg = my_theme::hex(LINK_BG[t]);
    let code_bg = my_theme::hex(CODE_BG[t]);
    let button_bg = my_theme::hex(BUTTON_BG[t]);
    let button_fg = my_theme::hex(BUTTON_FG[t]);
    let border = my_theme::hex(BORDER[t]);
    b.add_rule(":root", &[
        ("--bg-1", &bg1),
        ("--bg-2", &bg2),
        ("--panel", &panel),
        ("--text", &text),
        ("--muted", &muted),
        ("--accent", &accent),
        ("--link-bg", &link_bg),
        ("--code-bg", &code_bg),
        ("--button-bg", &button_bg),
        ("--button-fg", &button_fg),
        ("--border", &border),
        ("--radius", RADIUS),
    ]);
    b.add_rule("*, *::before, *::after", &[
        ("box-sizing", "border-box"),
    ]);
    b.add_rule("html, body", &[
        ("height", "100%"),
        ("margin", "0"),
        ("padding", "0"),
        ("font-family", FONT_FAMILY),
        ("color", "var(--text)"),
        (
            "background",
            "linear-gradient(180deg, var(--bg-1) 0%, var(--bg-2) 100%)",
        ),
        ("-webkit-font-smoothing", "antialiased"),
    ]);
    b.add_rule(".container", &[
        ("max-width", MAX_WIDTH),
        ("margin", "0.75rem auto"),
        ("padding", "0.5rem"),
        ("width", "100%"),
    ]);
    b.add_rule(".topbar", &[
        ("display", "block"),
        ("width", "100%"),
        ("padding", "0"),
        ("margin", "0 0 0.6rem 0"),
        ("border-radius", "var(--radius)"),
        ("background", "transparent"),
        ("box-shadow", "none"),
    ]);
    b.add_rule(".topbar a", &[
        ("display", "block"),
        ("width", "100%"),
        ("text-align", "center"),
        ("text-decoration", "none"),
        ("color", "var(--button-fg)"),
        ("background", "var(--button-bg)"),
        ("padding", "0.55rem 0.75rem"),
        ("border-radius", "var(--radius)"),
        ("font-weight", "600"),
        ("font-size", FONT_SIZE_SM),
        ("transition", "opacity 0.15s"),
        ("border", "none"),
    ]);
    b.add_rule(".topbar a:hover", &[
        ("opacity", "0.85"),
    ]);
    b.add_rule(".panel", &[
        ("background", "var(--panel)"),
        ("padding", "0.9rem"),
        ("border-radius", "var(--radius)"),
        ("box-shadow", "0 4px 14px rgba(0,0,0,0.14)"),
        ("border", "1px solid var(--border)"),
        ("width", "100%"),
        ("overflow", "hidden"),
        ("position", "relative"),
    ]);
    b.add_rule("h1, h2, h3", &[
        ("color", "var(--accent)"),
        ("margin", "0 0 0.6rem 0"),
        ("border-bottom", "2px solid var(--accent)"),
        ("padding-bottom", "0.3rem"),
    ]);
    b.add_rule("a.inline-link", &[
        ("color", "var(--button-fg)"),
        ("background", "var(--button-bg)"),
        ("padding", "0.15rem 0.4rem"),
        ("border-radius", "6px"),
        ("text-decoration", "none"),
        ("font-weight", "500"),
    ]);
    b.add_rule(".markdown-content", &[
        ("color", "var(--text)"),
        ("line-height", LINE_HEIGHT),
        ("margin", "0"),
        ("padding", "0"),
    ]);
    b.add_rule(".markdown-content pre, .markdown-content code", &[
        ("background", "var(--code-bg)"),
        ("padding", "0.35rem 0.5rem"),
        ("border-radius", "6px"),
        ("color", "var(--text)"),
        ("font-family", FONT_MONO),
        ("font-size", FONT_SIZE_CODE),
        ("border", "1px solid var(--border)"),
    ]);
    b.add_rule(".markdown-content img", &[
        ("max-width", "100%"),
        ("border-radius", "6px"),
        ("display", "block"),
        ("margin", "0.6rem 0"),
    ]);
    b.add_rule(".file-list", &[
        ("list-style", "none"),
        ("padding", "0"),
        ("margin", "0"),
        ("display", "grid"),
        ("gap", "0.4rem"),
        ("width", "100%"),
    ]);
    b.add_rule(".file-list li", &[
        ("margin", "0"),
        ("padding", "0"),
        ("width", "100%"),
    ]);
    b.add_rule(".file-list li a", &[
        ("display", "block"),
        ("width", "100%"),
        ("padding", "0.6rem 0.85rem"),
        ("border-radius", "var(--radius)"),
        ("text-decoration", "none"),
        ("text-align", "left"),
        ("color", "var(--text)"),
        ("background", "var(--code-bg)"),
        ("border", "1px solid var(--border)"),
        ("transition", "background 0.15s, color 0.15s"),
    ]);
    b.add_rule(".file-list li a:hover", &[
        ("background", "var(--link-bg)"),
        ("color", "var(--button-fg)"),
    ]);
    b.add_rule("button, .btn", &[
        ("display", "block"),
        ("width", "100%"),
        ("text-align", "center"),
        ("background", "var(--button-bg)"),
        ("color", "var(--button-fg)"),
        ("padding", "0.65rem 1.2rem"),
        ("font-size", FONT_SIZE_SM),
        ("font-weight", "600"),
        ("border-radius", "var(--radius)"),
        ("border", "none"),
        ("cursor", "pointer"),
        ("text-decoration", "none"),
        ("text-transform", "uppercase"),
        ("letter-spacing", "0.03em"),
        ("transition", "opacity 0.15s"),
        ("margin", "0"),
    ]);
    b.add_rule("button:hover, .btn:hover", &[
        ("opacity", "0.9"),
    ]);
    b.add_rule(".markdown-content + hr", &[
        ("margin", "1rem 0 0.75rem 0"),
        ("border", "none"),
        ("border-top", "1px solid var(--border)"),
        ("height", "0"),
        ("display", "block"),
    ]);
    b.add_rule(".back-wrap", &[
        ("display", "block"),
        ("width", "100%"),
        ("margin-top", "0"),
        ("padding-top", "0"),
        ("clear", "both"),
    ]);
    b.add_rule("footer", &[
        ("margin-top", "1rem"),
        ("padding", "0.4rem 0"),
        ("color", "var(--muted)"),
        ("font-size", FONT_SIZE_FOOTER),
        ("text-align", "center"),
        ("clear", "both"),
    ]);
    b.add_media_query("(max-width: 640px)", &[
        (".container", &[("margin", "0.4rem"), ("padding", "0.35rem")]),
        (".panel", &[("padding", "0.7rem")]),
        (
            "button, .btn",
            &[("padding", "0.55rem 1rem"), ("font-size", FONT_SIZE_BTN_MOBILE)],
        ),
        (".file-list li a", &[("padding", "0.55rem 0.7rem")]),
        (".topbar a", &[("padding", "0.5rem 0.65rem")]),
    ]);
    b.build()
}
