use oxibooru_shared::pagination::PagedResponse;
use oxibooru_shared::snapshot::SnapshotInfo;

use super::{ApiClient, ApiError};

impl ApiClient {
    pub async fn get_snapshots(
        &self,
        offset: i64,
        limit: i64,
    ) -> Result<PagedResponse<SnapshotInfo>, ApiError> {
        self.get(
            "/snapshots",
            &[
                ("offset", &offset.to_string()),
                ("limit", &limit.to_string()),
            ],
        )
        .await
    }
}
