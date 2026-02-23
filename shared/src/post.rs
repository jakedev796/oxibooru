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

/// Response from `GET /post/{id}/around` — neighboring posts for prev/next navigation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostNeighbors {
    pub prev: Option<PostInfo>,
    pub next: Option<PostInfo>,
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

    #[test]
    fn deserialize_post_info_full() {
        let json = r##"{
            "version": "2024-01-15T10:30:45Z",
            "id": 42,
            "user": {"name": "alice", "avatarUrl": "/avatars/alice.jpg"},
            "fileSize": 1048576,
            "canvasWidth": 1920,
            "canvasHeight": 1080,
            "safety": "sketchy",
            "type": "video",
            "mimeType": "video/mp4",
            "checksum": "abc123",
            "checksumMD5": "def456",
            "flags": ["loop"],
            "source": "https://example.com",
            "description": "A test post",
            "creationTime": "2024-01-10T08:15:30Z",
            "lastEditTime": "2024-01-15T10:30:45Z",
            "contentUrl": "/data/posts/42/content.mp4",
            "thumbnailUrl": "/data/posts/42/thumb.jpg",
            "tags": [{"names": ["landscape"], "category": "default", "usages": 10}],
            "comments": [],
            "relations": [{"id": 43, "thumbnailUrl": "/data/posts/43/thumb.jpg"}],
            "pools": [{"id": 1, "names": ["nature"], "category": "series", "description": "", "postCount": 5}],
            "notes": [],
            "score": 15,
            "ownScore": 1,
            "ownFavorite": true,
            "tagCount": 1,
            "commentCount": 0,
            "relationCount": 1,
            "noteCount": 0,
            "favoriteCount": 3,
            "featureCount": 1,
            "lastFeatureTime": "2024-01-12T00:00:00Z",
            "favoritedBy": [{"name": "bob", "avatarUrl": "/avatars/bob.jpg"}],
            "hasCustomThumbnail": false
        }"##;
        let post: PostInfo = serde_json::from_str(json).unwrap();
        assert_eq!(post.id, Some(42));
        assert_eq!(post.safety, Some(PostSafety::Sketchy));
        assert_eq!(post.type_, Some(PostType::Video));
        assert_eq!(post.file_size, Some(1048576));
        assert_eq!(post.canvas_width, Some(1920));
        assert_eq!(post.canvas_height, Some(1080));
        assert_eq!(post.score, Some(15));
        assert_eq!(post.favorite_count, Some(3));
        assert_eq!(post.tags.as_ref().unwrap().len(), 1);
        assert_eq!(post.relations.as_ref().unwrap().len(), 1);
        assert_eq!(post.pools.as_ref().unwrap().len(), 1);
        assert_eq!(post.favorited_by.as_ref().unwrap().len(), 1);
        assert_eq!(post.user.unwrap().unwrap().name, "alice");
        assert_eq!(post.source.as_deref(), Some("https://example.com"));
        assert_eq!(post.description.as_deref(), Some("A test post"));
        assert_eq!(post.content_url.as_deref(), Some("/data/posts/42/content.mp4"));
        assert_eq!(post.has_custom_thumbnail, Some(false));
    }

    #[test]
    fn deserialize_post_neighbors() {
        let json = r#"{
            "prev": {"id": 41, "thumbnailUrl": "/data/posts/41/thumb.jpg"},
            "next": {"id": 43, "thumbnailUrl": "/data/posts/43/thumb.jpg"}
        }"#;
        let neighbors: PostNeighbors = serde_json::from_str(json).unwrap();
        assert_eq!(neighbors.prev.as_ref().unwrap().id, Some(41));
        assert_eq!(neighbors.next.as_ref().unwrap().id, Some(43));
    }

    #[test]
    fn deserialize_post_neighbors_no_prev() {
        let json = r#"{"prev": null, "next": {"id": 2}}"#;
        let neighbors: PostNeighbors = serde_json::from_str(json).unwrap();
        assert!(neighbors.prev.is_none());
        assert_eq!(neighbors.next.as_ref().unwrap().id, Some(2));
    }
}
