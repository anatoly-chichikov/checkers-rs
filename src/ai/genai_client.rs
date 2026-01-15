use crate::ai::config::AIConfig;
use crate::ai::contract::{BoxFuture, Choice, HintGiver, MoveChooser, Moves, RulesExplainer};
use crate::ai::error::AIError;
use crate::ai::formatting::{board, square};
use crate::ai::ui::{animate_start, animate_start_with_message, animate_stop};
use crate::core::board::{Board, Grid};
use crate::core::game::{CheckersGame, Game};
use crate::core::move_history::{History, MoveHistory};
use crate::core::piece::Color as PieceColor;
use crate::interface::messages;
use crate::utils::prompts::{get_ai_move_prompt, get_hint_prompt};
use genai::chat::{ChatMessage, ChatOptions, ChatRequest};
use genai::Client;
use std::env;

pub struct GeminiAI {
    config: AIConfig,
}

impl GeminiAI {
    /// Builds AI adapter with provided configuration.
    pub fn new(config: AIConfig) -> Self {
        Self { config }
    }

    /// Produces rules description text.
    async fn story(&self) -> Result<String, AIError> {
        dotenv::dotenv().ok();
        let (running, loading_thread) = animate_start_with_message("Waiting for the magic...")?;
        let client = self.client();
        let chat_req = ChatRequest::new(vec![ChatMessage::user(messages::STORY_PROMPT)]);
        let chat_options = ChatOptions::default()
            .with_temperature(0.7)
            .with_max_tokens(512);
        let result = client
            .exec_chat(&self.config.model, chat_req, Some(&chat_options))
            .await;
        animate_stop(running, loading_thread)?;
        let result = result.map_err(|e| AIError::RequestFailed(e.to_string()))?;
        match result.content_text_as_str() {
            Some(text) => Ok(Self::strip(text)),
            None => Err(AIError::ParseError(
                "No text content in response".to_string(),
            )),
        }
    }

    /// Selects a move for the current black player.
    async fn select(&self, game: &CheckersGame) -> Result<Choice, AIError> {
        dotenv::dotenv().ok();
        if game.turn() != PieceColor::Black {
            return Err(AIError::InvalidResponseFormat(
                "AI can only play as black".to_string(),
            ));
        }
        let possible_moves = game.board().choices(game.turn());
        if possible_moves.is_empty() {
            return Err(AIError::NoPossibleMoves);
        }
        let prompt = Self::movelist(&possible_moves, game.board());
        let client = self.client();
        let chat_req = ChatRequest::new(vec![ChatMessage::user(prompt)]);
        let chat_options = ChatOptions::default()
            .with_temperature(0.1)
            .with_max_tokens(5);
        let (running, loading_thread) = animate_start()?;
        let raw_result = client
            .exec_chat(&self.config.model, chat_req, Some(&chat_options))
            .await;
        animate_stop(running, loading_thread)?;
        let result = raw_result.map_err(|e| AIError::RequestFailed(e.to_string()))?;
        match result.content_text_as_str() {
            Some(text_response) => {
                let index = Self::parse(text_response, possible_moves.len())?;
                let chosen_move = &possible_moves[index - 1];
                Ok((
                    (chosen_move.origin.row, chosen_move.origin.col),
                    (chosen_move.target.row, chosen_move.target.col),
                ))
            }
            None => Err(AIError::ParseError(
                "No text content in response".to_string(),
            )),
        }
    }

    /// Generates a hint for the given player and board.
    async fn advise(
        &self,
        board_state: &Board,
        player: PieceColor,
        history: &MoveHistory,
    ) -> Result<String, AIError> {
        dotenv::dotenv().ok();
        let prompt = Self::hintprompt(board_state, player, history);
        let client = self.client();
        let chat_req = ChatRequest::new(vec![ChatMessage::user(prompt)]);
        let chat_options = ChatOptions::default()
            .with_temperature(0.7)
            .with_max_tokens(150);
        let (running, loading_thread) = animate_start()?;
        let raw_result = client
            .exec_chat(&self.config.model, chat_req, Some(&chat_options))
            .await;
        animate_stop(running, loading_thread)?;
        let result = raw_result.map_err(|e| AIError::RequestFailed(e.to_string()))?;
        match result.content_text_as_str() {
            Some(text) => Ok(text.trim().to_string()),
            None => Err(AIError::ParseError(
                "No text content in response".to_string(),
            )),
        }
    }

