use oxibooru_shared::category::PoolCategoryInfo;
use oxibooru_shared::pagination::UnpagedResponse;

const BACKEND_URL: &str = "http://localhost:6666";

/// GET /pool-categories should return an unpaged response.
#[tokio::test]
async fn get_pool_categories_returns_unpaged() {
    let url = format!("{BACKEND_URL}/pool-categories");
    let resp = reqwest::get(&url)
        .await
        .expect("backend not reachable — is it running on port 6666?");
    assert!(resp.status().is_success(), "GET /pool-categories returned {}", resp.status());

    let data: UnpagedResponse<PoolCategoryInfo> = resp
        .json()
        .await
        .expect("failed to deserialize UnpagedResponse<PoolCategoryInfo>");
    assert!(!data.results.is_empty(), "should have at least one pool category (default)");

    let has_default = data.results.iter().any(|c| c.default == Some(true));
    assert!(has_default, "should have a default pool category");
}

/// Pool categories should have names and colors.
#[tokio::test]
async fn pool_categories_have_expected_fields() {
    let url = format!("{BACKEND_URL}/pool-categories");
    let resp = reqwest::get(&url).await.expect("backend not reachable");
    let data: UnpagedResponse<PoolCategoryInfo> = resp.json().await.unwrap();

    for cat in &data.results {
        assert!(cat.name.is_some(), "category should have a name");
        assert!(cat.color.is_some(), "category should have a color");
    }
}

/// POST /pool-categories without auth should fail.
#[tokio::test]
async fn create_pool_category_unauthenticated_fails() {
    let client = reqwest::Client::new();
    let url = format!("{BACKEND_URL}/pool-categories");
    let resp = client
        .post(&url)
        .json(&serde_json::json!({
            "name": "test-pool-cat-unauth",
            "color": "#00ff00"
        }))
        .send()
        .await
        .expect("backend not reachable");
    let status = resp.status().as_u16();
    assert!(status == 401 || status == 403, "unauthenticated POST /pool-categories should fail, got {status}");
}
