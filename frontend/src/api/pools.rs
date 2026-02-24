use oxibooru_shared::pagination::PagedResponse;
use oxibooru_shared::pool::PoolInfo;
use oxibooru_shared::request::{DeleteBody, MergeBody};
use serde::Serialize;

use super::{ApiClient, ApiError};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePoolBody {
    pub names: Vec<String>,
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posts: Option<Vec<i64>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePoolBody {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posts: Option<Vec<i64>>,
}

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

    /// POST /pool (singular!).
    pub async fn create_pool(
        &self,
        body: &CreatePoolBody,
    ) -> Result<PoolInfo, ApiError> {
        self.post("/pool", body).await
    }

    pub async fn update_pool(
        &self,
        id: i64,
        body: &UpdatePoolBody,
    ) -> Result<PoolInfo, ApiError> {
        self.put(&format!("/pool/{id}"), body).await
    }

    pub async fn delete_pool(
        &self,
        id: i64,
        body: &DeleteBody,
    ) -> Result<(), ApiError> {
        self.delete(&format!("/pool/{id}"), body).await
    }

    pub async fn merge_pools(
        &self,
        body: &MergeBody<i64>,
    ) -> Result<PoolInfo, ApiError> {
        self.post("/pool-merge", body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_create_pool_body() {
        let body = CreatePoolBody {
            names: vec!["my_pool".into()],
            category: "series".into(),
            description: Some("A pool".into()),
            posts: Some(vec![1, 2, 3]),
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("\"names\":[\"my_pool\"]"));
        assert!(json.contains("\"category\":\"series\""));
        assert!(json.contains("\"posts\":[1,2,3]"));
    }

    #[test]
    fn serialize_update_pool_body_skips_none() {
        let body = UpdatePoolBody {
            version: "v1".into(),
            category: None,
            names: Some(vec!["renamed".into()]),
            description: None,
            posts: None,
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("\"names\":[\"renamed\"]"));
        assert!(!json.contains("\"category\""));
        assert!(!json.contains("\"posts\""));
    }
}
