use reqwest::Client;
use std::env;

use crate::ai::error::AIError;
use crate::ai::models::{Content, GeminiRequest, GeminiResponse, Part, GenerationConfig};
use crate::ai::ui::{start_loading_animation, stop_loading_animation};
use crate::interface::messages;
use crate::core::game::CheckersGame;
use crate::core::board::Board;
use crate::core::piece::Color as PieceColor; // Renamed Color to PieceColor to avoid conflict, removed Piece
use crate::core::game_logic::get_all_valid_moves_for_player;

// Helper function to format square coordinates (e.g., (0,0) to "A1")
fn format_square(row: usize, col: usize) -> String {
    format!("{}{}", (col as u8 + b'A') as char, row + 1)
}

// Helper function to format the board into a textual representation
fn format_board(board: &Board) -> String {
    let mut board_str = String::new();
    board_str.push_str("  A B C D E F G H\n");
    for r in 0..8 {
        board_str.push_str(&format!("{} ", r + 1));
        for c in 0..8 {
            let piece_str: String = match board.get_piece(r, c) {
                Some(piece) => piece.display(), // Returns String, e.g., "(w)"
                None => ".".to_string(),        // Returns String, "."
            };
            board_str.push_str(&piece_str); // Append the string representation
            board_str.push(' '); // Add a space after the piece or placeholder
        }
        board_str.push('\n');
    }
    board_str
}


pub async fn explain_rules() -> Result<String, AIError> {
    dotenv::dotenv().ok();

    let api_key = env::var("GEMINI_API_KEY").map_err(|_| AIError::NoApiKey)?;

    // Start loading animation
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
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={}",
        api_key
    );

    let response = client
        .post(&api_url)
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| AIError::RequestFailed(e.to_string()))?;

    // Stop the loading animation and cleanup
    stop_loading_animation(running, loading_thread)?;

    let status = response.status();
    if !status.is_success() {
        let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(AIError::RequestFailed(format!("API request failed with status {}: {}", status, error_body)));
    }

    let response_data: GeminiResponse = response
        .json()
        .await
        .map_err(|e| AIError::ParseError(e.to_string()))?;

    if let Some(candidate) = response_data.candidates.first() {
        if let Some(part) = candidate.content.parts.first() {
            Ok(part.text.clone())
        } else {
            Err(AIError::ParseError("No parts in candidate content".to_string()))
        }
    } else {
        Err(AIError::ParseError("No candidates in response".to_string()))
    }
} 

pub async fn get_ai_move(
    game: &CheckersGame,
) -> Result<((usize, usize), (usize, usize)), AIError> {
    dotenv::dotenv().ok();
    let api_key = env::var("GEMINI_API_KEY").map_err(|_| AIError::NoApiKey)?;

    // Ensure it's Black's turn for AI
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
    for (i, ((from_row, from_col), (to_row, to_col), is_capture)) in possible_moves.iter().enumerate() {
        let formatted_from_sq = format_square(*from_row, *from_col);
        let formatted_to_sq = format_square(*to_row, *to_col);
        let mut move_desc = format!("{}. {} to {}", i + 1, formatted_from_sq, formatted_to_sq);

        if *is_capture {
            // Calculate midpoint for capture
            let mid_row = (from_row + to_row) / 2;
            let mid_col = (from_col + to_col) / 2;
            let formatted_captured_sq = format_square(mid_row, mid_col);
            move_desc.push_str(&format!(" (captures piece at {})", formatted_captured_sq));
        }
        moves_str.push_str(&move_desc);
        moves_str.push_str("\n");
    }

    let prompt = format!(
        "Current checkers board state:\n{}\nIt is Black's (b/B) turn.\n\nHere are your possible moves:\n{}\n\
        Please respond with ONLY the number corresponding to your chosen move from the list above. For example, if you choose move 1, respond with '1'.",
        board_representation,
        moves_str.trim()
    );

    let client = Client::new();
    let request = GeminiRequest {
        contents: vec![Content {
            parts: vec![Part { text: prompt }],
        }],
        generation_config: GenerationConfig {
            temperature: 0.5, // Slightly deterministic but allows some variation
            max_output_tokens: 10, // Expecting a short number
        },
    };

    let api_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={}",
        api_key
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
        let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(AIError::RequestFailed(format!("API request failed with status {}: {}", status, error_body)));
    }

    let response_data: GeminiResponse = response
        .json()
        .await
        .map_err(|e| AIError::ParseError(format!("Failed to parse JSON response: {}", e)))?;

    if let Some(candidate) = response_data.candidates.first() {
        if let Some(part) = candidate.content.parts.first() {
            let text_response = part.text.trim();
            // Try to parse only the numeric part, handling potential trailing characters like '.'
            let cleaned_response = text_response.chars().filter(|c| c.is_digit(10)).collect::<String>();
            match cleaned_response.parse::<usize>() {
                Ok(move_number) if move_number > 0 && move_number <= possible_moves.len() => {
                    let chosen_move_data = &possible_moves[move_number - 1];
                    // chosen_move_data is ((from_r, from_c), (to_r, to_c), is_capture)
                    // We need to return Ok(((from_r, from_c), (to_r, to_c)))
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
                    text_response,
                    cleaned_response
                ))),
            }
        } else {
            Err(AIError::ParseError("No parts in candidate content".to_string()))
        }
    } else {
        Err(AIError::ParseError("No candidates in response".to_string()))
    }
}

