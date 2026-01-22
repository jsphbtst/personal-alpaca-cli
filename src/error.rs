use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
  #[error("HTTP request failed: {0}")]
  Http(#[from] reqwest::Error),

  #[error("JSON parsing failed: {0}")]
  Json(#[from] serde_json::Error),

  #[error("File I/O failed: {0}")]
  Io(#[from] std::io::Error),

  #[error("Configuration error: {0}")]
  Config(String),

  #[error("API error: {0}")]
  Api(String),

  #[error("Missing required argument: {0}")]
  MissingArgument(String),
}

pub type AppResult<T> = Result<T, AppError>;
