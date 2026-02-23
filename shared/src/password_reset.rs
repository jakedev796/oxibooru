use serde::{Deserialize, Serialize};

/// Response from `POST /password-reset/{identifier}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordResetResponse {
    pub password: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_password_reset_response() {
        let json = r#"{"password": "temp_abc123XYZ"}"#;
        let resp: PasswordResetResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.password, "temp_abc123XYZ");
    }
}
