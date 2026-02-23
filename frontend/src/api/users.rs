use oxibooru_shared::pagination::PagedResponse;
use oxibooru_shared::user::UserInfo;

use super::{ApiClient, ApiError};

impl ApiClient {
    pub async fn get_users(
        &self,
        query: &str,
        offset: i64,
        limit: i64,
    ) -> Result<PagedResponse<UserInfo>, ApiError> {
        self.get(
            "/users",
            &[
                ("query", query),
                ("offset", &offset.to_string()),
                ("limit", &limit.to_string()),
            ],
        )
        .await
    }

    pub async fn get_user(&self, name: &str) -> Result<UserInfo, ApiError> {
        self.get(&format!("/user/{}", url_encode(name)), &[]).await
    }
}

fn url_encode(s: &str) -> String {
    js_sys::encode_uri_component(s).into()
}
