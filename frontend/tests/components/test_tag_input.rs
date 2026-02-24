use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
use leptos_router::components::Router;
use oxibooru_frontend::api::ApiClient;
use oxibooru_frontend::auth::AuthState;
use oxibooru_frontend::components::tag_input::TagInput;

/// Helper: mount TagInput with pre-populated tags.
fn mount_tag_input(initial_tags: Vec<String>) -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    let container = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&container).unwrap();

    LocalStorage::delete("oxibooru-auth-username");
    LocalStorage::delete("oxibooru-auth-password");

    let html_el: web_sys::HtmlElement = wasm_bindgen::JsCast::unchecked_into(container.clone());
    leptos::mount::mount_to(html_el, move || {
        let api = RwSignal::new(ApiClient::new("/api"));
        provide_context(api);
        let auth = AuthState::new(api);
        provide_context(auth);
        let tags = RwSignal::new(initial_tags);
        view! {
            <Router>
                <TagInput tags=tags />
            </Router>
        }
    })
    .forget();

    container
}

#[wasm_bindgen_test]
fn tag_input_renders_container() {
    let container = mount_tag_input(vec![]);
    let html = container.inner_html();
    assert!(html.contains("tag-input"), "should have tag-input class, got: {html}");
    container.remove();
}

#[wasm_bindgen_test]
fn tag_input_renders_existing_tags_as_chips() {
    let container = mount_tag_input(vec!["landscape".into(), "nature".into()]);
    let html = container.inner_html();
    assert!(html.contains("landscape"), "should show 'landscape' chip, got: {html}");
    assert!(html.contains("nature"), "should show 'nature' chip, got: {html}");
    assert!(html.contains("tag-chip"), "should have tag-chip class, got: {html}");
    container.remove();
}

#[wasm_bindgen_test]
fn tag_input_renders_autocomplete_input() {
    let container = mount_tag_input(vec![]);
    let html = container.inner_html();
    assert!(html.contains("tag-autocomplete"), "should have autocomplete input, got: {html}");
    container.remove();
}

#[wasm_bindgen_test]
fn tag_input_chips_have_remove_button() {
    let container = mount_tag_input(vec!["test-tag".into()]);
    let html = container.inner_html();
    // Each chip should have a remove (x) button
    assert!(html.contains("tag-chip"), "should have tag chip, got: {html}");
    // The chip should contain a button for removal
    let chip_buttons = html.matches("tag-chip").count();
    assert!(chip_buttons >= 1, "should have at least one tag chip");
    container.remove();
}
