pub mod comments;
pub mod info;
pub mod password_reset;
pub mod pools;
pub mod posts;
pub mod snapshots;
pub mod tags;
pub mod user_tokens;
pub mod users;

use gloo_net::http::{RequestBuilder, Response};
use oxibooru_shared::pagination::ErrorResponse;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt;
use wasm_bindgen::prelude::*;

/// API error type.
#[derive(Debug, Clone)]
pub enum ApiError {
    /// Server returned an error response with structured body.
    Server(ErrorResponse),
    /// Network or deserialization error.
    Network(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Server(e) => write!(f, "{}: {}", e.title, e.description),
            ApiError::Network(msg) => write!(f, "Network error: {msg}"),
        }
    }
}

/// Credentials for API authentication.
#[derive(Debug, Clone, PartialEq)]
pub enum Credentials {
    Basic { username: String, password: String },
    Token { username: String, token: String },
}

impl Credentials {
    /// Build the Authorization header value.
    /// Server expects `Basic <base64(user:pass)>` or `Token <base64(user:token)>`.
    pub fn header_value(&self) -> String {
        match self {
            Credentials::Basic { username, password } => {
                let plain = format!("{username}:{password}");
                let encoded = base64_encode(&plain);
                format!("Basic {encoded}")
            }
            Credentials::Token { username, token } => {
                let plain = format!("{username}:{token}");
                let encoded = base64_encode(&plain);
                format!("Token {encoded}")
            }
        }
    }
}

/// Base64-encode a string using the browser's `btoa()`.
fn base64_encode(input: &str) -> String {
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = btoa, catch)]
        fn js_btoa(s: &str) -> Result<String, JsValue>;
    }
    js_btoa(input).unwrap_or_default()
}

/// API client for making requests to the oxibooru backend.
#[derive(Debug, Clone)]
pub struct ApiClient {
    base_url: String,
    credentials: Option<Credentials>,
}

impl ApiClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            credentials: None,
        }
    }

    pub fn set_credentials(&mut self, creds: Option<Credentials>) {
        self.credentials = creds;
    }

    pub fn has_credentials(&self) -> bool {
        self.credentials.is_some()
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    fn apply_auth(&self, builder: RequestBuilder) -> RequestBuilder {
        match &self.credentials {
            Some(creds) => builder.header("Authorization", &creds.header_value()),
            None => builder,
        }
    }

    async fn parse_response<T: DeserializeOwned>(resp: Response) -> Result<T, ApiError> {
        let status = resp.status();
        if (200..300).contains(&status) {
            resp.json::<T>().await.map_err(|e| ApiError::Network(e.to_string()))
        } else {
            match resp.json::<ErrorResponse>().await {
                Ok(err) => Err(ApiError::Server(err)),
                Err(e) => Err(ApiError::Network(format!("HTTP {status}: {e}"))),
            }
        }
    }

    /// GET request.
    pub async fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<T, ApiError> {
        let builder = self.apply_auth(
            gloo_net::http::Request::get(&self.url(path)),
        );
        let builder = if query.is_empty() {
            builder
        } else {
            builder.query(query.iter().copied())
        };
        let resp = builder
            .send()
            .await
            .map_err(|e| ApiError::Network(e.to_string()))?;
        Self::parse_response(resp).await
    }

    /// POST request with JSON body.
    pub async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, ApiError> {
        let resp = self
            .apply_auth(gloo_net::http::Request::post(&self.url(path)))
            .json(body)
            .map_err(|e| ApiError::Network(e.to_string()))?
            .send()
            .await
            .map_err(|e| ApiError::Network(e.to_string()))?;
        Self::parse_response(resp).await
    }

    /// PUT request with JSON body.
    pub async fn put<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, ApiError> {
        let resp = self
            .apply_auth(gloo_net::http::Request::put(&self.url(path)))
            .json(body)
            .map_err(|e| ApiError::Network(e.to_string()))?
            .send()
            .await
            .map_err(|e| ApiError::Network(e.to_string()))?;
        Self::parse_response(resp).await
    }

    /// DELETE request with JSON body.
    pub async fn delete<B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<(), ApiError> {
        let resp = self
            .apply_auth(gloo_net::http::Request::delete(&self.url(path)))
            .json(body)
            .map_err(|e| ApiError::Network(e.to_string()))?
            .send()
            .await
            .map_err(|e| ApiError::Network(e.to_string()))?;
        let status = resp.status();
        if (200..300).contains(&status) {
            Ok(())
        } else {
            match resp.json::<ErrorResponse>().await {
                Ok(err) => Err(ApiError::Server(err)),
                Err(e) => Err(ApiError::Network(format!("HTTP {status}: {e}"))),
            }
        }
    }
}
