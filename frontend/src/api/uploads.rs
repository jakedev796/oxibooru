use oxibooru_shared::upload::UploadResponse;

use super::{ApiClient, ApiError};

impl ApiClient {
    /// POST /uploads (multipart with content file).
    pub async fn upload_file(&self, form_data: &web_sys::FormData) -> Result<UploadResponse, ApiError> {
        self.post_multipart("/uploads", form_data).await
    }
}
