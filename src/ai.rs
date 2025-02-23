use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    env, thread,
    time::Duration,
    io::{self, Write},
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
};
use thiserror::Error;
use crossterm::{
    cursor::{Hide, Show},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};

struct LoadingAnimation {
    frames: Vec<&'static str>,
    current: usize,
}

impl LoadingAnimation {
    fn new() -> Self {
        Self {
            frames: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            current: 0,
        }
    }

    fn next(&mut self) -> &str {
        let frame = self.frames[self.current];
        self.current = (self.current + 1) % self.frames.len();
        frame
    }
}

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

#[derive(Serialize, Debug)]
struct NebiusRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
    temperature: f32,
    top_p: f32,
    top_k: u32,
}

#[derive(Serialize, Debug)]
struct Message {
    role: String,
    content: Vec<Content>,
}

#[derive(Serialize, Debug)]
struct Content {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

#[derive(Deserialize, Debug)]
struct NebiusResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize, Debug)]
struct ResponseMessage {
    content: String,
}

pub async fn explain_rules() -> Result<String, AIError> {
    dotenv::dotenv().ok();
    
    let api_key = env::var("NEBIUS_API_KEY")
        .map_err(|_| AIError::NoApiKey)?;

    let mut stdout = io::stdout();
    stdout.execute(Hide)?;
    
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    
    let mut loading = LoadingAnimation::new();
    let loading_thread = thread::spawn(move || {
        while running_clone.load(Ordering::Relaxed) {
            print!("\r{} Waiting for AI response...", loading.next());
            if let Err(_) = io::stdout().flush() {
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }
    });

    let client = Client::new();
    let request = NebiusRequest {
        model: "meta-llama/Meta-Llama-3.1-8B-Instruct".to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: vec![Content {
                content_type: "text".to_string(),
                text: "Please explain the rules of checkers (draughts) in a clear, concise way that a beginner can understand. Include basic movement, capturing, and winning conditions.".to_string(),
            }],
        }],
        max_tokens: 512,
        temperature: 0.6,
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
    running.store(false, Ordering::Relaxed);
    let _ = loading_thread.join();
    
    let mut stdout = io::stdout();
    stdout.execute(Clear(ClearType::CurrentLine))?;
    stdout.execute(Show)?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await
            .map_err(|e| AIError::RequestFailed(e.to_string()))?;
        return Err(AIError::RequestFailed(format!(
            "API returned error status: {} with body: {}",
            status, error_text
        )));
    }

    let response_text = response
        .text()
        .await
        .map_err(|e| AIError::RequestFailed(e.to_string()))?;

    let response: NebiusResponse = serde_json::from_str(&response_text)
        .map_err(|e| AIError::ParseError(e.to_string()))?;

    Ok(response.choices[0].message.content.clone())
} 