use leptos::prelude::*;

/// Render markdown text as HTML.
#[component]
pub fn Markdown(#[prop(into)] text: String) -> impl IntoView {
    let html = render_markdown(&text);
    view! {
        <div class="markdown-content" inner_html=html />
    }
}

/// Convert markdown to HTML using pulldown-cmark.
fn render_markdown(input: &str) -> String {
    use pulldown_cmark::{html, Options, Parser};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_bold() {
        let html = render_markdown("**bold**");
        assert!(html.contains("<strong>bold</strong>"), "got: {html}");
    }

    #[test]
    fn render_italic() {
        let html = render_markdown("*italic*");
        assert!(html.contains("<em>italic</em>"), "got: {html}");
    }

    #[test]
    fn render_strikethrough() {
        let html = render_markdown("~~deleted~~");
        assert!(html.contains("<del>deleted</del>"), "got: {html}");
    }

    #[test]
    fn render_link() {
        let html = render_markdown("[text](https://example.com)");
        assert!(html.contains("<a href=\"https://example.com\">text</a>"), "got: {html}");
    }

    #[test]
    fn render_empty() {
        let html = render_markdown("");
        assert_eq!(html, "");
    }

    #[test]
    fn render_plain_text() {
        let html = render_markdown("just text");
        assert!(html.contains("just text"), "got: {html}");
    }

    #[test]
    fn render_table() {
        let input = "| A | B |\n|---|---|\n| 1 | 2 |";
        let html = render_markdown(input);
        assert!(html.contains("<table>"), "got: {html}");
        assert!(html.contains("<td>1</td>"), "got: {html}");
    }
}
