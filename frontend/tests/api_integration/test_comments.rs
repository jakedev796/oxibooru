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

/// POST /comments without auth should fail.
#[tokio::test]
async fn create_comment_unauthenticated_fails() {
    let client = reqwest::Client::new();
    let url = format!("{BACKEND_URL}/comments");
    let resp = client
        .post(&url)
        .json(&serde_json::json!({
            "postId": 1,
            "text": "test comment"
        }))
        .send()
        .await
        .expect("backend not reachable");
    let status = resp.status().as_u16();
    assert!(
        status == 401 || status == 403,
        "unauthenticated POST /comments should fail, got {status}"
    );
}

/// PUT /comment/{id}/score without auth should fail.
#[tokio::test]
async fn score_comment_unauthenticated_fails() {
    let list_url = format!("{BACKEND_URL}/comments?query=&offset=0&limit=1");
    let list_resp = reqwest::get(&list_url).await.expect("backend not reachable");
    let page: PagedResponse<CommentInfo> = list_resp.json().await.unwrap();

    if page.results.is_empty() {
        eprintln!("SKIP: no comments in database");
        return;
    }

    let comment_id = page.results[0].id.unwrap();
    let client = reqwest::Client::new();
    let url = format!("{BACKEND_URL}/comment/{comment_id}/score");
    let resp = client
        .put(&url)
        .json(&serde_json::json!({ "score": 1 }))
        .send()
        .await
        .expect("backend not reachable");
    let status = resp.status().as_u16();
    assert!(
        status == 401 || status == 403,
        "unauthenticated PUT /comment/{comment_id}/score should fail, got {status}"
    );
}

/// DELETE /comment/{id} without auth should fail.
#[tokio::test]
async fn delete_comment_unauthenticated_fails() {
    let list_url = format!("{BACKEND_URL}/comments?query=&offset=0&limit=1");
    let list_resp = reqwest::get(&list_url).await.expect("backend not reachable");
    let page: PagedResponse<CommentInfo> = list_resp.json().await.unwrap();

    if page.results.is_empty() {
        eprintln!("SKIP: no comments in database");
        return;
    }

    let comment_id = page.results[0].id.unwrap();
    let client = reqwest::Client::new();
    let url = format!("{BACKEND_URL}/comment/{comment_id}");
    let resp = client
        .delete(&url)
        .json(&serde_json::json!({ "version": "fake" }))
        .send()
        .await
        .expect("backend not reachable");
    let status = resp.status().as_u16();
    assert!(
        status == 401 || status == 403,
        "unauthenticated DELETE /comment/{comment_id} should fail, got {status}"
    );
}
