use serde::Deserialize;

/// Top-level error type for the Trello client library.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error {status}: {message}")]
    Api {
        status: reqwest::StatusCode,
        message: String,
    },

    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("failed to deserialize API response: {error}. Body: {body_preview}")]
    SerdeBody {
        error: serde_json::Error,
        body_preview: String,
    },

    #[error("URL parse error: {0}")]
    Url(#[from] url::ParseError),

    #[error("{0}")]
    Other(String),
}

/// Result type alias for convenience.
pub type Result<T> = std::result::Result<T, Error>;

/// Raw API error body from Trello.
#[derive(Debug, Deserialize)]
pub(crate) struct ApiErrorBody {
    #[allow(dead_code)]
    pub(crate) code: Option<u16>,
    pub(crate) message: Option<String>,
    #[serde(default)]
    pub(crate) error: Option<String>,
}

/// HTTP status code re-export.
pub use reqwest::StatusCode;
