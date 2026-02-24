use oxibooru_shared::pagination::PagedResponse;
use oxibooru_shared::snapshot::SnapshotInfo;

const BACKEND_URL: &str = "http://localhost:6666";

/// GET /snapshots should return a paged response.
#[tokio::test]
async fn get_snapshots_returns_paged_response() {
    let url = format!("{BACKEND_URL}/snapshots?offset=0&limit=5");
    let resp = reqwest::get(&url)
        .await
        .expect("backend not reachable — is it running on port 6666?");
    assert!(resp.status().is_success(), "GET /snapshots returned {}", resp.status());

    let page: PagedResponse<SnapshotInfo> = resp
        .json()
        .await
        .expect("failed to deserialize PagedResponse<SnapshotInfo>");
    assert!(page.total >= 0);
    assert!(page.limit == 5);
    assert!(page.results.len() <= 5);
}

/// Snapshots should have operation, type, and time fields when present.
#[tokio::test]
async fn get_snapshots_have_expected_fields() {
    let url = format!("{BACKEND_URL}/snapshots?offset=0&limit=3");
    let resp = reqwest::get(&url).await.expect("backend not reachable");
    let page: PagedResponse<SnapshotInfo> = resp.json().await.unwrap();

    for snap in &page.results {
        assert!(snap.operation.is_some(), "snapshot should have operation");
        assert!(snap.resource_type.is_some(), "snapshot should have resource_type");
        assert!(snap.time.is_some(), "snapshot should have time");
    }
}
