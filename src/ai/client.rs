use reqwest::Client;
use std::env;

use crate::ai::error::AIError;
use crate::ai::models::{Content, Message, NebiusRequest, NebiusResponse};
use crate::ai::ui::{start_loading_animation, stop_loading_animation};
use crate::interface::messages;

pub async fn explain_rules() -> Result<String, AIError> {
    dotenv::dotenv().ok();

    let api_key = env::var("NEBIUS_API_KEY").map_err(|_| AIError::NoApiKey)?;

    // Start loading animation
    let (running, loading_thread) = start_loading_animation()?;

    let client = Client::new();
    let request = NebiusRequest {
        model: "meta-llama/Llama-3.2-1B-Instruct".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: vec![Content {
                content_type: "text".to_string(),
                text: messages::STORY_PROMPT.to_string(),
            }],
        }],
        max_tokens: 512,
        temperature: 0.0,
        top_p: 0.9,
        top_k: 50,
    };

    let response = client
        .post("https://api.studio.nebius.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Accept", "*/*")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await
        .map_err(|e| AIError::RequestFailed(e.to_string()))?;

    // Stop the loading animation and cleanup
    stop_loading_animation(running, loading_thread)?;

    let response_data: NebiusResponse = response
        .json()
        .await
        .map_err(|e| AIError::ParseError(e.to_string()))?;

    Ok(response_data.choices[0].message.content.clone())
} 