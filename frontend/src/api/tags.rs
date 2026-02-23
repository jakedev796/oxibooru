use oxibooru_shared::pagination::{PagedResponse, UnpagedResponse};
use oxibooru_shared::tag::{TagInfo, TagSibling};

use super::{ApiClient, ApiError};

impl ApiClient {
    pub async fn get_tags(
        &self,
        query: &str,
        offset: i64,
        limit: i64,
        fields: &str,
    ) -> Result<PagedResponse<TagInfo>, ApiError> {
        self.get(
            "/tags",
            &[
                ("query", query),
                ("offset", &offset.to_string()),
                ("limit", &limit.to_string()),
                ("fields", fields),
            ],
        )
        .await
    }

    pub async fn get_tag(&self, name: &str) -> Result<TagInfo, ApiError> {
        self.get(&format!("/tag/{}", url_encode(name)), &[]).await
    }

    pub async fn get_tag_siblings(
        &self,
        name: &str,
    ) -> Result<UnpagedResponse<TagSibling>, ApiError> {
        self.get(&format!("/tag-siblings/{}", url_encode(name)), &[])
            .await
    }
}

fn url_encode(s: &str) -> String {
    js_sys::encode_uri_component(s).into()
}