    /// Creates a Gemini client with env key set.
    fn client(&self) -> Client {
        env::set_var("GEMINI_API_KEY", self.config.api_key.clone());
        Client::default()
    }

    /// Cleans HTML markers from response.
    fn strip(text: &str) -> String {
        text.replace("<br>", "\n")
            .replace("<br/>", "\n")
            .replace("<br />", "\n")
    }

    /// Renders prompt for AI move selection.
    fn movelist(possible_moves: &Moves, board_state: &Board) -> String {
        let board_representation = board(board_state);
        let mut moves_str = String::new();
        for (i, play) in possible_moves.iter().enumerate() {
            let from_sq = square(play.origin.row, play.origin.col);
            let to_sq = square(play.target.row, play.target.col);
            let mut move_desc = format!("{}. {} to {}", i + 1, from_sq, to_sq);
            let span = (play.target.row as i32 - play.origin.row as i32).abs();
            if span == 2 {
                let mid_row = (play.origin.row + play.target.row) / 2;
                let mid_col = (play.origin.col + play.target.col) / 2;
                let captured_sq = square(mid_row, mid_col);
                move_desc.push_str(&format!(" (captures piece at {captured_sq})"));
            }
            moves_str.push_str(&move_desc);
            moves_str.push('\n');
        }
        let template = get_ai_move_prompt();
        template
            .replace("{board_state}", &board_representation)
            .replace("{available_moves}", moves_str.trim())
    }

    /// Renders prompt for hint generation.
    fn hintprompt(board_state: &Board, player: PieceColor, history: &MoveHistory) -> String {
        let board_text = board(board_state);
        let move_history = history.notation();
        let possible_moves = board_state.choices(player);
        let mut moves_str = String::new();
        for play in possible_moves.iter() {
            let from_sq = square(play.origin.row, play.origin.col);
            let to_sq = square(play.target.row, play.target.col);
            let span = (play.target.row as i32 - play.origin.row as i32).abs();
            let move_type = if span == 2 { "capture" } else { "move" };
            moves_str.push_str(&format!("- {from_sq} to {to_sq} ({move_type})\n"));
        }
        let template = get_hint_prompt();
        let player_text = if player == PieceColor::White {
            "White"
        } else {
            "Black"
        };
        let history_text = if move_history.is_empty() {
            "No moves yet".to_string()
        } else {
            move_history
        };
        let moves_text = if moves_str.is_empty() {
            "No moves available".to_string()
        } else {
            moves_str
        };
        template
            .replace("{player_color}", player_text)
            .replace("{board_state}", &board_text)
            .replace("{move_history}", &history_text)
            .replace("{available_moves}", moves_text.trim())
    }

    /// Parses numeric move index from model reply.
    pub fn parse(response: &str, limit: usize) -> Result<usize, AIError> {
        let cleaned: String = response.chars().filter(|c| c.is_ascii_digit()).collect();
        match cleaned.parse::<usize>() {
            Ok(value) if value > 0 && value <= limit => Ok(value),
            Ok(_) => Err(AIError::InvalidResponseFormat(format!(
                "Move index {cleaned} is out of bounds in range 1-{limit}"
            ))),
            Err(_) => Err(AIError::InvalidResponseFormat(format!(
                "AI returned non numeric response {response}"
            ))),
        }
    }
}

impl RulesExplainer for GeminiAI {
    fn explain<'a>(&'a self) -> BoxFuture<'a, Result<String, AIError>> {
        Box::pin(self.story())
    }
}

impl MoveChooser for GeminiAI {
    fn choose<'a>(&'a self, game: &'a CheckersGame) -> BoxFuture<'a, Result<Choice, AIError>> {
        Box::pin(self.select(game))
    }
}

impl HintGiver for GeminiAI {
    fn hint<'a>(
        &'a self,
        board_state: &'a Board,
        player: PieceColor,
        history: &'a MoveHistory,
    ) -> BoxFuture<'a, Result<String, AIError>> {
        Box::pin(self.advise(board_state, player, history))
    }
}
