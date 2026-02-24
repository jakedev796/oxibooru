use serde::{Deserialize, Serialize};

use crate::user::MicroUser;

/// An API token for authentication.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserTokenInfo {
    pub version: Option<String>,
    pub user: Option<MicroUser>,
    pub token: Option<String>,
    pub note: Option<String>,
    pub enabled: Option<bool>,
    pub expiration_time: Option<Option<String>>,
    pub creation_time: Option<String>,
    pub last_edit_time: Option<String>,
    pub last_usage_time: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_user_token_info() {
        let json = r#"{
            "version": "2024-01-15T10:30:45Z",
            "user": {"name": "bob", "avatarUrl": "/avatars/bob.jpg"},
            "token": "550e8400-e29b-41d4-a716-446655440000",
            "note": "API token",
            "enabled": true,
            "expirationTime": "2024-12-31T23:59:59Z",
            "creationTime": "2024-01-01T00:00:00Z",
            "lastEditTime": "2024-01-15T10:30:45Z",
            "lastUsageTime": "2024-01-15T09:00:00Z"
        }"#;
        let token: UserTokenInfo = serde_json::from_str(json).unwrap();
        assert_eq!(token.enabled, Some(true));
        assert_eq!(token.note.as_deref(), Some("API token"));
        assert_eq!(token.user.unwrap().name, "bob");
    }

    #[test]
    fn deserialize_user_token_with_null_expiration() {
        let json = r#"{
            "token": "abc-123",
            "enabled": true,
            "expirationTime": null
        }"#;
        let token: UserTokenInfo = serde_json::from_str(json).unwrap();
        // serde doesn't distinguish missing from null for Option<Option<T>> by default
        assert!(token.expiration_time.is_none());
    }
}
