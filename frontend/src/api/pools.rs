use oxibooru_shared::pagination::PagedResponse;
use oxibooru_shared::pool::PoolInfo;

use super::{ApiClient, ApiError};

impl ApiClient {
    pub async fn get_pools(
        &self,
        query: &str,
        offset: i64,
        limit: i64,
        fields: &str,
    ) -> Result<PagedResponse<PoolInfo>, ApiError> {
        self.get(
            "/pools",
            &[
                ("query", query),
                ("offset", &offset.to_string()),
                ("limit", &limit.to_string()),
                ("fields", fields),
            ],
        )
        .await
    }

    pub async fn get_pool(&self, id: i64) -> Result<PoolInfo, ApiError> {
        self.get(&format!("/pool/{id}"), &[]).await
    }
}
