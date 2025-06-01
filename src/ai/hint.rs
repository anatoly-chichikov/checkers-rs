use crate::core::{board::Board, move_history::MoveHistory, piece::Color as PieceColor};
use genai::{
    chat::{ChatMessage, ChatOptions, ChatRequest},
    Client,
};
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

        // Set API key in environment for genai client
        env::set_var("GEMINI_API_KEY", &self.api_key);
        let client = Client::default();

        let chat_req = ChatRequest::new(vec![ChatMessage::user(prompt)]);

        let chat_options = ChatOptions::default()
            .with_temperature(0.7)
            .with_max_tokens(150);

        let result = client
            .exec_chat(&self.model, chat_req, Some(&chat_options))
            .await
            .map_err(|e| {
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
