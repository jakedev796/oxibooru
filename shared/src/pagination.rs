use serde::{Deserialize, Serialize};

/// Paginated API response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PagedResponse<T> {
    pub query: Option<String>,
    pub offset: i64,
    pub limit: i64,
    pub total: i64,
    pub results: Vec<T>,
}

/// Unpaginated API response (e.g., tag categories).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnpagedResponse<T> {
    pub results: Vec<T>,
}

/// Error response from the API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub name: String,
    pub title: String,
    pub description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_paged_response() {
        let json = r#"{
            "query": "tag:landscape",
            "offset": 0,
            "limit": 40,
            "total": 100,
            "results": [1, 2, 3]
        }"#;
        let page: PagedResponse<i64> = serde_json::from_str(json).unwrap();
        assert_eq!(page.offset, 0);
        assert_eq!(page.limit, 40);
        assert_eq!(page.total, 100);
        assert_eq!(page.results, vec![1, 2, 3]);
    }

    #[test]
    fn deserialize_error_response() {
        let json = r#"{
            "name": "PostNotFound",
            "title": "Resource Not Found",
            "description": "Post with ID 999 does not exist"
        }"#;
        let err: ErrorResponse = serde_json::from_str(json).unwrap();
        assert_eq!(err.name, "PostNotFound");
    }

    #[test]
    fn deserialize_paged_response_with_post_info() {
        use crate::post::PostInfo;
        let json = r#"{
            "query": "safety:safe",
            "offset": 0,
            "limit": 2,
            "total": 50,
            "results": [
                {"id": 1, "safety": "safe", "type": "image", "thumbnailUrl": "/thumb/1.jpg"},
                {"id": 2, "safety": "sketchy", "type": "video", "thumbnailUrl": "/thumb/2.jpg"}
            ]
        }"#;
        let page: PagedResponse<PostInfo> = serde_json::from_str(json).unwrap();
        assert_eq!(page.total, 50);
        assert_eq!(page.results.len(), 2);
        assert_eq!(page.results[0].id, Some(1));
        assert_eq!(page.results[1].id, Some(2));
    }

    #[test]
    fn deserialize_paged_response_with_tag_info() {
        use crate::tag::TagInfo;
        let json = r#"{
            "query": "",
            "offset": 0,
            "limit": 50,
            "total": 3,
            "results": [
                {"names": ["landscape"], "category": "default", "usages": 42},
                {"names": ["portrait"], "category": "default", "usages": 10},
                {"names": ["1girl"], "category": "character", "usages": 100}
            ]
        }"#;
        let page: PagedResponse<TagInfo> = serde_json::from_str(json).unwrap();
        assert_eq!(page.total, 3);
        assert_eq!(page.results.len(), 3);
        assert_eq!(page.results[2].category.as_deref(), Some("character"));
    }

    #[test]
    fn deserialize_unpaged_response() {
        use crate::tag::TagSibling;
        let json = r#"{
            "results": [
                {"tag": {"names": ["mountains"], "usages": 50}, "occurrences": 25},
                {"tag": {"names": ["sky"], "usages": 30}, "occurrences": 15}
            ]
        }"#;
        let resp: UnpagedResponse<TagSibling> = serde_json::from_str(json).unwrap();
        assert_eq!(resp.results.len(), 2);
        assert_eq!(resp.results[0].occurrences, 25);
        assert_eq!(resp.results[1].occurrences, 15);
    }

    #[test]
    fn deserialize_paged_response_null_query() {
        let json = r#"{"query": null, "offset": 10, "limit": 20, "total": 5, "results": []}"#;
        let page: PagedResponse<i64> = serde_json::from_str(json).unwrap();
        assert!(page.query.is_none());
        assert_eq!(page.offset, 10);
        assert!(page.results.is_empty());
    }
}
