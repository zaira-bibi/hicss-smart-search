use thiserror::Error;

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("Search API Error ({status_code}): {message}")]
    Api { status_code: u16, message: String },
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("JSON parsing failed: {0}")]
    Json(#[from] serde_json::Error),
}
