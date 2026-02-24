use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use leptos::prelude::*;
use oxibooru_shared::enums::Rating;
use oxibooru_frontend::components::score_widget::ScoreWidget;

/// Helper: mount ScoreWidget with given state.
fn mount_score(score: i64, own_score: Rating) -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    let container = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&container).unwrap();

    let html_el: web_sys::HtmlElement = wasm_bindgen::JsCast::unchecked_into(container.clone());
    leptos::mount::mount_to(html_el, move || {
        let score_sig = RwSignal::new(score);
        let own_score_sig = RwSignal::new(own_score);
        let on_vote = Callback::new(move |_: Rating| {});
        view! {
            <ScoreWidget score=score_sig own_score=own_score_sig on_vote=on_vote />
        }
    })
    .forget();

    container
}

#[wasm_bindgen_test]
fn score_widget_renders_score_value() {
    let container = mount_score(5, Rating::None);
    let html = container.inner_html();
    assert!(html.contains("score-widget"), "should have score-widget class, got: {html}");
    assert!(html.contains("5"), "should display score 5, got: {html}");
    container.remove();
}

#[wasm_bindgen_test]
fn score_widget_renders_up_and_down_buttons() {
    let container = mount_score(0, Rating::None);
    let html = container.inner_html();
    // Should have two buttons (up and down)
    let button_count = html.matches("<button").count();
    assert!(button_count >= 2, "should have at least 2 buttons (up/down), got {button_count}");
    container.remove();
}

#[wasm_bindgen_test]
fn score_widget_highlights_active_up_vote() {
    let container = mount_score(1, Rating::Like);
    let html = container.inner_html();
    assert!(html.contains("active-up"), "should have active-up class when liked, got: {html}");
    container.remove();
}

#[wasm_bindgen_test]
fn score_widget_highlights_active_down_vote() {
    let container = mount_score(-1, Rating::Dislike);
    let html = container.inner_html();
    assert!(html.contains("active-down"), "should have active-down class when disliked, got: {html}");
    container.remove();
}
