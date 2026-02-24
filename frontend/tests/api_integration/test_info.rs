use oxibooru_shared::info::InfoResponse;

const BACKEND_URL: &str = "http://localhost:6666";

/// Integration test: GET /info should return a valid InfoResponse.
/// Requires the backend to be running on port 6666.
#[tokio::test]
async fn get_info_returns_valid_response() {
    let url = format!("{BACKEND_URL}/info");
    let resp = reqwest::get(&url)
        .await
        .expect("backend not reachable — is it running on port 6666?");
    assert!(resp.status().is_success(), "GET /info returned {}", resp.status());

    let info: InfoResponse = resp.json().await.expect("failed to deserialize InfoResponse");

    // Server config is always present (not Option)
    assert!(!info.config.name.is_empty(), "config.name should not be empty");
    assert!(!info.config.username_regex.is_empty(), "username_regex should not be empty");

    // Privilege config is nested inside config
    assert!(info.config.privileges.get("posts:list").is_some(), "privileges should include posts:list");

    // Sanity check top-level fields
    assert!(info.post_count >= 0, "post_count should be non-negative");
    assert!(info.disk_usage >= 0, "disk_usage should be non-negative");
    assert!(!info.server_time.is_empty(), "server_time should not be empty");
}

/// Integration test: featured post field should be present (even if null).
#[tokio::test]
async fn get_info_has_featured_post_field() {
    let url = format!("{BACKEND_URL}/info");
    let resp = reqwest::get(&url).await.expect("backend not reachable");
    let body: serde_json::Value = resp.json().await.unwrap();

    // The JSON should have a "featured_post" key (may be null)
    // InfoResponse uses snake_case (no rename_all)
    assert!(body.get("featured_post").is_some(), "response should contain 'featured_post' key");
}
