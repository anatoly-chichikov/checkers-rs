use crate::ai::AIError;
use std::env;

#[derive(Clone)]
pub struct AIConfig {
    pub api_key: String,
    pub model: String,
}

impl AIConfig {
    /// Creates config from provided values without side effects.
    pub fn new(api_key: String, model: String) -> Self {
        Self { api_key, model }
    }
}

/// Loads AI configuration from environment.
pub fn load() -> Result<AIConfig, AIError> {
    let api_key = env::var("GEMINI_API_KEY").map_err(|_| AIError::NoApiKey)?;
    let model = env::var("GEMINI_MODEL").map_err(|_| AIError::NoModel)?;
    Ok(AIConfig::new(api_key, model))
}
