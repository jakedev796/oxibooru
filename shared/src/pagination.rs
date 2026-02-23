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
}
