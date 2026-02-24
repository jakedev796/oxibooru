use oxibooru_shared::pagination::{PagedResponse, UnpagedResponse};
use oxibooru_shared::tag::{TagInfo, TagSibling};

const BACKEND_URL: &str = "http://localhost:6666";

/// GET /tags should return a paged response.
#[tokio::test]
async fn get_tags_returns_paged_response() {
    let url = format!("{BACKEND_URL}/tags?query=&offset=0&limit=5&fields=names,category,usages");
    let resp = reqwest::get(&url)
        .await
        .expect("backend not reachable — is it running on port 6666?");
    assert!(resp.status().is_success(), "GET /tags returned {}", resp.status());

    let page: PagedResponse<TagInfo> = resp.json().await.expect("failed to deserialize PagedResponse<TagInfo>");
    assert!(page.total >= 0);
    assert!(page.limit == 5);
    assert!(page.results.len() <= 5);
}

/// GET /tags with sort should work.
#[tokio::test]
async fn get_tags_sorted_by_usages() {
    let url = format!("{BACKEND_URL}/tags?query=sort:usages&offset=0&limit=3&fields=names,usages");
    let resp = reqwest::get(&url).await.expect("backend not reachable");
    assert!(resp.status().is_success());

    let page: PagedResponse<TagInfo> = resp.json().await.unwrap();
    // If there are results, they should have names
    for tag in &page.results {
        assert!(tag.names.is_some(), "tag should have names field");
    }
}

/// GET /tag/{name} should return a single tag.
#[tokio::test]
async fn get_single_tag() {
    // First get any tag name
    let list_url = format!("{BACKEND_URL}/tags?query=&offset=0&limit=1&fields=names");
    let list_resp = reqwest::get(&list_url).await.expect("backend not reachable");
    let page: PagedResponse<TagInfo> = list_resp.json().await.unwrap();

    if page.results.is_empty() {
        eprintln!("SKIP: no tags in database");
        return;
    }

    let tag_name = &page.results[0].names.as_ref().unwrap()[0];
    let url = format!("{BACKEND_URL}/tag/{tag_name}");
    let resp = reqwest::get(&url).await.expect("backend not reachable");
    assert!(resp.status().is_success(), "GET /tag/{tag_name} returned {}", resp.status());

    let tag: TagInfo = resp.json().await.expect("failed to deserialize TagInfo");
    assert!(tag.names.is_some(), "tag should have names");
    assert!(tag.names.as_ref().unwrap().contains(tag_name), "tag names should contain the queried name");
}

/// POST /tags without auth should fail.
#[tokio::test]
async fn create_tag_unauthenticated_fails() {
    let client = reqwest::Client::new();
    let url = format!("{BACKEND_URL}/tags");
    let resp = client
        .post(&url)
        .json(&serde_json::json!({
            "names": ["test-tag-unauth"],
            "category": "default"
        }))
        .send()
        .await
        .expect("backend not reachable");
    let status = resp.status().as_u16();
    assert!(status == 401 || status == 403, "unauthenticated POST /tags should fail, got {status}");
}

/// PUT /tag/{name} without auth should fail.
#[tokio::test]
async fn update_tag_unauthenticated_fails() {
    let list_url = format!("{BACKEND_URL}/tags?query=&offset=0&limit=1&fields=names");
    let list_resp = reqwest::get(&list_url).await.expect("backend not reachable");
    let page: PagedResponse<TagInfo> = list_resp.json().await.unwrap();

    if page.results.is_empty() {
        eprintln!("SKIP: no tags in database");
        return;
    }

    let tag_name = &page.results[0].names.as_ref().unwrap()[0];
    let client = reqwest::Client::new();
    let url = format!("{BACKEND_URL}/tag/{tag_name}");
    let resp = client
        .put(&url)
        .json(&serde_json::json!({
            "version": "fake",
            "category": "default"
        }))
        .send()
        .await
        .expect("backend not reachable");
    let status = resp.status().as_u16();
    assert!(status == 401 || status == 403, "unauthenticated PUT /tag/{tag_name} should fail, got {status}");
}

/// DELETE /tag/{name} without auth should fail.
#[tokio::test]
async fn delete_tag_unauthenticated_fails() {
    let list_url = format!("{BACKEND_URL}/tags?query=&offset=0&limit=1&fields=names");
    let list_resp = reqwest::get(&list_url).await.expect("backend not reachable");
    let page: PagedResponse<TagInfo> = list_resp.json().await.unwrap();

    if page.results.is_empty() {
        eprintln!("SKIP: no tags in database");
        return;
    }

    let tag_name = &page.results[0].names.as_ref().unwrap()[0];
    let client = reqwest::Client::new();
    let url = format!("{BACKEND_URL}/tag/{tag_name}");
    let resp = client
        .delete(&url)
        .json(&serde_json::json!({ "version": "fake" }))
        .send()
        .await
        .expect("backend not reachable");
    let status = resp.status().as_u16();
    assert!(status == 401 || status == 403, "unauthenticated DELETE /tag/{tag_name} should fail, got {status}");
}

/// POST /tag-merge without auth should fail.
#[tokio::test]
async fn merge_tags_unauthenticated_fails() {
    let client = reqwest::Client::new();
    let url = format!("{BACKEND_URL}/tag-merge");
    let resp = client
        .post(&url)
        .json(&serde_json::json!({
            "remove": "fake-tag-a",
            "mergeTo": "fake-tag-b",
            "removeVersion": "fake",
            "mergeToVersion": "fake"
        }))
        .send()
        .await
        .expect("backend not reachable");
    let status = resp.status().as_u16();
    assert!(status == 401 || status == 403, "unauthenticated POST /tag-merge should fail, got {status}");
}

/// GET /tag-siblings/{name} should return an unpaged response.
#[tokio::test]
async fn get_tag_siblings() {
    // First get any tag name
    let list_url = format!("{BACKEND_URL}/tags?query=&offset=0&limit=1&fields=names");
    let list_resp = reqwest::get(&list_url).await.expect("backend not reachable");
    let page: PagedResponse<TagInfo> = list_resp.json().await.unwrap();

    if page.results.is_empty() {
        eprintln!("SKIP: no tags in database");
        return;
    }

    let tag_name = &page.results[0].names.as_ref().unwrap()[0];
    let url = format!("{BACKEND_URL}/tag-siblings/{tag_name}");
    let resp = reqwest::get(&url).await.expect("backend not reachable");
    assert!(resp.status().is_success(), "GET /tag-siblings/{tag_name} returned {}", resp.status());

    let siblings: UnpagedResponse<TagSibling> = resp
        .json()
        .await
        .expect("failed to deserialize UnpagedResponse<TagSibling>");
    // Siblings may be empty, but the response should parse
    for sibling in &siblings.results {
        assert!(sibling.occurrences >= 0, "occurrences should be non-negative");
    }
}
