use crate::ai::formatting::{format_board, format_square};
use crate::ai::ui::{start_loading_animation, stop_loading_animation};
use crate::core::{
    board::Board, game_logic::get_all_valid_moves_for_player, move_history::MoveHistory,
    piece::Color as PieceColor,
};
use crate::utils::prompts::get_hint_prompt;
use genai::{
    chat::{ChatMessage, ChatOptions, ChatRequest},
    Client,
};
use std::env;

pub struct HintProvider {
    api_key: String,
    model: String,
}

impl HintProvider {
    pub fn new(api_key: String) -> Result<Self, String> {
        let model = env::var("GEMINI_MODEL")
            .map_err(|_| "GEMINI_MODEL environment variable is required")?;
        Ok(Self { api_key, model })
    }

    pub async fn get_hint(
        &self,
        board: &Board,
        current_player: PieceColor,
        history: &MoveHistory,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let board_state = format_board(board);
        let move_history = history.to_notation();

        // Get all valid moves for the current player
        let possible_moves = get_all_valid_moves_for_player(board, current_player);
        let mut moves_str = String::new();

        for ((from_row, from_col), (to_row, to_col), is_capture) in possible_moves.iter() {
            let from_sq = format_square(*from_row, *from_col);
            let to_sq = format_square(*to_row, *to_col);
            let move_type = if *is_capture { "capture" } else { "move" };
            moves_str.push_str(&format!("- {} to {} ({})\n", from_sq, to_sq, move_type));
        }

        let prompt_template = get_hint_prompt();
        let prompt = prompt_template
            .replace(
                "{player_color}",
                if current_player == PieceColor::White {
                    "White"
                } else {
                    "Black (Red pieces)"
                },
            )
            .replace("{board_state}", &board_state)
            .replace(
                "{move_history}",
                if move_history.is_empty() {
                    "No moves yet"
                } else {
                    &move_history
                },
            )
            .replace(
                "{available_moves}",
                if moves_str.is_empty() {
                    "No moves available"
                } else {
                    &moves_str
                },
            );

        // Set API key in environment for genai client
        env::set_var("GEMINI_API_KEY", &self.api_key);
        let client = Client::default();

        let chat_req = ChatRequest::new(vec![ChatMessage::user(prompt)]);

        let chat_options = ChatOptions::default()
            .with_temperature(0.7)
            .with_max_tokens(150);

        // Start loading animation
        let (running, loading_thread) =
            start_loading_animation().map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        let result = client
            .exec_chat(&self.model, chat_req, Some(&chat_options))
            .await;

        // Stop loading animation
        stop_loading_animation(running, loading_thread)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        let result = result.map_err(|e| {
            eprintln!("Hint API error: {:?}", e);
            Box::new(std::io::Error::other(format!("API request failed: {}", e)))
                as Box<dyn std::error::Error>
        })?;

        match result.content_text_as_str() {
            Some(text) => Ok(text.trim().to_string()),
            None => {
                eprintln!("Hint response has no text content");
                Err("Failed to parse hint from API response".into())
            }
        }
    }
}
