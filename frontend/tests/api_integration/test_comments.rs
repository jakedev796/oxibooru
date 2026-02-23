use oxibooru_shared::comment::CommentInfo;
use oxibooru_shared::pagination::PagedResponse;

const BACKEND_URL: &str = "http://localhost:6666";

/// GET /comments should return a paged response.
#[tokio::test]
async fn get_comments_returns_paged_response() {
    let url = format!("{BACKEND_URL}/comments?query=&offset=0&limit=5");
    let resp = reqwest::get(&url)
        .await
        .expect("backend not reachable — is it running on port 6666?");
    assert!(resp.status().is_success(), "GET /comments returned {}", resp.status());

    let page: PagedResponse<CommentInfo> =
        resp.json().await.expect("failed to deserialize PagedResponse<CommentInfo>");
    assert!(page.total >= 0);
    assert!(page.limit == 5);
    assert!(page.results.len() <= 5);
}

/// Comments should have text and an id when present.
#[tokio::test]
async fn get_comments_have_expected_fields() {
    let url = format!("{BACKEND_URL}/comments?query=&offset=0&limit=3");
    let resp = reqwest::get(&url).await.expect("backend not reachable");
    let page: PagedResponse<CommentInfo> = resp.json().await.unwrap();

    for comment in &page.results {
        assert!(comment.id.is_some(), "comment should have an id");
        assert!(comment.text.is_some(), "comment should have text");
    }
}
