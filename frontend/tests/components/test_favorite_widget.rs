use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use leptos::prelude::*;
use oxibooru_frontend::components::favorite_widget::FavoriteWidget;

/// Helper: mount FavoriteWidget with given state.
fn mount_favorite(favorited: bool, count: i64) -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    let container = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&container).unwrap();

    let html_el: web_sys::HtmlElement = wasm_bindgen::JsCast::unchecked_into(container.clone());
    leptos::mount::mount_to(html_el, move || {
        let fav_sig = RwSignal::new(favorited);
        let count_sig = RwSignal::new(count);
        let on_toggle = Callback::new(move |_: bool| {});
        view! {
            <FavoriteWidget favorited=fav_sig count=count_sig on_toggle=on_toggle />
        }
    })
    .forget();

    container
}

#[wasm_bindgen_test]
fn favorite_widget_renders_count() {
    let container = mount_favorite(false, 7);
    let html = container.inner_html();
    assert!(html.contains("favorite-widget"), "should have favorite-widget class, got: {html}");
    assert!(html.contains("7"), "should display count 7, got: {html}");
    container.remove();
}

#[wasm_bindgen_test]
fn favorite_widget_renders_heart_button() {
    let container = mount_favorite(false, 0);
    let html = container.inner_html();
    assert!(html.contains("<button"), "should have a heart button, got: {html}");
    container.remove();
}

#[wasm_bindgen_test]
fn favorite_widget_active_when_favorited() {
    let container = mount_favorite(true, 3);
    let html = container.inner_html();
    assert!(html.contains("active"), "should have active class when favorited, got: {html}");
    container.remove();
}

#[wasm_bindgen_test]
fn favorite_widget_not_active_when_not_favorited() {
    let container = mount_favorite(false, 0);
    let html = container.inner_html();
    // The button should NOT have the active class
    assert!(!html.contains("active"), "should not have active class when not favorited, got: {html}");
    container.remove();
}
