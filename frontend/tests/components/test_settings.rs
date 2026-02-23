use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use leptos::prelude::*;
use leptos_router::components::Router;
use oxibooru_frontend::pages::settings::SettingsPage;
use oxibooru_frontend::settings::SettingsState;

/// Helper: mount SettingsPage with required context.
fn mount_settings() -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    let container = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&container).unwrap();

    let html_el: web_sys::HtmlElement = wasm_bindgen::JsCast::unchecked_into(container.clone());
    leptos::mount::mount_to(html_el, move || {
        let settings = SettingsState::new();
        provide_context(settings);
        view! {
            <Router>
                <SettingsPage />
            </Router>
        }
    })
    .forget();

    container
}

#[wasm_bindgen_test]
fn settings_renders_dark_theme_toggle() {
    let container = mount_settings();
    let html = container.inner_html();
    assert!(html.contains("dark theme"), "should have dark theme toggle, got: {html}");
    container.remove();
}

#[wasm_bindgen_test]
fn settings_renders_posts_per_page_input() {
    let container = mount_settings();
    let html = container.inner_html();
    assert!(html.contains("Posts per page"), "should have posts per page input, got: {html}");
    assert!(html.contains("type=\"number\""), "should have number input, got: {html}");
    container.remove();
}

#[wasm_bindgen_test]
fn settings_renders_safety_checkboxes() {
    let container = mount_settings();
    let html = container.inner_html();
    assert!(html.contains("Safe"), "should have safe checkbox, got: {html}");
    assert!(html.contains("Sketchy"), "should have sketchy checkbox, got: {html}");
    assert!(html.contains("Unsafe"), "should have unsafe checkbox, got: {html}");
    container.remove();
}

#[wasm_bindgen_test]
fn settings_renders_fit_mode_select() {
    let container = mount_settings();
    let html = container.inner_html();
    assert!(html.contains("Post fit mode"), "should have fit mode select, got: {html}");
    assert!(html.contains("fit-both"), "should have fit-both option, got: {html}");
    assert!(html.contains("fit-width"), "should have fit-width option, got: {html}");
    container.remove();
}

#[wasm_bindgen_test]
fn settings_renders_save_button() {
    let container = mount_settings();
    let html = container.inner_html();
    assert!(html.contains("Save settings"), "should have save button, got: {html}");
    container.remove();
}
