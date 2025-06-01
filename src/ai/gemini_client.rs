use reqwest::Client;
use std::env;

use crate::ai::error::AIError;
use crate::ai::models::{Content, GeminiRequest, GeminiResponse, GenerationConfig, Part};
use crate::ai::ui::{start_loading_animation, stop_loading_animation};
use crate::core::board::Board;
use crate::core::game::CheckersGame;
use crate::core::game_logic::get_all_valid_moves_for_player;
use crate::core::piece::Color as PieceColor;
use crate::interface::messages;

fn format_square(row: usize, col: usize) -> String {
    format!("{}{}", (col as u8 + b'A') as char, row + 1)
}

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

pub async fn explain_rules() -> Result<String, AIError> {
    dotenv::dotenv().ok();

    let api_key = env::var("GEMINI_API_KEY").map_err(|_| AIError::NoApiKey)?;
    let model = env::var("GEMINI_MODEL").map_err(|_| AIError::NoModel)?;

    let (running, loading_thread) = start_loading_animation()?;

    let client = Client::new();
    let request = GeminiRequest {
        contents: vec![Content {
            parts: vec![Part {
                text: messages::STORY_PROMPT.to_string(),
            }],
        }],
        generation_config: GenerationConfig {
            temperature: 0.7,
            max_output_tokens: 512,
        },
    };

    let api_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model, api_key
    );

    let response = client
        .post(&api_url)
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| AIError::RequestFailed(e.to_string()))?;

    stop_loading_animation(running, loading_thread)?;

    let status = response.status();
    if !status.is_success() {
        let error_body = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(AIError::RequestFailed(format!(
            "API request failed with status {}: {}",
            status, error_body
        )));
    }

    let response_data: GeminiResponse = response
        .json()
        .await
        .map_err(|e| AIError::ParseError(e.to_string()))?;

    if let Some(candidate) = response_data.candidates.first() {
        if let Some(part) = candidate.content.parts.first() {
            // Remove HTML tags from the response
            let cleaned_text = part
                .text
                .replace("<br>", "\n")
                .replace("<br/>", "\n")
                .replace("<br />", "\n");
            Ok(cleaned_text)
        } else {
            Err(AIError::ParseError(
                "No parts in candidate content".to_string(),
            ))
        }
    } else {
        Err(AIError::ParseError("No candidates in response".to_string()))
    }
}

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

    let prompt = format!(
        "You are playing checkers as Black (b/B pieces). Analyze the board and choose your move.\n\n\
        Current board state:\n{}\n\n\
        Available moves:\n{}\n\n\
        IMPORTANT: Respond with ONLY a single number (1, 2, 3, etc.) - nothing else.\n\
        Do not include any text, explanation, or punctuation.\n\
        Just the move number.\n\n\
        Your move number:",
        board_representation,
        moves_str.trim()
    );

    let client = Client::new();
    let request = GeminiRequest {
        contents: vec![Content {
            parts: vec![Part { text: prompt }],
        }],
        generation_config: GenerationConfig {
            temperature: 0.1,     // Lower temperature for more deterministic responses
            max_output_tokens: 5, // We only need a single digit
        },
    };

    let model = env::var("GEMINI_MODEL").map_err(|_| AIError::NoModel)?;
    let api_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model, api_key
    );

    let (running, loading_thread) = start_loading_animation()?;

    let response = client
        .post(&api_url)
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| AIError::RequestFailed(e.to_string()))?;

    stop_loading_animation(running, loading_thread)?;

    let status = response.status();
    if !status.is_success() {
        let error_body = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(AIError::RequestFailed(format!(
            "API request failed with status {}: {}",
            status, error_body
        )));
    }

    let response_data: GeminiResponse = response
        .json()
        .await
        .map_err(|e| AIError::ParseError(format!("Failed to parse JSON response: {}", e)))?;

    if let Some(candidate) = response_data.candidates.first() {
        if let Some(part) = candidate.content.parts.first() {
            let text_response = part.text.trim();

            // Debug: Print what AI returned
            eprintln!("AI raw response: '{}'", text_response);

            let cleaned_response = text_response
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>();

            eprintln!("AI cleaned response: '{}'", cleaned_response);

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
        } else {
            Err(AIError::ParseError(
                "No parts in candidate content".to_string(),
            ))
        }
    } else {
        Err(AIError::ParseError("No candidates in response".to_string()))
    }
}
