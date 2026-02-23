use oxibooru_shared::pagination::PagedResponse;
use oxibooru_shared::post::{PostInfo, PostNeighbors};

use super::{ApiClient, ApiError};

impl ApiClient {
    pub async fn get_posts(
        &self,
        query: &str,
        offset: i64,
        limit: i64,
        fields: &str,
    ) -> Result<PagedResponse<PostInfo>, ApiError> {
        self.get(
            "/posts",
            &[
                ("query", query),
                ("offset", &offset.to_string()),
                ("limit", &limit.to_string()),
                ("fields", fields),
            ],
        )
        .await
    }

    pub async fn get_post(&self, id: i64) -> Result<PostInfo, ApiError> {
        self.get(&format!("/post/{id}"), &[]).await
    }

    pub async fn get_post_around(
        &self,
        id: i64,
        query: &str,
        fields: &str,
    ) -> Result<PostNeighbors, ApiError> {
        self.get(
            &format!("/post/{id}/around"),
            &[("query", query), ("fields", fields)],
        )
        .await
    }

    pub async fn get_featured_post(&self) -> Result<PostInfo, ApiError> {
        self.get("/featured-post", &[]).await
    }
}
