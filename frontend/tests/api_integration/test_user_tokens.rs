const BACKEND_URL: &str = "http://localhost:6666";

/// Integration test: GET /user-tokens/{nonexistent} without auth should return error.
/// Requires the backend to be running on port 6666.
#[tokio::test]
async fn user_tokens_unauthenticated_returns_error() {
    let url = format!("{BACKEND_URL}/user-tokens/nobody");
    let resp = reqwest::get(&url)
        .await
        .expect("backend not reachable — is it running on port 6666?");
    assert!(
        resp.status().is_client_error(),
        "unauthenticated token list should return client error, got {}",
        resp.status()
    );
}

/// Integration test: POST /user-token/{nonexistent} without auth should return error.
#[tokio::test]
async fn create_user_token_unauthenticated_returns_error() {
    let client = reqwest::Client::new();
    let url = format!("{BACKEND_URL}/user-token/nobody");
    let resp = client
        .post(&url)
        .json(&serde_json::json!({"enabled": true, "note": "test"}))
        .send()
        .await
        .expect("backend not reachable");
    assert!(
        resp.status().is_client_error(),
        "unauthenticated token create should return client error, got {}",
        resp.status()
    );
}
