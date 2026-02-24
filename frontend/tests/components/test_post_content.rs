use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use leptos::prelude::*;
use oxibooru_frontend::components::post_content::PostContent;
use oxibooru_shared::enums::PostType;

/// Helper: mount PostContent into a test container.
fn mount_content(url: &str, post_type: PostType, fit_mode: &str, flags: Vec<String>) -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    let container = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&container).unwrap();

    let url = url.to_string();
    let fit_mode = fit_mode.to_string();
    let html_el: web_sys::HtmlElement = wasm_bindgen::JsCast::unchecked_into(container.clone());
    leptos::mount::mount_to(html_el, move || {
        view! {
            <PostContent content_url=url post_type=post_type fit_mode=fit_mode flags=flags />
        }
    })
    .forget();

    container
}

#[wasm_bindgen_test]
fn content_image_renders_img_tag() {
    let container = mount_content("/data/42/content.jpg", PostType::Image, "fit-both", vec![]);
    let html = container.inner_html();
    assert!(html.contains("<img"), "should render img tag for image type");
    assert!(html.contains("/data/42/content.jpg"), "should contain content URL");
    assert!(html.contains("fit-both"), "should have fit mode class");
    container.remove();
}

#[wasm_bindgen_test]
fn content_video_renders_video_tag() {
    let container = mount_content("/data/42/content.mp4", PostType::Video, "fit-width", vec![]);
    let html = container.inner_html();
    assert!(html.contains("<video"), "should render video tag for video type");
    assert!(html.contains("/data/42/content.mp4"), "should contain content URL");
    assert!(html.contains("fit-width"), "should have fit mode class");
    container.remove();
}

#[wasm_bindgen_test]
fn content_flash_shows_notice() {
    let container = mount_content("/data/42/content.swf", PostType::Flash, "fit-both", vec![]);
    let html = container.inner_html();
    assert!(html.contains("Flash content is not supported"), "should show flash notice");
    container.remove();
}

#[wasm_bindgen_test]
fn content_animation_renders_img_tag() {
    let container = mount_content("/data/42/content.gif", PostType::Animation, "fit-both", vec![]);
    let html = container.inner_html();
    assert!(html.contains("<img"), "should render img tag for animation type");
    container.remove();
}
