use crate::style::generate_css;
pub use crate::my_theme::Theme;

pub fn page_template(title: &str, content: &str, theme: Theme) -> String {
    format!(
        r#"<!doctype html>
<html lang="de">
<head>
<meta charset="utf-8">
<title>{title}</title>
<meta name="viewport" content="width=device-width,initial-scale=1">
<style>
{css}
</style>
</head>
<body>
<div class="container">
  <div class="topbar">
    <a href="/">BACK</a>
  </div>
  <main class="panel">
    {content}
  </main>
  <footer>
    <div>MD-Files</div>
  </footer>
</div>
</body>
</html>"#,
        title = html_escape::encode_text(title),
        css = generate_css(theme),
        content = content
    )
}

pub fn get_subcontent_tags(inner_html: &str) -> String {
    format!(
        r#"<article class="markdown-content">{inner}</article>
<hr>
<div class="back-wrap">
  <a class="btn" href="/">BACK</a>
</div>"#,
        inner = inner_html
    )
}


