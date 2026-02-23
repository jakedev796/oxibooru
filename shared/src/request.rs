use serde::Serialize;

use crate::enums::Rating;

/// Request body for scoring a post or comment.
#[derive(Debug, Serialize)]
pub struct RatingBody {
    pub score: Rating,
}

/// Request body for deleting a resource (requires version for optimistic locking).
#[derive(Debug, Serialize)]
pub struct DeleteBody {
    pub version: String,
}

/// Request body for merging two resources.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MergeBody<T: Serialize> {
    pub remove: T,
    pub merge_to: T,
    pub remove_version: String,
    pub merge_to_version: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::enums::Rating;

    #[test]
    fn serialize_rating_body() {
        let body = RatingBody { score: Rating::Like };
        let json = serde_json::to_string(&body).unwrap();
        assert_eq!(json, r#"{"score":1}"#);
    }

    #[test]
    fn serialize_delete_body() {
        let body = DeleteBody { version: "2024-01-15T10:30:45Z".into() };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("2024-01-15T10:30:45Z"));
    }

    #[test]
    fn serialize_merge_body() {
        let body = MergeBody {
            remove: 1i64,
            merge_to: 2i64,
            remove_version: "v1".into(),
            merge_to_version: "v2".into(),
        };
        let json = serde_json::to_string(&body).unwrap();
        assert!(json.contains("\"mergeTo\":2"));
        assert!(json.contains("\"removeVersion\":\"v1\""));
    }
}
