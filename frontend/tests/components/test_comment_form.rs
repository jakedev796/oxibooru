use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use gloo_storage::{LocalStorage, Storage};
use leptos::prelude::*;
use leptos_router::components::Router;
use oxibooru_frontend::api::ApiClient;
use oxibooru_frontend::auth::AuthState;
use oxibooru_frontend::components::comment_form::CommentForm;
use oxibooru_shared::comment::CommentInfo;

/// Helper: mount CommentForm in create mode.
fn mount_comment_form(post_id: i64) -> web_sys::Element {
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
        let on_submit = Callback::new(move |_: CommentInfo| {});
        view! {
            <Router>
                <CommentForm post_id=post_id on_submit=on_submit />
            </Router>
        }
    })
    .forget();

    container
}

#[wasm_bindgen_test]
fn comment_form_renders_textarea() {
    let container = mount_comment_form(1);
    let html = container.inner_html();
    assert!(html.contains("<textarea"), "should have a textarea, got: {html}");
    container.remove();
}

#[wasm_bindgen_test]
fn comment_form_renders_submit_button() {
    let container = mount_comment_form(1);
    let html = container.inner_html();
    assert!(html.contains("Submit") || html.contains("submit"), "should have a submit button, got: {html}");
    container.remove();
}

#[wasm_bindgen_test]
fn comment_form_has_form_class() {
    let container = mount_comment_form(1);
    let html = container.inner_html();
    assert!(html.contains("comment-form"), "should have comment-form class, got: {html}");
    container.remove();
}
