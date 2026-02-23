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
