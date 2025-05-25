use serde::{Deserialize, Serialize};

// New structs for Gemini API communication
#[derive(Debug, Serialize)]
pub struct GeminiRequest {
    pub contents: Vec<Content>,
    #[serde(rename = "generationConfig")]
    pub generation_config: GenerationConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub parts: Vec<Part>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Part {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct GenerationConfig {
    pub temperature: f32,
    #[serde(rename = "maxOutputTokens")]
    pub max_output_tokens: u32,
}

#[derive(Debug, Deserialize)]
pub struct GeminiResponse {
    pub candidates: Vec<Candidate>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Candidate {
    pub content: Content,
    // pub finish_reason: String, // This line is removed
} 