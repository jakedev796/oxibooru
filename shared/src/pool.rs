use serde::{Deserialize, Serialize};

use crate::post::MicroPost;

/// A pool resource stripped down to summary fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MicroPool {
    pub id: i64,
    pub names: Vec<String>,
    pub category: String,
    pub description: String,
    pub post_count: i64,
}

/// Full pool resource. All fields optional to support field selection.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolInfo {
    pub version: Option<String>,
    pub id: Option<i64>,
    pub description: Option<String>,
    pub creation_time: Option<String>,
    pub last_edit_time: Option<String>,
    pub category: Option<String>,
    pub names: Option<Vec<String>>,
    pub posts: Option<Vec<MicroPost>>,
    pub post_count: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_micro_pool() {
        let json = r#"{
            "id": 1,
            "names": ["primes", "prime_numbers"],
            "category": "mathematical",
            "description": "Prime number collection",
            "postCount": 7
        }"#;
        let pool: MicroPool = serde_json::from_str(json).unwrap();
        assert_eq!(pool.id, 1);
        assert_eq!(pool.names, vec!["primes", "prime_numbers"]);
        assert_eq!(pool.category, "mathematical");
        assert_eq!(pool.post_count, 7);
    }

    #[test]
    fn deserialize_pool_info() {
        let json = r#"{
            "version": "2024-01-15T10:30:45Z",
            "id": 1,
            "description": "A collection",
            "creationTime": "2024-01-01T00:00:00Z",
            "lastEditTime": "2024-01-15T10:30:45Z",
            "category": "series",
            "names": ["test_pool"],
            "posts": [{"id": 101, "thumbnailUrl": "/thumb/101.jpg"}],
            "postCount": 1
        }"#;
        let pool: PoolInfo = serde_json::from_str(json).unwrap();
        assert_eq!(pool.id, Some(1));
        assert_eq!(pool.category.as_deref(), Some("series"));
        assert_eq!(pool.posts.as_ref().unwrap().len(), 1);
        assert_eq!(pool.post_count, Some(1));
    }
}
