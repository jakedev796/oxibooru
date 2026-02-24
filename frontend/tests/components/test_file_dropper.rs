use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use leptos::prelude::*;
use oxibooru_frontend::components::file_dropper::FileDropper;

/// Helper: mount FileDropper.
fn mount_file_dropper(label: &'static str) -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    let container = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&container).unwrap();

    let html_el: web_sys::HtmlElement = wasm_bindgen::JsCast::unchecked_into(container.clone());
    leptos::mount::mount_to(html_el, move || {
        let on_file = Callback::new(move |_: web_sys::File| {});
        view! {
            <FileDropper on_file=on_file label=label />
        }
    })
    .forget();

    container
}

#[wasm_bindgen_test]
fn file_dropper_renders_drop_zone() {
    let container = mount_file_dropper("Content");
    let html = container.inner_html();
    assert!(html.contains("drop-zone"), "should have drop-zone class, got: {html}");
    container.remove();
}

#[wasm_bindgen_test]
fn file_dropper_renders_label() {
    let container = mount_file_dropper("Upload file");
    let html = container.inner_html();
    assert!(html.contains("Upload file"), "should show label, got: {html}");
    container.remove();
}

#[wasm_bindgen_test]
fn file_dropper_renders_instructions() {
    let container = mount_file_dropper("Content");
    let html = container.inner_html();
    assert!(
        html.contains("drop-instructions") || html.contains("Drop") || html.contains("click"),
        "should have drop instructions, got: {html}"
    );
    container.remove();
}

#[wasm_bindgen_test]
fn file_dropper_has_hidden_file_input() {
    let container = mount_file_dropper("Content");
    let html = container.inner_html();
    assert!(html.contains("type=\"file\""), "should have hidden file input, got: {html}");
    container.remove();
}
