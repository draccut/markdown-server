#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    #[cfg(feature = "theme-light-modern")]
    LightModern = 0,
    #[cfg(feature = "theme-black-white")]
    BlackWhite =  1,
    #[cfg(feature = "theme-dark-blue")]
    DarkBlue =    2,
    #[cfg(feature = "theme-green-black")]
    GreenBlack =  3,
}

pub const THEME_COUNT: usize = 4;

pub const BG_1:         [u64; THEME_COUNT] = [0xF8FAFC, 0x000000, 0x0B1220, 0x0A0A0A];
pub const BG_2:         [u64; THEME_COUNT] = [0xEEF2FF, 0x111111, 0x071018, 0x050505];
pub const PANEL:        [u64; THEME_COUNT] = [0xFFFFFF, 0x1A1A1A, 0x0F172A, 0x111111];
pub const TEXT:         [u64; THEME_COUNT] = [0x0F172A, 0xFFFFFF, 0xE6EEF8, 0x22FF22];
pub const MUTED:        [u64; THEME_COUNT] = [0x64748B, 0xAAAAAA, 0x94A3B8, 0x15AA15];
pub const ACCENT:       [u64; THEME_COUNT] = [0x7C3AED, 0xFFFFFF, 0x38BDF8, 0x22FF22];
pub const LINK_BG:      [u64; THEME_COUNT] = [0x7C3AED, 0x333333, 0x1E3A5F, 0x003300];
pub const CODE_BG:      [u64; THEME_COUNT] = [0xF1F5F9, 0x222222, 0x1E293B, 0x000000];
pub const BUTTON_BG:    [u64; THEME_COUNT] = [0x7C3AED, 0xFFFFFF, 0x0EA5E9, 0x22FF22];
pub const BUTTON_FG:    [u64; THEME_COUNT] = [0xFFFFFF, 0x000000, 0x0B1220, 0x000000];
pub const BORDER:       [u64; THEME_COUNT] = [0xE2E8F0, 0x444444, 0x1E293B, 0x22FF22];

pub const FONT_FAMILY: &str =
    "Inter, system-ui, -apple-system, \"Segoe UI\", Roboto, \"Helvetica Neue\", Arial, sans-serif";
pub const FONT_MONO: &str =
    "ui-monospace, SFMono-Regular, Menlo, Monaco, \"Roboto Mono\", monospace";

#[allow(dead_code)]
pub const FONT_SIZE_BASE: &str = "1rem";
pub const FONT_SIZE_SM: &str = "0.95rem";
pub const FONT_SIZE_CODE: &str = "0.92rem";
pub const FONT_SIZE_FOOTER: &str = "0.85rem";
pub const FONT_SIZE_BTN_MOBILE: &str = "0.9rem";

pub const RADIUS: &str = "8px";
pub const MAX_WIDTH: &str = "960px";
pub const LINE_HEIGHT: &str = "1.65";

pub fn hex(c: u64) -> String {
    format!("#{:06X}", c & 0xFFFFFF)
}

#[cfg(feature = "theme-light-modern")]
pub const ACTIVE_THEME: Theme = Theme::LightModern;

#[cfg(all(feature = "theme-black-white", not(feature = "theme-light-modern")))]
pub const ACTIVE_THEME: Theme = Theme::BlackWhite;

#[cfg(all(
feature = "theme-green-black",
not(feature = "theme-light-modern"),
          not(feature = "theme-black-white")
))]
pub const ACTIVE_THEME: Theme = Theme::GreenBlack;

#[cfg(all(
feature = "theme-dark-blue",
not(feature = "theme-light-modern"),
          not(feature = "theme-black-white"),
          not(feature = "theme-green-black")
))]
pub const ACTIVE_THEME: Theme = Theme::DarkBlue;

#[cfg(not(any(
feature = "theme-light-modern",
feature = "theme-black-white",
feature = "theme-dark-blue",
feature = "theme-green-black"
)))]
pub const ACTIVE_THEME: Theme = Theme::DarkBlue;
