use oxibooru_shared::enums::{PostSafety, Rating};
use oxibooru_shared::pagination::PagedResponse;
use oxibooru_shared::post::{Note, PostInfo, PostNeighbors, ReverseSearchResponse};
use oxibooru_shared::request::{DeleteBody, RatingBody};
use serde::Serialize;

use super::{ApiClient, ApiError};

/// Body for POST /posts (JSON metadata part of multipart).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePostBody {
    pub safety: PostSafety,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relations: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<Note>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,
}

/// Body for PUT /post/{id}.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePostBody {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety: Option<PostSafety>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relations: Option<Vec<i64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<Vec<Note>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_token: Option<String>,
}

/// Body for POST /post-merge.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostMergeBody {
    pub remove: i64,
    pub merge_to: i64,
    pub remove_version: String,
    pub merge_to_version: String,
    pub replace_content: bool,
}

/// Body for POST /posts/reverse-search (JSON).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReverseSearchBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_url: Option<String>,
}

/// Body for POST /featured-post.
#[derive(Debug, Serialize)]
pub struct FeatureBody {
    pub id: i64,
}

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

    pub async fn get_post_around(&self, id: i64, query: &str, fields: &str) -> Result<PostNeighbors, ApiError> {
        self.get(&format!("/post/{id}/around"), &[("query", query), ("fields", fields)])
            .await
    }

    pub async fn get_featured_post(&self) -> Result<PostInfo, ApiError> {
        self.get("/featured-post", &[]).await
    }

    /// POST /posts (multipart: metadata JSON + content binary).
    pub async fn create_post(&self, form_data: &web_sys::FormData) -> Result<PostInfo, ApiError> {
        self.post_multipart("/posts", form_data).await
    }

    /// POST /posts (JSON body — when content is already uploaded via token or URL).
    pub async fn create_post_json(&self, body: &CreatePostBody) -> Result<PostInfo, ApiError> {
        self.post("/posts", body).await
    }

    /// PUT /post/{id} (JSON body — metadata-only update).
    pub async fn update_post_json(&self, id: i64, body: &UpdatePostBody) -> Result<PostInfo, ApiError> {
        self.put(&format!("/post/{id}"), body).await
    }

    /// PUT /post/{id} (multipart — for content/thumbnail replacement).
    pub async fn update_post(&self, id: i64, form_data: &web_sys::FormData) -> Result<PostInfo, ApiError> {
        self.put_multipart(&format!("/post/{id}"), form_data).await
    }

    /// DELETE /post/{id}.
    pub async fn delete_post(&self, id: i64, body: &DeleteBody) -> Result<(), ApiError> {
        self.delete(&format!("/post/{id}"), body).await
    }

    /// PUT /post/{id}/score.
    pub async fn score_post(&self, id: i64, score: Rating) -> Result<PostInfo, ApiError> {
        self.put(&format!("/post/{id}/score"), &RatingBody { score }).await
    }

    /// POST /post/{id}/favorite (no body).
    pub async fn add_favorite(&self, id: i64) -> Result<PostInfo, ApiError> {
        self.post_no_body(&format!("/post/{id}/favorite")).await
    }

    /// DELETE /post/{id}/favorite (no body, returns PostInfo).
    pub async fn remove_favorite(&self, id: i64) -> Result<PostInfo, ApiError> {
        self.delete_with_response(&format!("/post/{id}/favorite")).await
    }

    /// POST /post-merge.
    pub async fn merge_posts(&self, body: &PostMergeBody) -> Result<PostInfo, ApiError> {
        self.post("/post-merge", body).await
    }

    /// POST /posts/reverse-search (multipart).
    pub async fn reverse_search(&self, form_data: &web_sys::FormData) -> Result<ReverseSearchResponse, ApiError> {
        self.post_multipart("/posts/reverse-search", form_data).await
    }

    /// POST /posts/reverse-search (JSON — for token/URL based search).
    pub async fn reverse_search_json(&self, body: &ReverseSearchBody) -> Result<ReverseSearchResponse, ApiError> {
        self.post("/posts/reverse-search", body).await
    }

    /// POST /featured-post.
    pub async fn feature_post(&self, body: &FeatureBody) -> Result<PostInfo, ApiError> {
        self.post("/featured-post", body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_create_post_body_minimal() {
        let body = CreatePostBody {
            safety: PostSafety::Safe,
            content_token: Some("abc-123".into()),
            content_url: None,
            thumbnail_token: None,
            source: None,
            description: None,
            relations: None,
            anonymous: None,
            tags: Some(vec!["landscape".into(), "nature".into()]),
            notes: None,
            flags: None,
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("\"safety\":\"safe\""));
        assert!(json.contains("\"contentToken\":\"abc-123\""));
        assert!(json.contains("\"tags\":[\"landscape\",\"nature\"]"));
        assert!(!json.contains("\"contentUrl\""));
        assert!(!json.contains("\"anonymous\""));
    }

    #[test]
    fn serialize_update_post_body_skips_none() {
        let body = UpdatePostBody {
            version: "2024-01-15T10:30:45Z".into(),
            safety: Some(PostSafety::Sketchy),
            source: None,
            description: None,
            relations: None,
            tags: Some(vec!["tag1".into()]),
            notes: None,
            flags: None,
            content_token: None,
            content_url: None,
            thumbnail_token: None,
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("\"safety\":\"sketchy\""));
        assert!(json.contains("\"tags\":[\"tag1\"]"));
        assert!(!json.contains("\"source\""));
        assert!(!json.contains("\"contentToken\""));
    }

    #[test]
    fn serialize_post_merge_body() {
        let body = PostMergeBody {
            remove: 1,
            merge_to: 2,
            remove_version: "v1".into(),
            merge_to_version: "v2".into(),
            replace_content: true,
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("\"mergeTo\":2"));
        assert!(json.contains("\"replaceContent\":true"));
    }

    #[test]
    fn serialize_feature_body() {
        let body = FeatureBody { id: 42 };
        let json = serde_json::to_string(&body).unwrap();
        assert_eq!(json, r#"{"id":42}"#);
    }

    #[test]
    fn serialize_reverse_search_body_skips_none() {
        let body = ReverseSearchBody {
            content_token: Some("tok-123".into()),
            content_url: None,
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("\"contentToken\":\"tok-123\""));
        assert!(!json.contains("\"contentUrl\""));
    }
}
