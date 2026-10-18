#[derive(Clone)]
pub struct HighlightConfig {
    pub color_keyword: &'static str,
    pub color_string: &'static str,
    pub color_comment: &'static str,
    pub color_header: &'static str,
    pub color_tag: &'static str,
    pub color_numbers: &'static str,
    pub color_reset: &'static str,
    pub new_line: &'static str
}

impl HighlightConfig {
    pub fn html_colors() -> Self {
        Self {
            color_keyword: "<span style='color: #0000DD; font-weight: bold;'>",
            color_string:  "<span style='color: #855500;'>",
            color_comment: "<span style='color: #AAAA00; font-style: italic;'>",
            color_header:  "<span style='color: #75AA3B;'>",
            color_tag:     "<span style='color: #A70022;'>",
            color_numbers: "<span style='color: #000088;'>",
            color_reset:   "</span>",
            new_line:      "<br>",
        }
    }
}

