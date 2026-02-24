use oxibooru_shared::comment::CommentInfo;
use oxibooru_shared::enums::Rating;
use oxibooru_shared::pagination::PagedResponse;
use oxibooru_shared::request::{DeleteBody, RatingBody};
use serde::Serialize;

use super::{ApiClient, ApiError};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCommentBody {
    pub post_id: i64,
    pub text: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCommentBody {
    pub version: String,
    pub text: String,
}

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

    pub async fn create_comment(
        &self,
        body: &CreateCommentBody,
    ) -> Result<CommentInfo, ApiError> {
        self.post("/comments", body).await
    }

    pub async fn update_comment(
        &self,
        id: i64,
        body: &UpdateCommentBody,
    ) -> Result<CommentInfo, ApiError> {
        self.put(&format!("/comment/{id}"), body).await
    }

    pub async fn delete_comment(
        &self,
        id: i64,
        body: &DeleteBody,
    ) -> Result<(), ApiError> {
        self.delete(&format!("/comment/{id}"), body).await
    }

    pub async fn score_comment(
        &self,
        id: i64,
        score: Rating,
    ) -> Result<CommentInfo, ApiError> {
        self.put(&format!("/comment/{id}/score"), &RatingBody { score })
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_create_comment_body() {
        let body = CreateCommentBody {
            post_id: 42,
            text: "Great post!".into(),
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("\"postId\":42"));
        assert!(json.contains("\"text\":\"Great post!\""));
    }

    #[test]
    fn serialize_update_comment_body() {
        let body = UpdateCommentBody {
            version: "2024-01-15T10:30:45Z".into(),
            text: "Updated comment".into(),
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("\"version\":\"2024-01-15T10:30:45Z\""));
        assert!(json.contains("\"text\":\"Updated comment\""));
    }
}
