mod ai;
mod core;
mod interface;
mod utils;

use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::ai::{explain_rules, get_ai_move, hint::HintProvider, AIError, Hint};
use crate::core::{game::CheckersGame, piece::Color};
use crate::interface::ui_ratatui::{Input, UI};

async fn get_welcome_content() -> (String, String, String) {
    match explain_rules().await {
        Ok(rules) => {
            let parts: Vec<&str> = rules.split("\n\n").collect();
            if parts.len() >= 3 {
                let did_you_know = parts[0]
                    .strip_prefix("Did You Know?\n")
                    .or_else(|| parts[0].strip_prefix("**Did You Know?**\n"))
                    .unwrap_or(parts[0])
                    .replace("**", "")
                    .to_string();
                let tip_of_the_day = parts[1]
                    .strip_prefix("Tip of the Day\n")
                    .or_else(|| parts[1].strip_prefix("**Tip of the Day**\n"))
                    .unwrap_or(parts[1])
                    .replace("**", "")
                    .to_string();
                let todays_challenge = parts[2]
                    .strip_prefix("Challenge\n")
                    .or_else(|| parts[2].strip_prefix("**Challenge**\n"))
                    .or_else(|| parts[2].strip_prefix("**Today's Challenge**\n"))
                    .unwrap_or(parts[2])
                    .replace("**", "")
                    .to_string();
                (did_you_know, tip_of_the_day, todays_challenge)
            } else {
                default_welcome_content()
            }
        }
        Err(AIError::NoApiKey) | Err(AIError::NoModel) => {
            eprintln!("Note: Add GEMINI_API_KEY and GEMINI_MODEL to your .env file to enable AI-powered content.");
            default_welcome_content()
        }
        Err(e) => {
            eprintln!("Failed to get checkers content from AI: {}", e);
            default_welcome_content()
        }
    }
}

fn default_welcome_content() -> (String, String, String) {
    (
        "The game of checkers has been played for thousands of years!".to_string(),
        "Always try to control the center of the board.".to_string(),
        "Try to win a game without losing any pieces!".to_string(),
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize UI
    let mut ui = UI::new()?;
    ui.init()?;

    // Get welcome content
    let (did_you_know, tip_of_the_day, todays_challenge) = get_welcome_content().await;

    // Display welcome screen
    ui.draw_welcome_screen(&did_you_know, &tip_of_the_day, &todays_challenge)?;

    // Wait for user input
    loop {
        match ui.get_input()? {
            Input::Select => break, // ENTER pressed
            Input::Quit => {
                ui.restore()?;
                return Ok(());
            }
            _ => {} // Ignore other inputs
        }
    }

    // Setup Ctrl-C handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    // Initialize game
    let mut game = CheckersGame::new();
    let mut current_hint: Option<Hint> = None;

    // Check if AI mode is available
    let api_key = env::var("GEMINI_API_KEY").ok();
    let model = env::var("GEMINI_MODEL").ok();
    let ai_enabled = api_key.is_some() && model.is_some();
    let hint_provider = api_key
        .as_ref()
        .and_then(|key| HintProvider::new(key.clone()).ok());

    // Main game loop
    while running.load(Ordering::SeqCst) && !game.is_game_over {
        // Draw the game
        ui.draw_game(
            &game,
            game.selected_piece,
            game.possible_moves.as_ref().unwrap_or(&Vec::new()),
            current_hint.as_ref(),
            game.ai_thinking,
            game.ai_error.as_deref(),
        )?;

        // Handle AI turn
        if ai_enabled && game.current_player == Color::Black {
            game.ai_thinking = true;
            ui.draw_game(
                &game,
                game.selected_piece,
                game.possible_moves.as_ref().unwrap_or(&Vec::new()),
                current_hint.as_ref(),
                game.ai_thinking,
                game.ai_error.as_deref(),
            )?;

            match get_ai_move(&game).await {
                Ok(((from_row, from_col), (to_row, to_col))) => {
                    game.ai_thinking = false;
                    if let Err(e) = game.select_piece(from_row, from_col) {
                        game.ai_error = Some(format!("AI failed to select piece: {}", e));
                        game.switch_player();
                    } else if let Err(e) = game.make_move(to_row, to_col) {
                        game.ai_error = Some(format!("AI failed to move: {}", e));
                        if game.current_player == Color::Black {
                            game.switch_player();
                        }
                    } else {
                        game.ai_error = None;
                        // Check for game over
                        if game.check_winner().is_some() || game.is_stalemate() {
                            game.is_game_over = true;
                        }
                        // Update hint after AI move
                        if let Some(ref provider) = hint_provider {
                            if game.current_player == Color::White && !game.is_game_over {
                                match provider
                                    .get_hint(&game.board, Color::White, &game.move_history)
                                    .await
                                {
                                    Ok(hint_text) => {
                                        current_hint = Some(Hint { hint: hint_text });
                                    }
                                    Err(_) => {
                                        current_hint = None;
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    game.ai_thinking = false;
                    game.ai_error = Some(format!("AI Error: {}", e));
                    game.switch_player();
                }
            }
            game.ai_thinking = false;
            continue; // Skip input handling for AI turn
        }

        // Handle player input
        let input = ui.get_input()?;
        match input {
            Input::Up | Input::Down | Input::Left | Input::Right => {
                ui.move_cursor(input);
            }
            Input::Select => {
                let cursor_pos = ui.get_cursor_position();

                // If we have a selected piece
                if let Some(selected) = game.selected_piece {
                    // Check if clicking on the same piece (deselect)
                    if selected == cursor_pos {
                        game.selected_piece = None;
                        game.possible_moves = None;
                    }
                    // Check if clicking on a possible move
                    else if let Some(ref moves) = game.possible_moves {
                        if moves.contains(&cursor_pos) {
                            // Make the move
                            if let Err(e) = game.make_move(cursor_pos.0, cursor_pos.1) {
                                // Handle error (could show in UI)
                                eprintln!("Move failed: {}", e);
                            } else {
                                // Check for game over
                                if game.check_winner().is_some() || game.is_stalemate() {
                                    game.is_game_over = true;
                                }
                                // Clear hint after player move
                                current_hint = None;
                            }
                        } else {
                            // Try to select a new piece
                            game.selected_piece = None;
                            if let Ok(()) = game.select_piece(cursor_pos.0, cursor_pos.1) {
                                // Selection successful
                            }
                        }
                    }
                }
                // No piece selected, try to select one
                else {
                    if let Ok(()) = game.select_piece(cursor_pos.0, cursor_pos.1) {
                        // Selection successful
                    }
                }
            }
            Input::Quit => break,
        }
    }

    // Show game over screen
    if game.is_game_over {
        let winner = game.check_winner();
        ui.draw_game_over(winner)?;

        // Wait for any key
        loop {
            match ui.get_input()? {
                Input::Quit | Input::Select => break,
                _ => {}
            }
        }
    }

    // Restore terminal
    ui.restore()?;
    Ok(())
}
