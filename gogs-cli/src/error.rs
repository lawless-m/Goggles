use thiserror::Error;

#[derive(Debug, Error)]
pub enum GogsError {
    #[error("API error: {0}")]
    ApiError(String),

    #[error("Authentication failed: {0}")]
    AuthError(String),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}

impl GogsError {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::NotFound(_) => 2,
            _ => 1,
        }
    }
}
