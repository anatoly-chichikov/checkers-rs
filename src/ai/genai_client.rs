use genai::{
    chat::{ChatMessage, ChatOptions, ChatRequest},
    Client,
};
use std::env;

use crate::ai::error::AIError;
use crate::ai::formatting::{format_board, format_square};
use crate::ai::ui::{start_loading_animation, stop_loading_animation};
use crate::core::game::CheckersGame;
use crate::core::game_logic::get_all_valid_moves_for_player;
use crate::core::piece::Color as PieceColor;
use crate::interface::messages;
use crate::utils::prompts::get_ai_move_prompt;



pub async fn explain_rules() -> Result<String, AIError> {
    dotenv::dotenv().ok();

    let api_key = env::var("GEMINI_API_KEY").map_err(|_| AIError::NoApiKey)?;
    let model = env::var("GEMINI_MODEL").map_err(|_| AIError::NoModel)?;

    let (running, loading_thread) = start_loading_animation()?;

    // Create client with the API key set in environment
    env::set_var("GEMINI_API_KEY", api_key);
    let client = Client::default();

    let chat_req = ChatRequest::new(vec![ChatMessage::user(messages::STORY_PROMPT)]);

    let chat_options = ChatOptions::default()
        .with_temperature(0.7)
        .with_max_tokens(512);

    let result = client
        .exec_chat(&model, chat_req, Some(&chat_options))
        .await;

    stop_loading_animation(running, loading_thread)?;

    let result = result.map_err(|e| AIError::RequestFailed(e.to_string()))?;

    match result.content_text_as_str() {
        Some(text) => {
            // Remove HTML tags from the response
            let cleaned_text = text
                .replace("<br>", "\n")
                .replace("<br/>", "\n")
                .replace("<br />", "\n");
            Ok(cleaned_text)
        }
        None => Err(AIError::ParseError(
            "No text content in response".to_string(),
        )),
    }
}

#[allow(dead_code)]
pub async fn get_ai_move(game: &CheckersGame) -> Result<((usize, usize), (usize, usize)), AIError> {
    dotenv::dotenv().ok();
    let api_key = env::var("GEMINI_API_KEY").map_err(|_| AIError::NoApiKey)?;

    if game.current_player != PieceColor::Black {
        return Err(AIError::InvalidResponseFormat(
            "AI can only play as Black.".to_string(),
        ));
    }

    let possible_moves = get_all_valid_moves_for_player(&game.board, game.current_player);
    if possible_moves.is_empty() {
        return Err(AIError::NoPossibleMoves);
    }

    let board_representation = format_board(&game.board);
    let mut moves_str = String::new();
    for (i, ((from_row, from_col), (to_row, to_col), is_capture)) in
        possible_moves.iter().enumerate()
    {
        let formatted_from_sq = format_square(*from_row, *from_col);
        let formatted_to_sq = format_square(*to_row, *to_col);
        let mut move_desc = format!("{}. {} to {}", i + 1, formatted_from_sq, formatted_to_sq);

        if *is_capture {
            let mid_row = (from_row + to_row) / 2;
            let mid_col = (from_col + to_col) / 2;
            let formatted_captured_sq = format_square(mid_row, mid_col);
            move_desc.push_str(&format!(" (captures piece at {})", formatted_captured_sq));
        }
        moves_str.push_str(&move_desc);
        moves_str.push('\n');
    }

    let prompt_template = get_ai_move_prompt();
    let prompt = prompt_template
        .replace("{board_state}", &board_representation)
        .replace("{available_moves}", moves_str.trim());

    // Create client with the API key set in environment
    env::set_var("GEMINI_API_KEY", api_key);
    let client = Client::default();
    let model = env::var("GEMINI_MODEL").map_err(|_| AIError::NoModel)?;

    let chat_req = ChatRequest::new(vec![ChatMessage::user(prompt)]);

    let chat_options = ChatOptions::default()
        .with_temperature(0.1) // Lower temperature for more deterministic responses
        .with_max_tokens(5); // We only need a single digit

    let (running, loading_thread) = start_loading_animation()?;

    let result = client
        .exec_chat(&model, chat_req, Some(&chat_options))
        .await;

    stop_loading_animation(running, loading_thread)?;

    let result = result.map_err(|e| AIError::RequestFailed(e.to_string()))?;

    match result.content_text_as_str() {
        Some(text_response) => {
            let text_response = text_response.trim();

            let cleaned_response = text_response
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>();

            match cleaned_response.parse::<usize>() {
                Ok(move_number) if move_number > 0 && move_number <= possible_moves.len() => {
                    let chosen_move_data = &possible_moves[move_number - 1];
                    Ok((chosen_move_data.0, chosen_move_data.1))
                }
                Ok(_) => Err(AIError::InvalidResponseFormat(format!(
                    "Move index {} is out of bounds. Valid range: 1-{}. Original response: '{}'",
                    cleaned_response,
                    possible_moves.len(),
                    text_response
                ))),
                Err(_) => Err(AIError::InvalidResponseFormat(format!(
                    "AI returned non-numeric or invalid response: '{}'. Cleaned: '{}'",
                    text_response, cleaned_response
                ))),
            }
        }
        None => Err(AIError::ParseError(
            "No text content in response".to_string(),
        )),
    }
}