#[cfg(test)]
mod tests {
    // Removed: use super::*;

    #[test]
    fn test_parse_ai_move_string() {
        // Define a dummy list of possible moves, similar to what get_all_valid_moves_for_player would produce.
        // ((from_row, from_col), (to_row, to_col), is_capture)
        let possible_moves: Vec<((usize, usize), (usize, usize), bool)> = vec![
            ((2, 2), (3, 3), false), // AI response "1" should map to this
            ((4, 4), (5, 5), false), // AI response "2" should map to this
            ((6, 6), (7, 7), true),  // AI response "3" should map to this
        ];

        // Test cases: (ai_response_string, expected_parsed_move_tuple, description)
        // expected_parsed_move_tuple is Option<((usize, usize), (usize, usize))>
        let test_cases = vec![
            ("1", Some((possible_moves[0].0, possible_moves[0].1)), "Valid: Basic number"),
            (" 2 ", Some((possible_moves[1].0, possible_moves[1].1)), "Valid: Number with whitespace"),
            ("3.", Some((possible_moves[2].0, possible_moves[2].1)), "Valid: Number with trailing period (AI might add this)"),
            ("  1  ", Some((possible_moves[0].0, possible_moves[0].1)), "Valid: Number with extensive whitespace"),
            ("0", None, "Invalid: Zero index"),
            ("4", None, "Invalid: Out of bounds (too high)"),
            ("99", None, "Invalid: Way out of bounds"),
            ("hello", None, "Invalid: Non-numeric response"),
            ("1.5", None, "Invalid: Non-integer number"),
            ("", None, "Invalid: Empty string"),
            ("  ", None, "Invalid: Whitespace only string"),
        ];

        for (ai_response, expected_move_tuple, description) in test_cases {
            println!("Testing case: {}", description); // For better test output

            let text_response = ai_response.trim();
            let parsed_move_index: Option<usize> = match text_response.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<usize>() {
                Ok(move_number) if move_number > 0 && move_number <= possible_moves.len() => {
                    Some(move_number - 1) // Convert to 0-based index
                }
                _ => None, // Covers parsing errors, 0, or out-of-bounds
            };

            if let Some(expected_coords) = expected_move_tuple {
                assert!(parsed_move_index.is_some(), "Expected a valid index for '{}', but got None. Case: {}", ai_response, description);
                let index = parsed_move_index.unwrap();
                let chosen_move_data = &possible_moves[index];
                let actual_coords = (chosen_move_data.0, chosen_move_data.1);
                assert_eq!(actual_coords, expected_coords, "Mismatch for '{}'. Expected {:?}, got {:?}. Case: {}", ai_response, expected_coords, actual_coords, description);
            } else {
                assert!(parsed_move_index.is_none(), "Expected None (invalid index) for '{}', but got Some({:?}). Case: {}", ai_response, parsed_move_index, description);
            }
        }
    }
}