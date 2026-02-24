use oxibooru_shared::category::{PoolCategoryInfo, TagCategoryInfo};
use oxibooru_shared::pagination::UnpagedResponse;
use oxibooru_shared::request::DeleteBody;
use serde::Serialize;

use super::{ApiClient, ApiError};

#[derive(Debug, Serialize)]
pub struct CreateTagCategoryBody {
    pub name: String,
    pub color: String,
    pub order: i32,
}

#[derive(Debug, Serialize)]
pub struct UpdateTagCategoryBody {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct CreatePoolCategoryBody {
    pub name: String,
    pub color: String,
}

#[derive(Debug, Serialize)]
pub struct UpdatePoolCategoryBody {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

fn url_encode(s: &str) -> String {
    js_sys::encode_uri_component(s).into()
}

impl ApiClient {
    // --- Tag categories ---

    pub async fn get_tag_categories(
        &self,
    ) -> Result<UnpagedResponse<TagCategoryInfo>, ApiError> {
        self.get("/tag-categories", &[]).await
    }

    pub async fn create_tag_category(
        &self,
        body: &CreateTagCategoryBody,
    ) -> Result<TagCategoryInfo, ApiError> {
        self.post("/tag-categories", body).await
    }

    pub async fn update_tag_category(
        &self,
        name: &str,
        body: &UpdateTagCategoryBody,
    ) -> Result<TagCategoryInfo, ApiError> {
        self.put(&format!("/tag-category/{}", url_encode(name)), body)
            .await
    }

    pub async fn delete_tag_category(
        &self,
        name: &str,
        body: &DeleteBody,
    ) -> Result<(), ApiError> {
        self.delete(&format!("/tag-category/{}", url_encode(name)), body)
            .await
    }

    pub async fn set_default_tag_category(
        &self,
        name: &str,
    ) -> Result<TagCategoryInfo, ApiError> {
        self.put(
            &format!("/tag-category/{}/default", url_encode(name)),
            &serde_json::json!({}),
        )
        .await
    }

    // --- Pool categories ---

    pub async fn get_pool_categories(
        &self,
    ) -> Result<UnpagedResponse<PoolCategoryInfo>, ApiError> {
        self.get("/pool-categories", &[]).await
    }

    pub async fn create_pool_category(
        &self,
        body: &CreatePoolCategoryBody,
    ) -> Result<PoolCategoryInfo, ApiError> {
        self.post("/pool-categories", body).await
    }

    pub async fn update_pool_category(
        &self,
        name: &str,
        body: &UpdatePoolCategoryBody,
    ) -> Result<PoolCategoryInfo, ApiError> {
        self.put(&format!("/pool-category/{}", url_encode(name)), body)
            .await
    }

    pub async fn delete_pool_category(
        &self,
        name: &str,
        body: &DeleteBody,
    ) -> Result<(), ApiError> {
        self.delete(&format!("/pool-category/{}", url_encode(name)), body)
            .await
    }

    pub async fn set_default_pool_category(
        &self,
        name: &str,
    ) -> Result<PoolCategoryInfo, ApiError> {
        self.put(
            &format!("/pool-category/{}/default", url_encode(name)),
            &serde_json::json!({}),
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_create_tag_category_body() {
        let body = CreateTagCategoryBody {
            name: "character".into(),
            color: "#FF0000".into(),
            order: 1,
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("\"name\":\"character\""));
        assert!(json.contains("\"color\":\"#FF0000\""));
        assert!(json.contains("\"order\":1"));
    }

    #[test]
    fn serialize_update_tag_category_body_skips_none() {
        let body = UpdateTagCategoryBody {
            version: "v1".into(),
            name: Some("new_name".into()),
            color: None,
            order: None,
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("\"name\":\"new_name\""));
        assert!(!json.contains("color"));
        assert!(!json.contains("order"));
    }

    #[test]
    fn serialize_create_pool_category_body() {
        let body = CreatePoolCategoryBody {
            name: "series".into(),
            color: "#00FF00".into(),
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("\"name\":\"series\""));
        assert!(json.contains("\"color\":\"#00FF00\""));
    }
}
