use crate::core::{board::Board, move_history::MoveHistory, piece::Color as PieceColor};
use reqwest::Client;
use serde_json::json;
use std::env;

fn format_board(board: &Board) -> String {
    let mut board_str = String::new();
    board_str.push_str("  A B C D E F G H\n");
    for r in 0..board.size {
        board_str.push_str(&format!("{} ", r + 1));
        for c in 0..board.size {
            let piece_str: String = match board.get_piece(r, c) {
                Some(piece) => piece.display(),
                None => ".".to_string(),
            };
            board_str.push_str(&piece_str);
            board_str.push(' ');
        }
        board_str.push('\n');
    }
    board_str
}

pub struct HintProvider {
    client: Client,
    url: String,
}

impl HintProvider {
    pub fn new(api_key: String) -> Result<Self, String> {
        let model = env::var("GEMINI_MODEL")
            .map_err(|_| "GEMINI_MODEL environment variable is required")?;
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            model, api_key
        );
        Ok(Self {
            client: Client::new(),
            url,
        })
    }

    pub async fn get_hint(
        &self,
        board: &Board,
        current_player: PieceColor,
        history: &MoveHistory,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let board_state = format_board(board);
        let move_history = history.to_notation();

        let prompt = format!(
            "You are a checkers expert providing hints to a {} player. Analyze the board and suggest the best move with a brief explanation (2-3 sentences max).

Current board state:
{}

Move history: {}

Provide a hint in this format:
1. Suggest the best move (e.g., 'Move piece from d6 to e5')
2. Briefly explain why this is a good move (e.g., 'This creates a double jump opportunity' or 'This blocks your opponent's king')

Be concise and educational. Focus on strategy, not just the immediate move.",
            if current_player == PieceColor::White { "White" } else { "Black (Red pieces)" },
            board_state,
            if move_history.is_empty() { "No moves yet" } else { &move_history }
        );

        let request_body = json!({
            "contents": [{
                "parts": [{
                    "text": prompt
                }]
            }],
            "generationConfig": {
                "temperature": 0.7,
                "maxOutputTokens": 150,
            }
        });

        let response = self
            .client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(format!("API request failed: {}", response.status()).into());
        }

        let response_json: serde_json::Value = response.json().await?;

        if let Some(text) = response_json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
            Ok(text.trim().to_string())
        } else {
            Err("Failed to parse hint from API response".into())
        }
    }
}
