use oxibooru_shared::pagination::PagedResponse;
use oxibooru_shared::pool::PoolInfo;

const BACKEND_URL: &str = "http://localhost:6666";

/// GET /pools should return a paged response.
#[tokio::test]
async fn get_pools_returns_paged_response() {
    let url = format!("{BACKEND_URL}/pools?query=&offset=0&limit=5&fields=id,names,category,postCount");
    let resp = reqwest::get(&url)
        .await
        .expect("backend not reachable — is it running on port 6666?");
    assert!(resp.status().is_success(), "GET /pools returned {}", resp.status());

    let page: PagedResponse<PoolInfo> = resp.json().await.expect("failed to deserialize PagedResponse<PoolInfo>");
    assert!(page.total >= 0);
    assert!(page.limit == 5);
    assert!(page.results.len() <= 5);
}

/// GET /pool/{id} should return a single pool if any exist.
#[tokio::test]
async fn get_single_pool() {
    let list_url = format!("{BACKEND_URL}/pools?query=&offset=0&limit=1&fields=id");
    let list_resp = reqwest::get(&list_url).await.expect("backend not reachable");
    let page: PagedResponse<PoolInfo> = list_resp.json().await.unwrap();

    if page.results.is_empty() {
        eprintln!("SKIP: no pools in database");
        return;
    }

    let pool_id = page.results[0].id.unwrap();
    let url = format!("{BACKEND_URL}/pool/{pool_id}");
    let resp = reqwest::get(&url).await.expect("backend not reachable");
    assert!(resp.status().is_success(), "GET /pool/{pool_id} returned {}", resp.status());

    let pool: PoolInfo = resp.json().await.expect("failed to deserialize PoolInfo");
    assert_eq!(pool.id, Some(pool_id));
}

/// POST /pool without auth should fail.
#[tokio::test]
async fn create_pool_unauthenticated_fails() {
    let client = reqwest::Client::new();
    let url = format!("{BACKEND_URL}/pool");
    let resp = client
        .post(&url)
        .json(&serde_json::json!({
            "names": ["test-pool-unauth"],
            "category": "default"
        }))
        .send()
        .await
        .expect("backend not reachable");
    let status = resp.status().as_u16();
    assert!(
        status == 401 || status == 403,
        "unauthenticated POST /pool should fail, got {status}"
    );
}

/// PUT /pool/{id} without auth should fail.
#[tokio::test]
async fn update_pool_unauthenticated_fails() {
    let list_url = format!("{BACKEND_URL}/pools?query=&offset=0&limit=1&fields=id");
    let list_resp = reqwest::get(&list_url).await.expect("backend not reachable");
    let page: PagedResponse<PoolInfo> = list_resp.json().await.unwrap();

    if page.results.is_empty() {
        eprintln!("SKIP: no pools in database");
        return;
    }

    let pool_id = page.results[0].id.unwrap();
    let client = reqwest::Client::new();
    let url = format!("{BACKEND_URL}/pool/{pool_id}");
    let resp = client
        .put(&url)
        .json(&serde_json::json!({
            "version": "fake",
            "names": ["test-rename"]
        }))
        .send()
        .await
        .expect("backend not reachable");
    let status = resp.status().as_u16();
    assert!(
        status == 401 || status == 403,
        "unauthenticated PUT /pool/{pool_id} should fail, got {status}"
    );
}

/// DELETE /pool/{id} without auth should fail.
#[tokio::test]
async fn delete_pool_unauthenticated_fails() {
    let list_url = format!("{BACKEND_URL}/pools?query=&offset=0&limit=1&fields=id");
    let list_resp = reqwest::get(&list_url).await.expect("backend not reachable");
    let page: PagedResponse<PoolInfo> = list_resp.json().await.unwrap();

    if page.results.is_empty() {
        eprintln!("SKIP: no pools in database");
        return;
    }

    let pool_id = page.results[0].id.unwrap();
    let client = reqwest::Client::new();
    let url = format!("{BACKEND_URL}/pool/{pool_id}");
    let resp = client
        .delete(&url)
        .json(&serde_json::json!({ "version": "fake" }))
        .send()
        .await
        .expect("backend not reachable");
    let status = resp.status().as_u16();
    assert!(
        status == 401 || status == 403,
        "unauthenticated DELETE /pool/{pool_id} should fail, got {status}"
    );
}

/// POST /pool-merge without auth should fail.
#[tokio::test]
async fn merge_pools_unauthenticated_fails() {
    let client = reqwest::Client::new();
    let url = format!("{BACKEND_URL}/pool-merge");
    let resp = client
        .post(&url)
        .json(&serde_json::json!({
            "remove": 999999,
            "mergeTo": 999998,
            "removeVersion": "fake",
            "mergeToVersion": "fake"
        }))
        .send()
        .await
        .expect("backend not reachable");
    let status = resp.status().as_u16();
    assert!(
        status == 401 || status == 403,
        "unauthenticated POST /pool-merge should fail, got {status}"
    );
}
