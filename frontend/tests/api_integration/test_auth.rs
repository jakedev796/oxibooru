use oxibooru_shared::info::InfoResponse;

const BACKEND_URL: &str = "http://localhost:6666";

/// Integration test: GET /info?bump-login=true without credentials should still return info.
/// Requires the backend to be running on port 6666.
#[tokio::test]
async fn info_bump_login_anonymous_returns_ok() {
    let url = format!("{BACKEND_URL}/info?bump-login=true");
    let resp = reqwest::get(&url)
        .await
        .expect("backend not reachable — is it running on port 6666?");
    assert!(
        resp.status().is_success(),
        "GET /info?bump-login=true should return success for anonymous, got {}",
        resp.status()
    );

    let info: InfoResponse = resp.json().await.expect("failed to deserialize InfoResponse");
    assert!(!info.config.name.is_empty());
}

/// Integration test: GET /info with invalid Basic auth should return 401.
#[tokio::test]
async fn info_with_invalid_credentials_returns_401() {
    let client = reqwest::Client::new();
    let url = format!("{BACKEND_URL}/info?bump-login=true");
    let resp = client
        .get(&url)
        .header("Authorization", "Basic bm9ib2R5Om5vcGFzcw==") // nobody:nopass
        .send()
        .await
        .expect("backend not reachable");
    assert_eq!(resp.status().as_u16(), 401, "invalid credentials should return 401, got {}", resp.status());
}
