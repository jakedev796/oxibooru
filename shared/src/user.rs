use serde::{Deserialize, Serialize};

use crate::enums::{AvatarStyle, UserRank};

/// A user resource stripped down to name and avatar URL.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MicroUser {
    pub name: String,
    pub avatar_url: String,
}

/// A field that may be hidden based on viewer permissions.
/// When visible, contains the actual value. When hidden, contains `false`.
/// Uses `#[serde(untagged)]` to match the server's serialization.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrivateData<T> {
    Value(T),
    Hidden(bool),
}

impl<T> PrivateData<T> {
    /// Returns the inner value if exposed, or `None` if hidden.
    pub fn value(&self) -> Option<&T> {
        match self {
            PrivateData::Value(v) => Some(v),
            PrivateData::Hidden(_) => None,
        }
    }
}

/// Full user resource. All fields optional to support field selection.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    pub version: Option<String>,
    pub name: Option<String>,
    pub email: Option<PrivateData<Option<String>>>,
    pub rank: Option<UserRank>,
    pub last_login_time: Option<String>,
    pub creation_time: Option<String>,
    pub avatar_style: Option<AvatarStyle>,
    pub avatar_url: Option<String>,
    pub comment_count: Option<i64>,
    pub uploaded_post_count: Option<i64>,
    pub liked_post_count: Option<PrivateData<i64>>,
    pub disliked_post_count: Option<PrivateData<i64>>,
    pub favorite_post_count: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_private_data_exposed() {
        let json = r#""user@example.com""#;
        let data: PrivateData<String> = serde_json::from_str(json).unwrap();
        assert_eq!(data.value(), Some(&"user@example.com".to_string()));
    }

    #[test]
    fn deserialize_private_data_hidden() {
        let json = "false";
        let data: PrivateData<String> = serde_json::from_str(json).unwrap();
        assert!(data.value().is_none());
    }

    #[test]
    fn deserialize_user_info_with_private_fields() {
        let json = r#"{
            "name": "alice",
            "rank": "regular",
            "email": "alice@example.com",
            "likedPostCount": false,
            "dislikedPostCount": false
        }"#;
        let user: UserInfo = serde_json::from_str(json).unwrap();
        assert_eq!(user.name, Some("alice".to_string()));
        assert_eq!(user.rank, Some(UserRank::Regular));
        assert!(user.liked_post_count.as_ref().unwrap().value().is_none());
    }
}
