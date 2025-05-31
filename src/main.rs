mod core;
mod interface;
mod ai;
mod utils;

use crossterm::{
    cursor::MoveTo,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};
use std::io::{self, stdout, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::core::game;
use crate::interface::input;
use crate::interface::input::{CursorDirection, GameInput};
use crate::interface::messages;
use crate::interface::ui::UI;
use crate::utils::markdown::parser::MarkdownRenderer;
use crate::ai::{explain_rules, get_ai_move, AIError};
use crate::core::piece::Color as PieceColor;

fn cleanup_terminal() -> io::Result<()> {
    let mut stdout = stdout();
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn format_model_output(message: &str) -> std::io::Result<String> {
    let mut renderer = MarkdownRenderer::new();
    renderer.render(message)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Try to get AI story about checkers if API key is available
    let welcome_message = match explain_rules().await {
        Ok(rules) => format!("# Welcome to Checkers!\n\n{}", rules),
        Err(AIError::NoApiKey) => String::from(messages::WELCOME_MESSAGE_NO_API),
        Err(e) => {
            eprintln!("Failed to get checkers story from AI: {}", e);
            String::from(messages::WELCOME_MESSAGE_ERROR)
        }
    };

    // Format and display the welcome message using MarkdownRenderer
    let formatted_message = format_model_output(&welcome_message)?;

    // Use write! macro to properly display ANSI color codes
    write!(io::stdout(), "{}\n\n", formatted_message)?;
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Setup Ctrl+C handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // Setup terminal
    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;

    // Initialize game state
    let mut game = game::CheckersGame::new();
    let mut ui = UI::new();
    let mut needs_render = true; // Initialize needs_render flag

    // Game loop
    while running.load(Ordering::SeqCst) {
        // Render current state if needed
        if needs_render {
            ui.render_game(&game)?;
            needs_render = false;
        }

        // Handle input
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
                            None => { // Attempting to select a piece
                                let select_result = game.select_piece(row, col);
                                if let Err(e) = select_result {
                                    // Error when selecting, display message
                                    stdout.queue(MoveTo(0, game.board.size as u16 + 3))?;
                                    let error_msg = format_model_output(&format!(
                                        "{} {}",
                                        messages::ERROR_PREFIX,
                                        e
                                    ))?;
                                    writeln!(stdout, "{}", error_msg)?;
                                    stdout.flush()?;
                                }
                                // select_piece call (successful or not) always means we should update visual state or show error
                                needs_render = true;
                            }
                            Some(_) => { // Attempting to make a move
                                let move_result = game.make_move(row, col);
                                if let Err(e) = move_result {
                                    // Error when moving, display message
                                    stdout.queue(MoveTo(0, game.board.size as u16 + 3))?;
                                    let error_msg = format_model_output(&format!(
                                        "{} {}",
                                        messages::ERROR_PREFIX,
                                        e
                                    ))?;
                                    writeln!(stdout, "{}", error_msg)?;
                                    stdout.flush()?;
                                } else { // Move was successful
                                    if game.check_winner().is_some() || game.is_stalemate() {
                                        game.is_game_over = true;
                                    }
                                }
                                // Whether move succeeded or failed, game state (board, player) changed or error shown
                                needs_render = true;
                            }
                        }
                    }
                }
                GameInput::Quit => break,
            }
        }

        // AI's turn (Black)
        if game.current_player == PieceColor::Black && !game.is_game_over {
            stdout.queue(MoveTo(0, game.board.size as u16 + 4))?;
            let thinking_msg = format_model_output("Black (AI) is thinking...")?;
            write!(stdout, "{}", thinking_msg)?;
            stdout.queue(MoveTo(0, game.board.size as u16 + 5))?;
            stdout.flush()?;

            match get_ai_move(&game).await {
                Ok(((from_row, from_col), (to_row, to_col))) => {
                    let select_result = game.select_piece(from_row, from_col);
                    if let Err(e) = select_result {
                        stdout.queue(MoveTo(0, game.board.size as u16 + 4))?;
                        let error_msg = format_model_output(&format!(
                            "{} AI failed to select piece: {}. Skipping turn.",
                            messages::ERROR_PREFIX, e
                        ))?;
                        writeln!(stdout, "{}", error_msg)?;
                        stdout.flush()?;
                        game.switch_player();
                    } else { // Piece selected successfully by AI
                        let move_result = game.make_move(to_row, to_col);
                        if let Err(e) = move_result {
                            stdout.queue(MoveTo(0, game.board.size as u16 + 4))?;
                            let error_msg = format_model_output(&format!(
                                "{} AI failed to make move: {}. Skipping turn.",
                                messages::ERROR_PREFIX, e
                            ))?;
                            writeln!(stdout, "{}", error_msg)?;
                            stdout.flush()?;
                            if game.current_player == PieceColor::Black { game.switch_player(); }
                        } else { // AI move successful
                            if game.check_winner().is_some() || game.is_stalemate() {
                                game.is_game_over = true;
                            }
                        }
                    }
                }
                Err(ai_error) => { // Error from get_ai_move
                    stdout.queue(MoveTo(0, game.board.size as u16 + 4))?;
                    let error_msg = format_model_output(&format!(
                        "{} AI error: {}. Skipping turn.",
                        messages::ERROR_PREFIX, ai_error
                    ))?;
                    writeln!(stdout, "{}", error_msg)?;
                    stdout.flush()?;
                    game.switch_player();
                }
            }
            needs_render = true; // AI turn always triggers a re-render
            // std::thread::sleep(std::time::Duration::from_millis(500)); // Optional
        }

        // Check for game over (after player or AI move)
        if game.is_game_over {
            needs_render = true; // Ensure final state is rendered
        }

        if needs_render && game.is_game_over { // Render once more if game ended and needs_render was set
            ui.render_game(&game)?; // Render the final game state
            // needs_render = false; // Redundant: loop breaks immediately after.
            break; // Exit loop as game is over
        }
    }

    // Cleanup terminal
    cleanup_terminal()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_ctrl_c_handler() {
        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        // Set up the Ctrl+C handler
        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");

        // Verify initial state
        assert!(running.load(Ordering::SeqCst));

        // Simulate Ctrl+C by sending SIGINT
        unsafe {
            libc::raise(libc::SIGINT);
        }

        // Give a small amount of time for the handler to process
        thread::sleep(Duration::from_millis(100));

        // Verify the handler worked
        assert!(!running.load(Ordering::SeqCst));
    }

    #[test]
    fn test_cleanup_terminal() {
        // First enable raw mode and enter alternate screen
        terminal::enable_raw_mode().unwrap();
        stdout().execute(EnterAlternateScreen).unwrap();

        // Test cleanup
        cleanup_terminal().unwrap();

        // Verify terminal state
        assert!(!terminal::is_raw_mode_enabled().unwrap());
    }
}
