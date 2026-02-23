use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
use leptos_router::components::Router;
use oxibooru_frontend::api::ApiClient;
use oxibooru_frontend::auth::AuthState;
use oxibooru_frontend::pages::register::RegisterPage;
use oxibooru_shared::info::PublicConfig;

/// Helper: mount RegisterPage with required context.
fn mount_register() -> web_sys::Element {
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
        let server_config: RwSignal<Option<PublicConfig>> = RwSignal::new(None);
        provide_context(server_config);
        view! {
            <Router>
                <RegisterPage />
            </Router>
        }
    })
    .forget();

    container
}

#[wasm_bindgen_test]
fn register_form_renders_username_input() {
    let container = mount_register();
    let html = container.inner_html();
    assert!(html.contains("user-name"), "should have username input, got: {html}");
    container.remove();
}

#[wasm_bindgen_test]
fn register_form_renders_password_input() {
    let container = mount_register();
    let html = container.inner_html();
    assert!(html.contains("user-password"), "should have password input, got: {html}");
    container.remove();
}

#[wasm_bindgen_test]
fn register_form_renders_email_input() {
    let container = mount_register();
    let html = container.inner_html();
    assert!(html.contains("user-email"), "should have email input, got: {html}");
    assert!(html.contains("optional"), "should indicate email is optional, got: {html}");
    container.remove();
}

#[wasm_bindgen_test]
fn register_form_renders_create_button() {
    let container = mount_register();
    let html = container.inner_html();
    assert!(html.contains("Create account"), "should have create account button, got: {html}");
    container.remove();
}
