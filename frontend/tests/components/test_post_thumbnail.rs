use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use leptos::prelude::*;
use leptos_router::components::Router;
use oxibooru_frontend::components::post_thumbnail::PostThumbnail;
use oxibooru_shared::enums::{PostSafety, PostType};

/// Helper: mount PostThumbnail into a test container.
fn mount_thumbnail(
    id: i64,
    url: &str,
    safety: PostSafety,
    post_type: PostType,
    score: Option<i64>,
    fav_count: Option<i64>,
    comment_count: Option<i64>,
) -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    let container = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&container).unwrap();

    let url = url.to_string();
    let html_el: web_sys::HtmlElement = wasm_bindgen::JsCast::unchecked_into(container.clone());
    leptos::mount::mount_to(html_el, move || {
        view! {
            <Router>
                <PostThumbnail
                    id=id
                    thumbnail_url=url
                    safety=safety
                    post_type=post_type
                    score=score
                    favorite_count=fav_count
                    comment_count=comment_count
                />
            </Router>
        }
    })
    .forget();

    container
}

#[wasm_bindgen_test]
fn thumbnail_renders_image() {
    let container = mount_thumbnail(42, "/thumb/42.jpg", PostSafety::Safe, PostType::Image, None, None, None);
    let html = container.inner_html();
    assert!(html.contains("post-thumbnail"), "should render post-thumbnail class");
    assert!(html.contains("/thumb/42.jpg"), "should contain thumbnail URL");
    assert!(html.contains("/post/42"), "should link to post detail page");
    container.remove();
}

#[wasm_bindgen_test]
fn thumbnail_shows_safety_badge() {
    let container = mount_thumbnail(1, "/t.jpg", PostSafety::Sketchy, PostType::Image, None, None, None);
    let html = container.inner_html();
    assert!(html.contains("safety-sketchy"), "should have sketchy safety class");
    container.remove();
}

#[wasm_bindgen_test]
fn thumbnail_shows_video_badge() {
    let container = mount_thumbnail(1, "/t.jpg", PostSafety::Safe, PostType::Video, None, None, None);
    let html = container.inner_html();
    assert!(html.contains("type-badge"), "should show type badge for video");
    container.remove();
}

#[wasm_bindgen_test]
fn thumbnail_no_type_badge_for_image() {
    let container = mount_thumbnail(1, "/t.jpg", PostSafety::Safe, PostType::Image, None, None, None);
    let html = container.inner_html();
    assert!(!html.contains("type-badge"), "should not show type badge for image");
    container.remove();
}

#[wasm_bindgen_test]
fn thumbnail_shows_stats_when_provided() {
    let container = mount_thumbnail(1, "/t.jpg", PostSafety::Safe, PostType::Image, Some(10), Some(3), Some(2));
    let html = container.inner_html();
    assert!(html.contains("10"), "should show score");
    assert!(html.contains("3"), "should show favorite count");
    assert!(html.contains("2"), "should show comment count");
    container.remove();
}
