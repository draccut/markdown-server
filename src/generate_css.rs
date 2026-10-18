use std::fmt::Write;

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

#[derive(Clone, Copy)]
pub enum Theme {
    LightModern = 0,
    BlackWhite = 1,
    DarkBlue = 2,
    GreenBlack = 3,
}

/**
 * Theme zur Compile-Zeit über Cargo-Features
 */
pub const ACTIVE_THEME: Theme = {
    if cfg!(feature = "theme-light-modern") {
        Theme::LightModern
    } else if cfg!(feature = "theme-black-white") {
        Theme::BlackWhite
    } else if cfg!(feature = "theme-green-black") {
        Theme::GreenBlack
    } else {
        Theme::DarkBlue
    }
};

const THEME_COUNT: usize = 4;

/* ------------------------------------- Das sind Farben --------------------- */
const BG_1: [u64; THEME_COUNT] =        [0xF8FAFC, 0x000000, 0x0B1220, 0x0A0A0A];
const BG_2: [u64; THEME_COUNT] =        [0xEEF2FF, 0x111111, 0x071018, 0x050505];
const PANEL: [u64; THEME_COUNT] =       [0xFFFFFF, 0x1A1A1A, 0x0F172A, 0x111111];
const TEXT: [u64; THEME_COUNT] =        [0x0F172A, 0xFFFFFF, 0xE6EEF8, 0x22FF22];
const MUTED: [u64; THEME_COUNT] =       [0x64748B, 0xAAAAAA, 0x94A3B8, 0x15AA15];
const ACCENT: [u64; THEME_COUNT] =      [0x7C3AED, 0xFFFFFF, 0x38BDF8, 0x22FF22];
const LINK_BG: [u64; THEME_COUNT] =     [0x7C3AED, 0x333333, 0x1E3A5F, 0x003300];
const CODE_BG: [u64; THEME_COUNT] =     [0xF1F5F9, 0x222222, 0x1E293B, 0x000000];
const BUTTON_BG: [u64; THEME_COUNT] =   [0x7C3AED, 0xFFFFFF, 0x0EA5E9, 0x22FF22];
const BUTTON_FG: [u64; THEME_COUNT] =   [0xFFFFFF, 0x000000, 0x0B1220, 0x000000];
const BORDER: [u64; THEME_COUNT] =      [0xE2E8F0, 0x444444, 0x1E293B, 0x22FF22];

fn hex(c: u64) -> String {
    format!("#{:06X}", c & 0xFFFFFF)
}

pub fn generate_css(theme: Theme) -> String {
    let t = theme as usize;
    let mut b = StyleBuilder::new();

    b.add_rule(":root", &[
        ("--bg-1", &hex(BG_1[t])),
               ("--bg-2", &hex(BG_2[t])),
               ("--panel", &hex(PANEL[t])),
               ("--text", &hex(TEXT[t])),
               ("--muted", &hex(MUTED[t])),
               ("--accent", &hex(ACCENT[t])),
               ("--link-bg", &hex(LINK_BG[t])),
               ("--code-bg", &hex(CODE_BG[t])),
               ("--button-bg", &hex(BUTTON_BG[t])),
               ("--button-fg", &hex(BUTTON_FG[t])),
               ("--border", &hex(BORDER[t])),
               ("--radius", "8px"),
    ]);

    b.add_rule("*, *::before, *::after", &[
        ("box-sizing", "border-box"),
    ]);

    b.add_rule("html, body", &[
        ("height", "100%"),
               ("margin", "0"),
               ("padding", "0"),
               (
                   "font-family",
                   "Inter, system-ui, -apple-system, \"Segoe UI\", Roboto, \"Helvetica Neue\", Arial, sans-serif",
               ),
               ("color", "var(--text)"),
               (
                   "background",
                   "linear-gradient(180deg, var(--bg-1) 0%, var(--bg-2) 100%)",
               ),
               ("-webkit-font-smoothing", "antialiased"),
    ]);

    b.add_rule(".container", &[
        ("max-width", "960px"),
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
               ("font-size", "0.95rem"),
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
               ("line-height", "1.65"),
               ("margin", "0"),
               ("padding", "0"),
    ]);

    b.add_rule(".markdown-content pre, .markdown-content code", &[
        ("background", "var(--code-bg)"),
               ("padding", "0.35rem 0.5rem"),
               ("border-radius", "6px"),
               ("color", "var(--text)"),
               (
                   "font-family",
                   "ui-monospace, SFMono-Regular, Menlo, Monaco, \"Roboto Mono\", monospace",
               ),
               ("font-size", "0.92rem"),
               ("border", "1px solid var(--border)"),
    ]);

    b.add_rule(".markdown-content img", &[
        ("max-width", "100%"),
               ("border-radius", "6px"),
               ("display", "block"),
               ("margin", "0.6rem 0"),
    ]);
