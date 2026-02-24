use oxibooru_shared::password_reset::PasswordResetResponse;
use serde::Serialize;

use super::{ApiClient, ApiError};

#[derive(Debug, Serialize)]
pub struct PasswordResetTokenBody {
    pub token: String,
}

impl ApiClient {
    /// Request a password reset email. `GET /password-reset/{identifier}`.
    pub async fn request_password_reset(&self, identifier: &str) -> Result<(), ApiError> {
        let _: serde_json::Value = self
            .get(&format!("/password-reset/{}", url_encode(identifier)), &[])
            .await?;
        Ok(())
    }

    /// Confirm a password reset with the token from the email.
    /// `POST /password-reset/{identifier}`.
    pub async fn confirm_password_reset(
        &self,
        identifier: &str,
        body: &PasswordResetTokenBody,
    ) -> Result<PasswordResetResponse, ApiError> {
        self.post(&format!("/password-reset/{}", url_encode(identifier)), body)
            .await
    }
}

fn url_encode(s: &str) -> String {
    js_sys::encode_uri_component(s).into()
}
