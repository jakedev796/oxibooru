use super::{ApiClient, ApiError};
use oxibooru_shared::info::InfoResponse;

impl ApiClient {
    /// Fetch server info from `GET /info`.
    pub async fn get_info(&self) -> Result<InfoResponse, ApiError> {
        self.get("/info", &[]).await
    }

    /// Fetch server info and bump login timestamp.
    /// Used during login to verify credentials.
    pub async fn get_info_bump_login(&self) -> Result<InfoResponse, ApiError> {
        self.get("/info", &[("bump-login", "true")]).await
    }
}
