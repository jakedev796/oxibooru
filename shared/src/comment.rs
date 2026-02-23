use serde::{Deserialize, Serialize};

use crate::enums::Rating;
use crate::user::MicroUser;

/// Full comment resource. All fields optional to support field selection.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentInfo {
    pub version: Option<String>,
    pub id: Option<i64>,
    pub post_id: Option<i64>,
    pub text: Option<String>,
    pub creation_time: Option<String>,
    pub last_edit_time: Option<String>,
    pub user: Option<Option<MicroUser>>,
    pub score: Option<i64>,
    pub own_score: Option<Rating>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_comment_info() {
        let json = r#"{
            "version": "2024-01-15T10:30:45Z",
            "id": 42,
            "postId": 101,
            "text": "Great post!",
            "creationTime": "2024-01-10T08:15:30Z",
            "lastEditTime": "2024-01-15T10:30:45Z",
            "user": {"name": "alice", "avatarUrl": "/avatars/alice.jpg"},
            "score": 5,
            "ownScore": 1
        }"#;
        let comment: CommentInfo = serde_json::from_str(json).unwrap();
        assert_eq!(comment.id, Some(42));
        assert_eq!(comment.post_id, Some(101));
        assert_eq!(comment.text.as_deref(), Some("Great post!"));
        assert_eq!(comment.score, Some(5));
        assert_eq!(comment.own_score, Some(Rating::Like));
        let user = comment.user.unwrap().unwrap();
        assert_eq!(user.name, "alice");
    }

    #[test]
    fn deserialize_comment_with_null_user() {
        let json = r#"{
            "id": 1,
            "user": null,
            "score": 0
        }"#;
        let comment: CommentInfo = serde_json::from_str(json).unwrap();
        assert!(comment.user.is_none());
    }
}
