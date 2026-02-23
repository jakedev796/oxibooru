use serde::{Deserialize, Serialize};

use crate::comment::CommentInfo;
use crate::enums::{MimeType, PostSafety, PostType, Rating};
use crate::pool::MicroPool;
use crate::tag::MicroTag;
use crate::user::MicroUser;

/// A post annotation (text overlay on an image region).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    /// Polygon vertices as [[x, y], ...] with coordinates in 0..1 range.
    pub polygon: Vec<[f32; 2]>,
    /// Annotation text (rendered as markdown).
    pub text: String,
}

/// A post resource stripped down to `id` and `thumbnailUrl`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MicroPost {
    pub id: i64,
    pub thumbnail_url: String,
}

/// Full post resource. All fields are optional to support field selection via `?fields=`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostInfo {
    pub version: Option<String>,
    pub id: Option<i64>,
    pub user: Option<Option<MicroUser>>,
    pub file_size: Option<i64>,
    pub canvas_width: Option<i32>,
    pub canvas_height: Option<i32>,
    pub safety: Option<PostSafety>,
    #[serde(rename = "type")]
    pub type_: Option<PostType>,
    pub mime_type: Option<MimeType>,
    pub checksum: Option<String>,
    #[serde(rename = "checksumMD5")]
    pub checksum_md5: Option<String>,
    /// Post flags such as "loop" and "sound". Serialized as `["loop", "sound"]`.
    pub flags: Option<Vec<String>>,
    pub source: Option<String>,
    pub description: Option<String>,
    pub creation_time: Option<String>,
    pub last_edit_time: Option<String>,
    pub content_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub tags: Option<Vec<MicroTag>>,
    pub comments: Option<Vec<CommentInfo>>,
    pub relations: Option<Vec<MicroPost>>,
    pub pools: Option<Vec<MicroPool>>,
    pub notes: Option<Vec<Note>>,
    pub score: Option<i64>,
    pub own_score: Option<Rating>,
    pub own_favorite: Option<bool>,
    pub tag_count: Option<i64>,
    pub comment_count: Option<i64>,
    pub relation_count: Option<i64>,
    pub note_count: Option<i64>,
    pub favorite_count: Option<i64>,
    pub feature_count: Option<i64>,
    pub last_feature_time: Option<Option<String>>,
    pub favorited_by: Option<Vec<MicroUser>>,
    pub has_custom_thumbnail: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_micro_post() {
        let json = r#"{"id": 42, "thumbnailUrl": "/data/posts/42/thumb.jpg"}"#;
        let post: MicroPost = serde_json::from_str(json).unwrap();
        assert_eq!(post.id, 42);
        assert_eq!(post.thumbnail_url, "/data/posts/42/thumb.jpg");
    }

    #[test]
    fn deserialize_post_info_partial_fields() {
        let json = r#"{"id": 1, "safety": "safe", "type": "image", "tagCount": 5}"#;
        let post: PostInfo = serde_json::from_str(json).unwrap();
        assert_eq!(post.id, Some(1));
        assert_eq!(post.safety, Some(PostSafety::Safe));
        assert_eq!(post.type_, Some(PostType::Image));
        assert_eq!(post.tag_count, Some(5));
        assert!(post.tags.is_none());
    }

    #[test]
    fn deserialize_post_info_with_flags() {
        let json = r#"{"id": 1, "flags": ["loop", "sound"]}"#;
        let post: PostInfo = serde_json::from_str(json).unwrap();
        assert_eq!(post.flags, Some(vec!["loop".to_string(), "sound".to_string()]));
    }

    #[test]
    fn deserialize_note() {
        let json = r#"{"polygon": [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]], "text": "Hello"}"#;
        let note: Note = serde_json::from_str(json).unwrap();
        assert_eq!(note.polygon.len(), 4);
        assert_eq!(note.text, "Hello");
    }
}
