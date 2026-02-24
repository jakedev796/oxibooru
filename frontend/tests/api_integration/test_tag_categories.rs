use oxibooru_shared::category::TagCategoryInfo;
use oxibooru_shared::pagination::UnpagedResponse;

const BACKEND_URL: &str = "http://localhost:6666";

/// GET /tag-categories should return an unpaged response.
#[tokio::test]
async fn get_tag_categories_returns_unpaged() {
    let url = format!("{BACKEND_URL}/tag-categories");
    let resp = reqwest::get(&url)
        .await
        .expect("backend not reachable — is it running on port 6666?");
    assert!(
        resp.status().is_success(),
        "GET /tag-categories returned {}",
        resp.status()
    );

    let data: UnpagedResponse<TagCategoryInfo> = resp
        .json()
        .await
        .expect("failed to deserialize UnpagedResponse<TagCategoryInfo>");
    assert!(
        !data.results.is_empty(),
        "should have at least one tag category (default)"
    );

    // At least one should be default
    let has_default = data.results.iter().any(|c| c.default == Some(true));
    assert!(has_default, "should have a default tag category");
}

/// Tag categories should have names and colors.
#[tokio::test]
async fn tag_categories_have_expected_fields() {
    let url = format!("{BACKEND_URL}/tag-categories");
    let resp = reqwest::get(&url).await.expect("backend not reachable");
    let data: UnpagedResponse<TagCategoryInfo> = resp.json().await.unwrap();

    for cat in &data.results {
        assert!(cat.name.is_some(), "category should have a name");
        assert!(cat.color.is_some(), "category should have a color");
    }
}

/// POST /tag-categories without auth should fail.
#[tokio::test]
async fn create_tag_category_unauthenticated_fails() {
    let client = reqwest::Client::new();
    let url = format!("{BACKEND_URL}/tag-categories");
    let resp = client
        .post(&url)
        .json(&serde_json::json!({
            "name": "test-cat-unauth",
            "color": "#ff0000"
        }))
        .send()
        .await
        .expect("backend not reachable");
    let status = resp.status().as_u16();
    assert!(
        status == 401 || status == 403,
        "unauthenticated POST /tag-categories should fail, got {status}"
    );
}
