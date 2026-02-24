use oxibooru_shared::enums::{AvatarStyle, UserRank};
use oxibooru_shared::pagination::PagedResponse;
use oxibooru_shared::request::DeleteBody;
use oxibooru_shared::user::UserInfo;
use serde::Serialize;

use super::{ApiClient, ApiError};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserBody {
    pub name: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<UserRank>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_style: Option<AvatarStyle>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserBody {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<UserRank>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_style: Option<AvatarStyle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}

impl ApiClient {
    pub async fn get_users(&self, query: &str, offset: i64, limit: i64) -> Result<PagedResponse<UserInfo>, ApiError> {
        self.get(
            "/users",
            &[
                ("query", query),
                ("offset", &offset.to_string()),
                ("limit", &limit.to_string()),
            ],
        )
        .await
    }

    pub async fn get_user(&self, name: &str) -> Result<UserInfo, ApiError> {
        self.get(&format!("/user/{}", url_encode(name)), &[]).await
    }

    pub async fn create_user(&self, body: &CreateUserBody) -> Result<UserInfo, ApiError> {
        self.post("/users", body).await
    }

    pub async fn update_user(&self, name: &str, body: &UpdateUserBody) -> Result<UserInfo, ApiError> {
        self.put(&format!("/user/{}", url_encode(name)), body).await
    }

    pub async fn delete_user(&self, name: &str, body: &DeleteBody) -> Result<(), ApiError> {
        self.delete(&format!("/user/{}", url_encode(name)), body).await
    }
}

fn url_encode(s: &str) -> String {
    js_sys::encode_uri_component(s).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_create_user_body() {
        let body = CreateUserBody {
            name: "alice".into(),
            password: "secret123".into(),
            email: Some("alice@example.com".into()),
            rank: None,
            avatar_style: None,
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("\"name\":\"alice\""));
        assert!(json.contains("\"password\":\"secret123\""));
        assert!(json.contains("\"email\":\"alice@example.com\""));
        assert!(!json.contains("rank"));
        assert!(!json.contains("avatarStyle"));
    }

    #[test]
    fn serialize_update_user_body_partial() {
        let body = UpdateUserBody {
            version: "2024-01-01T00:00:00Z".into(),
            name: None,
            password: None,
            email: Some(Some("new@example.com".into())),
            rank: None,
            avatar_style: None,
            avatar_url: None,
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("\"version\""));
        assert!(json.contains("\"email\":\"new@example.com\""));
        assert!(!json.contains("\"name\""));
        assert!(!json.contains("\"password\""));
    }

    #[test]
    fn serialize_update_user_body_clear_email() {
        let body = UpdateUserBody {
            version: "v1".into(),
            name: None,
            password: None,
            email: Some(None), // explicitly clear email
            rank: None,
            avatar_style: None,
            avatar_url: None,
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("\"email\":null"));
    }
}
