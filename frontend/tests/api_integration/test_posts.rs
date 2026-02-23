use oxibooru_shared::pagination::PagedResponse;
use oxibooru_shared::post::{PostInfo, PostNeighbors};

const BACKEND_URL: &str = "http://localhost:6666";

/// GET /posts should return a paged response.
#[tokio::test]
async fn get_posts_returns_paged_response() {
    let url = format!("{BACKEND_URL}/posts?query=&offset=0&limit=2&fields=id,thumbnailUrl,type,safety");
    let resp = reqwest::get(&url)
        .await
        .expect("backend not reachable — is it running on port 6666?");
    assert!(resp.status().is_success(), "GET /posts returned {}", resp.status());

    let page: PagedResponse<PostInfo> = resp.json().await.expect("failed to deserialize PagedResponse<PostInfo>");
    assert!(page.total >= 0, "total should be non-negative");
    assert!(page.limit == 2, "limit should match request");
    assert!(page.offset == 0, "offset should match request");
    assert!(page.results.len() <= 2, "should return at most `limit` results");
}

/// GET /posts with query should filter results.
#[tokio::test]
async fn get_posts_with_query() {
    let url = format!("{BACKEND_URL}/posts?query=sort:id&offset=0&limit=1&fields=id,safety");
    let resp = reqwest::get(&url).await.expect("backend not reachable");
    assert!(resp.status().is_success());

    let page: PagedResponse<PostInfo> = resp.json().await.unwrap();
    // With sort:id, if there are any posts, the first result should have an id
    if !page.results.is_empty() {
        assert!(page.results[0].id.is_some(), "post should have an id field");
    }
}

/// GET /post/{id} should return a single post.
#[tokio::test]
async fn get_single_post() {
    // First get any post id
    let list_url = format!("{BACKEND_URL}/posts?query=&offset=0&limit=1&fields=id");
    let list_resp = reqwest::get(&list_url).await.expect("backend not reachable");
    let page: PagedResponse<PostInfo> = list_resp.json().await.unwrap();

    if page.results.is_empty() {
        eprintln!("SKIP: no posts in database");
        return;
    }

    let post_id = page.results[0].id.unwrap();
    let url = format!("{BACKEND_URL}/post/{post_id}");
    let resp = reqwest::get(&url).await.expect("backend not reachable");
    assert!(resp.status().is_success(), "GET /post/{post_id} returned {}", resp.status());

    let post: PostInfo = resp.json().await.expect("failed to deserialize PostInfo");
    assert_eq!(post.id, Some(post_id));
}

/// GET /post/{id}/around should return prev/next neighbors.
#[tokio::test]
async fn get_post_around() {
    // First get any post id
    let list_url = format!("{BACKEND_URL}/posts?query=&offset=0&limit=1&fields=id");
    let list_resp = reqwest::get(&list_url).await.expect("backend not reachable");
    let page: PagedResponse<PostInfo> = list_resp.json().await.unwrap();

    if page.results.is_empty() {
        eprintln!("SKIP: no posts in database");
        return;
    }

    let post_id = page.results[0].id.unwrap();
    let url = format!("{BACKEND_URL}/post/{post_id}/around?query=&fields=id");
    let resp = reqwest::get(&url).await.expect("backend not reachable");
    assert!(resp.status().is_success(), "GET /post/{post_id}/around returned {}", resp.status());

    let neighbors: PostNeighbors = resp.json().await.expect("failed to deserialize PostNeighbors");
    // prev and next may be null, but the response should be parseable
    if let Some(prev) = &neighbors.prev {
        assert!(prev.id.is_some(), "prev post should have an id");
    }
    if let Some(next) = &neighbors.next {
        assert!(next.id.is_some(), "next post should have an id");
    }
}

/// GET /featured-post should return either a post or 404.
#[tokio::test]
async fn get_featured_post() {
    let url = format!("{BACKEND_URL}/featured-post");
    let resp = reqwest::get(&url).await.expect("backend not reachable");

    // Featured post may not exist (404) — both are valid
    let status = resp.status().as_u16();
    assert!(
        status == 200 || status == 404,
        "GET /featured-post should return 200 or 404, got {status}"
    );

    if status == 200 {
        let post: PostInfo = resp.json().await.expect("failed to deserialize featured PostInfo");
        assert!(post.id.is_some(), "featured post should have an id");
    }
}
