use oxibooru_shared::pagination::{PagedResponse, UnpagedResponse};
use oxibooru_shared::request::{DeleteBody, MergeBody};
use oxibooru_shared::tag::{TagInfo, TagSibling};
use serde::Serialize;

use super::{ApiClient, ApiError};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTagBody {
    pub category: String,
    pub names: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implications: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestions: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTagBody {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implications: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestions: Option<Vec<String>>,
}

fn url_encode(s: &str) -> String {
    js_sys::encode_uri_component(s).into()
}

impl ApiClient {
    pub async fn get_tags(
        &self,
        query: &str,
        offset: i64,
        limit: i64,
        fields: &str,
    ) -> Result<PagedResponse<TagInfo>, ApiError> {
        self.get(
            "/tags",
            &[
                ("query", query),
                ("offset", &offset.to_string()),
                ("limit", &limit.to_string()),
                ("fields", fields),
            ],
        )
        .await
    }

    pub async fn get_tag(&self, name: &str) -> Result<TagInfo, ApiError> {
        self.get(&format!("/tag/{}", url_encode(name)), &[]).await
    }

    pub async fn get_tag_siblings(
        &self,
        name: &str,
    ) -> Result<UnpagedResponse<TagSibling>, ApiError> {
        self.get(&format!("/tag-siblings/{}", url_encode(name)), &[])
            .await
    }

    pub async fn create_tag(
        &self,
        body: &CreateTagBody,
    ) -> Result<TagInfo, ApiError> {
        self.post("/tags", body).await
    }

    pub async fn update_tag(
        &self,
        name: &str,
        body: &UpdateTagBody,
    ) -> Result<TagInfo, ApiError> {
        self.put(&format!("/tag/{}", url_encode(name)), body).await
    }

    pub async fn delete_tag(
        &self,
        name: &str,
        body: &DeleteBody,
    ) -> Result<(), ApiError> {
        self.delete(&format!("/tag/{}", url_encode(name)), body)
            .await
    }

    pub async fn merge_tags(
        &self,
        body: &MergeBody<String>,
    ) -> Result<TagInfo, ApiError> {
        self.post("/tag-merge", body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_create_tag_body() {
        let body = CreateTagBody {
            category: "character".into(),
            names: vec!["alice".into(), "alice_wonderland".into()],
            description: None,
            implications: Some(vec!["female".into()]),
            suggestions: None,
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("\"category\":\"character\""));
        assert!(json.contains("\"names\":[\"alice\",\"alice_wonderland\"]"));
        assert!(json.contains("\"implications\":[\"female\"]"));
        assert!(!json.contains("\"description\""));
        assert!(!json.contains("\"suggestions\""));
    }

    #[test]
    fn serialize_update_tag_body_skips_none() {
        let body = UpdateTagBody {
            version: "v1".into(),
            category: Some("artist".into()),
            names: None,
            description: None,
            implications: None,
            suggestions: None,
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("\"category\":\"artist\""));
        assert!(!json.contains("\"names\""));
    }
}
