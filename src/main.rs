mod ai;
mod core;
mod interface;
mod utils;

use crossterm::{
    cursor::{self, Show},
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::{self, stdout};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::ai::{explain_rules, get_ai_move, hint::HintProvider, AIError};
use crate::core::game;
use crate::core::piece::Color as PieceColor;
use crate::interface::input;
use crate::interface::input::{CursorDirection, GameInput};
use crate::interface::ui::UI;
use crate::interface::welcome_screen::display_welcome_screen;
use std::env;

fn cleanup_terminal() -> io::Result<()> {
    let mut stdout = stdout();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    // Clear the main screen after leaving alternate screen
    stdout.execute(Clear(ClearType::All))?;
    stdout.execute(cursor::MoveTo(0, 0))?;
    Ok(())
}

fn check_and_set_game_over(game: &mut game::CheckersGame) {
    if game.check_winner().is_some() || game.is_stalemate() {
        game.is_game_over = true;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize terminal for loading animation
    terminal::enable_raw_mode()?;
    {
        let mut stdout = stdout();
        stdout.execute(terminal::EnterAlternateScreen)?;
        stdout.execute(Clear(ClearType::All))?;
    }

    let welcome_message = match explain_rules().await {
        Ok(rules) => rules,
        Err(AIError::NoApiKey) | Err(AIError::NoModel) => {
            eprintln!("Note: Add GEMINI_API_KEY and GEMINI_MODEL to your .env file to enable AI-powered content.");
            String::from("Did You Know?\n...The game of checkers has been played for thousands of years!\n\nTip of the Day\n...Always try to control the center of the board.\n\nChallenge\n...Try to win a game without losing any pieces!")
        }
        Err(e) => {
            eprintln!("Failed to get checkers content from AI: {}", e);
            String::from("Did You Know?\n...The game of checkers has been played for thousands of years!\n\nTip of the Day\n...Always try to control the center of the board.\n\nChallenge\n...Try to win a game without losing any pieces!")
        }
    };

    // Reset terminal for welcome screen
    terminal::disable_raw_mode()?;
    {
        let mut stdout = stdout();
        stdout.execute(terminal::LeaveAlternateScreen)?;
    }

    let should_continue = display_welcome_screen(&welcome_message)?;

    // Show cursor after welcome screen
    stdout().execute(Show)?;

    if !should_continue {
        // Clear the screen before exiting
        let mut stdout = stdout();
        stdout.execute(Clear(ClearType::All))?;
        stdout.execute(cursor::MoveTo(0, 0))?;
        cleanup_terminal()?;
        return Ok(());
    }

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let mut stdout = stdout();
    // Clear the welcome screen before entering alternate screen
    stdout.execute(Clear(ClearType::All))?;
    stdout.execute(cursor::MoveTo(0, 0))?;
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;

    let mut game = game::CheckersGame::new();
    let mut ui = UI::new();
    let mut needs_render = true;
    let mut current_hint: Option<String> = None;

    // Check if AI mode is available
    let api_key = env::var("GEMINI_API_KEY").ok();
    let model = env::var("GEMINI_MODEL").ok();
    let ai_enabled = api_key.is_some() && model.is_some();
    let hint_provider = api_key
        .as_ref()
        .and_then(|key| HintProvider::new(key.clone()).ok());

    while running.load(Ordering::SeqCst) {
        if needs_render {
            ui.render_game_with_hint_and_mode(&game, current_hint.as_deref(), ai_enabled)?;
            needs_render = false;
        }

        if let Some(input) = input::read_input()? {
            match input {
                GameInput::MoveCursor(direction) => {
                    let (row, col) = ui.get_cursor();
                    let (new_row, new_col) = match direction {
                        CursorDirection::Up => (row.saturating_sub(1), col),
                        CursorDirection::Down => ((row + 1).min(game.board.size - 1), col),
                        CursorDirection::Left => (row, col.saturating_sub(1)),
                        CursorDirection::Right => (row, (col + 1).min(game.board.size - 1)),
                    };
                    ui.set_cursor(new_row, new_col);
                    needs_render = true;
                }
                GameInput::Select => {
                    let (row, col) = ui.get_cursor();
                    if game.selected_piece == Some((row, col)) {
                        game.selected_piece = None;
                        needs_render = true;
                    } else {
                        match game.selected_piece {
                            None => {
                                let select_result = game.select_piece(row, col);
                                if let Err(_e) = select_result {
                                    // Error will be visible in UI
                                }
                                needs_render = true;
                            }
                            Some(_) => {
                                // Check if the selected cell is in possible moves
                                if let Some(possible_moves) = &game.possible_moves {
                                    if !possible_moves.contains(&(row, col)) {
                                        // Only clear selection if NOT in a multi-capture sequence
                                        if !game.is_in_multi_capture() {
                                            game.selected_piece = None;
                                            game.possible_moves = None;
                                        }
                                        needs_render = true;
                                    } else {
                                        // Attempt the move
                                        let move_result = game.make_move(row, col);
                                        if let Err(_e) = move_result {
                                            // Error will be visible in UI
                                        } else {
                                            game.ai_error = None; // Clear AI error on successful player move
                                            check_and_set_game_over(&mut game);
                                        }
                                        needs_render = true;
                                    }
                                } else {
                                    // No possible moves, clear selection
                                    game.selected_piece = None;
                                    needs_render = true;
                                }
                            }
                        }
                    }
                }
                GameInput::Quit => break,
            }
        }

        // Reset flag before checking for AI turn
        game.ai_thinking = false;

        // AI mode: Black is controlled by AI
        if ai_enabled && game.current_player == PieceColor::Black && !game.is_game_over {
            game.ai_thinking = true;
            ui.render_game_with_hint_and_mode(&game, current_hint.as_deref(), ai_enabled)?;

            match get_ai_move(&game).await {
                Ok(((from_row, from_col), (to_row, to_col))) => {
                    game.ai_thinking = false;
                    let select_result = game.select_piece(from_row, from_col);
                    if let Err(e) = select_result {
                        eprintln!(
                            "AI failed to select piece at ({}, {}): {:?}",
                            from_row, from_col, e
                        );
                        game.switch_player();
                    } else {
                        let move_result = game.make_move(to_row, to_col);
                        if let Err(e) = move_result {
                            eprintln!("AI failed to move to ({}, {}): {:?}", to_row, to_col, e);
                            if game.current_player == PieceColor::Black {
                                game.switch_player();
                            }
                        } else {
                            game.ai_error = None; // Clear AI error on successful move
                            check_and_set_game_over(&mut game);

                            // Automatically update hint after AI move
                            if let Some(ref provider) = hint_provider {
                                if game.current_player == PieceColor::White && !game.is_game_over {
                                    match provider
                                        .get_hint(
                                            &game.board,
                                            PieceColor::White,
                                            &game.move_history,
                                        )
                                        .await
                                    {
                                        Ok(hint) => {
                                            current_hint = Some(hint);
                                        }
                                        Err(e) => {
                                            eprintln!("Failed to get hint after AI move: {}", e);
                                            current_hint = None;
                                        }
                                    }
                                } else {
                                    current_hint = None;
                                }
                            } else {
                                current_hint = None;
                            }
                        }
                    }
                }
                Err(ai_error) => {
                    game.ai_thinking = false;
                    game.ai_error = Some(format!("AI Error: {}", ai_error));
                    game.switch_player();
                }
            }
            game.ai_thinking = false; // Ensure flag is reset after AI move

            // Immediately render the board to show AI move
            ui.render_game_with_hint_and_mode(&game, current_hint.as_deref(), ai_enabled)?;
            needs_render = false; // Reset since we just rendered
        }

        if game.is_game_over {
            ui.render_game_with_hint_and_mode(&game, None, ai_enabled)?;
            // Wait for any key press before exiting
            loop {
                if input::read_input()?.is_some() {
                    break;
                }
            }
            break;
        }
    }

    // Cleanup and exit
    cleanup_terminal()?;
    Ok(())
}
