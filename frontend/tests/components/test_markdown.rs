use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use leptos::prelude::*;
use oxibooru_frontend::components::markdown::Markdown;

/// Helper: mount Markdown component into a test container.
fn mount_markdown(text: &str) -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    let container = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&container).unwrap();

    let text = text.to_string();
    let html_el: web_sys::HtmlElement = wasm_bindgen::JsCast::unchecked_into(container.clone());
    leptos::mount::mount_to(html_el, move || {
        view! {
            <Markdown text=text />
        }
    })
    .forget();

    container
}

#[wasm_bindgen_test]
fn markdown_renders_bold() {
    let container = mount_markdown("**hello**");
    let html = container.inner_html();
    assert!(html.contains("<strong>hello</strong>"), "should render bold text, got: {html}");
    container.remove();
}

#[wasm_bindgen_test]
fn markdown_renders_into_content_div() {
    let container = mount_markdown("test");
    let html = container.inner_html();
    assert!(html.contains("markdown-content"), "should have markdown-content class, got: {html}");
    container.remove();
}

#[wasm_bindgen_test]
fn markdown_renders_link() {
    let container = mount_markdown("[click](https://example.com)");
    let html = container.inner_html();
    assert!(html.contains("href=\"https://example.com\""), "should render link, got: {html}");
    assert!(html.contains("click"), "should have link text, got: {html}");
    container.remove();
}

#[wasm_bindgen_test]
fn markdown_renders_empty() {
    let container = mount_markdown("");
    let html = container.inner_html();
    assert!(html.contains("markdown-content"), "should still render container, got: {html}");
    container.remove();
}
