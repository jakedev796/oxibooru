const BACKEND_URL: &str = "http://localhost:6666";

/// Integration test: GET /password-reset/{nonexistent} should return 404.
/// Requires the backend to be running on port 6666.
#[tokio::test]
async fn password_reset_nonexistent_user_returns_404() {
    let url = format!("{BACKEND_URL}/password-reset/nonexistent_user_12345");
    let resp = reqwest::get(&url)
        .await
        .expect("backend not reachable — is it running on port 6666?");
    assert_eq!(
        resp.status().as_u16(),
        404,
        "password reset for nonexistent user should return 404, got {}",
        resp.status()
    );
}

/// Integration test: POST /password-reset/{user} with invalid token should return error.
#[tokio::test]
async fn password_reset_invalid_token_returns_error() {
    let client = reqwest::Client::new();
    let url = format!("{BACKEND_URL}/password-reset/nonexistent_user_12345");
    let resp = client
        .post(&url)
        .json(&serde_json::json!({"token": "invalid-token"}))
        .send()
        .await
        .expect("backend not reachable");
    assert!(
        resp.status().is_client_error(),
        "invalid token should return client error, got {}",
        resp.status()
    );
}
