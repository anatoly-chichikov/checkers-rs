use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AIError {
    #[error("API request failed: {0}")]
    RequestFailed(String),
    #[error("Failed to parse API response: {0}")]
    ParseError(String),
    #[error("API key not found - add NEBIUS_API_KEY to your .env file to enable AI features")]
    NoApiKey,
    #[error("IO error: {0}")]
    IOError(#[from] io::Error),
} 