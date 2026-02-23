use oxibooru_shared::pagination::UnpagedResponse;
use oxibooru_shared::user_token::UserTokenInfo;
use serde::Serialize;

use super::{ApiClient, ApiError};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserTokenBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserTokenBody {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<Option<String>>,
}

impl ApiClient {
    pub async fn get_user_tokens(
        &self,
        username: &str,
    ) -> Result<UnpagedResponse<UserTokenInfo>, ApiError> {
        self.get(&format!("/user-tokens/{}", url_encode(username)), &[])
            .await
    }

    pub async fn create_user_token(
        &self,
        username: &str,
        body: &CreateUserTokenBody,
    ) -> Result<UserTokenInfo, ApiError> {
        self.post(&format!("/user-token/{}", url_encode(username)), body)
            .await
    }

    pub async fn update_user_token(
        &self,
        username: &str,
        token: &str,
        body: &UpdateUserTokenBody,
    ) -> Result<UserTokenInfo, ApiError> {
        self.put(
            &format!("/user-token/{}/{}", url_encode(username), url_encode(token)),
            body,
        )
        .await
    }

    pub async fn delete_user_token(
        &self,
        username: &str,
        token: &str,
    ) -> Result<(), ApiError> {
        self.delete(
            &format!("/user-token/{}/{}", url_encode(username), url_encode(token)),
            &serde_json::json!({}),
        )
        .await
    }
}

fn url_encode(s: &str) -> String {
    js_sys::encode_uri_component(s).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_create_user_token_body() {
        let body = CreateUserTokenBody {
            enabled: Some(true),
            note: Some("test token".into()),
            expiration_time: None,
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("\"enabled\":true"));
        assert!(json.contains("\"note\":\"test token\""));
        assert!(!json.contains("expirationTime"));
    }

    #[test]
    fn serialize_update_user_token_body() {
        let body = UpdateUserTokenBody {
            version: "2024-01-01T00:00:00Z".into(),
            enabled: Some(false),
            note: None,
            expiration_time: Some(None), // explicitly clear expiration
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("\"enabled\":false"));
        assert!(!json.contains("\"note\""));
        assert!(json.contains("\"expirationTime\":null"));
    }
}
