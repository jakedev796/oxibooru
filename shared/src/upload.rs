use serde::{Deserialize, Serialize};

/// Response from `POST /uploads`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadResponse {
    pub token: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_upload_response() {
        let json = r#"{"token": "550e8400-e29b-41d4-a716-446655440000"}"#;
        let resp: UploadResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.token, "550e8400-e29b-41d4-a716-446655440000");
    }
}
