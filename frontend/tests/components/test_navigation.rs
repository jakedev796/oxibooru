use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use leptos::prelude::*;
use leptos_router::components::Router;
use oxibooru_frontend::api::ApiClient;
use oxibooru_frontend::auth::AuthState;
use oxibooru_frontend::components::navigation::Navigation;

/// Create an AuthState for testing (no credentials, no privileges loaded).
fn test_auth_state() -> AuthState {
    let api = RwSignal::new(ApiClient::new("http://localhost:6666"));
    AuthState::new(api)
}

/// Helper: mount Navigation with required contexts into a test container.
/// Returns the container element for assertions.
fn mount_navigation(auth: AuthState) -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    let container = document.create_element("div").unwrap();
    container.set_id("test-nav-container");
    document.body().unwrap().append_child(&container).unwrap();

    let html_el: web_sys::HtmlElement = container.clone().unchecked_into();
    leptos::mount::mount_to(html_el, move || {
        provide_context(auth);
        view! {
            <Router>
                <Navigation/>
            </Router>
        }
    })
    .forget();

    container
}

#[wasm_bindgen_test]
fn navigation_renders_nav_element() {
    let auth = test_auth_state();
    let container = mount_navigation(auth);

    let html = container.inner_html();
    assert!(html.contains("top-navigation"), "should render nav with 'top-navigation' class");

    container.remove();
}

#[wasm_bindgen_test]
fn navigation_renders_home_and_help_links() {
    let auth = test_auth_state();
    let container = mount_navigation(auth);

    let html = container.inner_html();
    assert!(html.contains("Home"), "should render Home link");
    assert!(html.contains("Help"), "should render Help link");

    container.remove();
}

#[wasm_bindgen_test]
fn navigation_shows_login_for_anonymous_user() {
    let auth = test_auth_state();
    let container = mount_navigation(auth);

    let html = container.inner_html();
    assert!(html.contains("Log in"), "anonymous user should see 'Log in'");
    assert!(html.contains("Register"), "anonymous user should see 'Register'");

    container.remove();
}
