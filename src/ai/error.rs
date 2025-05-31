use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AIError {
    #[error("API request failed: {0}")]
    RequestFailed(String),
    #[error("Failed to parse API response: {0}")]
    ParseError(String),
    #[error("API key not found - add GEMINI_API_KEY to your .env file to enable AI features")]
    NoApiKey,
    #[error("AI response format is invalid: {0}")]
    InvalidResponseFormat(String),
    #[error("No possible moves available for the AI.")]
    NoPossibleMoves,
    #[error("IO error: {0}")]
    IOError(#[from] io::Error),
}
