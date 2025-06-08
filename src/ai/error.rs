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
    #[error("Model not specified - add GEMINI_MODEL to your .env file")]
    NoModel,
    #[error("AI response format is invalid: {0}")]
    #[allow(dead_code)]
    InvalidResponseFormat(String),
    #[error("No possible moves available for the AI.")]
    #[allow(dead_code)]
    NoPossibleMoves,
    #[error("IO error: {0}")]
    IOError(#[from] io::Error),
}
