use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
use leptos_router::components::Router;
use oxibooru_frontend::api::ApiClient;
use oxibooru_frontend::auth::AuthState;
use oxibooru_frontend::pages::login::LoginPage;

/// Helper: mount LoginPage with required context.
fn mount_login() -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    let container = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&container).unwrap();

    // Clear stored credentials
    LocalStorage::delete("oxibooru-auth-username");
    LocalStorage::delete("oxibooru-auth-password");

    let html_el: web_sys::HtmlElement = wasm_bindgen::JsCast::unchecked_into(container.clone());
    leptos::mount::mount_to(html_el, move || {
        let api = RwSignal::new(ApiClient::new("/api"));
        provide_context(api);
        let auth = AuthState::new(api);
        provide_context(auth);
        view! {
            <Router>
                <LoginPage />
            </Router>
        }
    })
    .forget();

    container
}

#[wasm_bindgen_test]
fn login_form_renders_username_input() {
    let container = mount_login();
    let html = container.inner_html();
    assert!(html.contains("user-name"), "should have username input, got: {html}");
    assert!(html.contains("User name"), "should have username label, got: {html}");
    container.remove();
}

#[wasm_bindgen_test]
fn login_form_renders_password_input() {
    let container = mount_login();
    let html = container.inner_html();
    assert!(html.contains("user-password"), "should have password input, got: {html}");
    assert!(html.contains("Password"), "should have password label, got: {html}");
    container.remove();
}

#[wasm_bindgen_test]
fn login_form_renders_submit_button() {
    let container = mount_login();
    let html = container.inner_html();
    assert!(html.contains("Log in"), "should have submit button, got: {html}");
    container.remove();
}

#[wasm_bindgen_test]
fn login_form_renders_forgot_password_link() {
    let container = mount_login();
    let html = container.inner_html();
    assert!(html.contains("Forgot password"), "should have forgot password link, got: {html}");
    assert!(html.contains("password-reset"), "should link to password-reset, got: {html}");
    container.remove();
}
