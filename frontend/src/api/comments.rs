use oxibooru_shared::comment::CommentInfo;
use oxibooru_shared::pagination::PagedResponse;

use super::{ApiClient, ApiError};

impl ApiClient {
    pub async fn get_comments(
        &self,
        query: &str,
        offset: i64,
        limit: i64,
    ) -> Result<PagedResponse<CommentInfo>, ApiError> {
        self.get(
            "/comments",
            &[
                ("query", query),
                ("offset", &offset.to_string()),
                ("limit", &limit.to_string()),
            ],
        )
        .await
    }
}
