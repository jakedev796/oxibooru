use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use leptos::prelude::*;
use leptos_router::components::Router;
use oxibooru_frontend::components::pagination::Pagination;

/// Helper: mount Pagination into a test container.
fn mount_pagination(offset: i64, limit: i64, total: i64) -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    let container = document.create_element("div").unwrap();
    document.body().unwrap().append_child(&container).unwrap();

    let html_el: web_sys::HtmlElement = wasm_bindgen::JsCast::unchecked_into(container.clone());
    leptos::mount::mount_to(html_el, move || {
        let href_for_page = Callback::new(move |(off, lim): (i64, i64)| format!("/posts?offset={off}&limit={lim}"));
        view! {
            <Router>
                <Pagination offset=offset limit=limit total=total href_for_page=href_for_page />
            </Router>
        }
    })
    .forget();

    container
}

#[wasm_bindgen_test]
fn pagination_empty_for_zero_total() {
    let container = mount_pagination(0, 10, 0);
    let html = container.inner_html();
    // Should render nothing (empty view)
    assert!(!html.contains("pagination"), "should not render pagination for zero total");
    container.remove();
}

#[wasm_bindgen_test]
fn pagination_single_page_shows_total() {
    let container = mount_pagination(0, 10, 5);
    let html = container.inner_html();
    assert!(html.contains("5 results"), "should show '5 results' for single page, got: {html}");
    // Should not have prev/next
    assert!(!html.contains("Prev"), "should not have Prev on single page");
    assert!(!html.contains("Next"), "should not have Next on single page");
    container.remove();
}

#[wasm_bindgen_test]
fn pagination_multiple_pages_shows_links() {
    let container = mount_pagination(0, 10, 50);
    let html = container.inner_html();
    assert!(html.contains("pagination"), "should render pagination nav");
    assert!(html.contains("50 results"), "should show total, got: {html}");
    // On first page, no Prev but has Next
    assert!(!html.contains("Prev"), "should not have Prev on first page");
    assert!(html.contains("Next"), "should have Next link");
    container.remove();
}

#[wasm_bindgen_test]
fn pagination_middle_page_has_prev_and_next() {
    // Page 3 of 5 (offset=20, limit=10, total=50)
    let container = mount_pagination(20, 10, 50);
    let html = container.inner_html();
    assert!(html.contains("Prev"), "should have Prev link on middle page");
    assert!(html.contains("Next"), "should have Next link on middle page");
    container.remove();
}

#[wasm_bindgen_test]
fn pagination_last_page_has_prev_no_next() {
    // Page 5 of 5 (offset=40, limit=10, total=50)
    let container = mount_pagination(40, 10, 50);
    let html = container.inner_html();
    assert!(html.contains("Prev"), "should have Prev on last page");
    assert!(!html.contains("Next"), "should not have Next on last page");
    container.remove();
}

#[wasm_bindgen_test]
fn pagination_links_contain_correct_urls() {
    let container = mount_pagination(0, 10, 50);
    let html = container.inner_html();
    // Next should go to offset=10
    assert!(html.contains("offset=10"), "Next should link to offset=10, got: {html}");
    container.remove();
}
