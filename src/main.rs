mod ai;
mod core;
mod interface;
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
    let welcome_message = match crate::ai::ai::explain_rules().await {
        Ok(rules) => format!("# Welcome to Checkers!\n\n{}", rules),
        Err(crate::ai::ai::AIError::NoApiKey) => String::from(messages::WELCOME_MESSAGE_NO_API),
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

    // Game loop
    while running.load(Ordering::SeqCst) {
        // Render current state
        ui.render_game(&game)?;

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
                }
                GameInput::Select => {
                    let (row, col) = ui.get_cursor();
                    // If we have a selected piece and cursor is on it, deselect it
                    if game.selected_piece == Some((row, col)) {
                        game.selected_piece = None;
                    } else {
                        match game.selected_piece {
                            None => {
                                if let Err(e) = game.select_piece(row, col) {
                                    // Move to bottom of screen and show error
                                    stdout.queue(MoveTo(0, game.board.size as u16 + 3))?;
                                    let error_msg = format_model_output(&format!(
                                        "{} {}",
                                        messages::ERROR_PREFIX,
                                        e
                                    ))?;
                                    writeln!(stdout, "{}", error_msg)?;
                                    stdout.flush()?;
                                }
                            }
                            Some(_) => {
                                if let Err(e) = game.make_move(row, col) {
                                    // Move to bottom of screen and show error
                                    stdout.queue(MoveTo(0, game.board.size as u16 + 3))?;
                                    let error_msg = format_model_output(&format!(
                                        "{} {}",
                                        messages::ERROR_PREFIX,
                                        e
                                    ))?;
                                    writeln!(stdout, "{}", error_msg)?;
                                    stdout.flush()?;
                                }
                            }
                        }
                    }
                }
                GameInput::Quit => break,
            }
        }

        // Check for game over
        if let Some(_winner) = game.check_winner() {
            game.is_game_over = true;
            ui.render_game(&game)?;
            break;
        }

        // Check for stalemate
        if game.is_stalemate() {
            game.is_game_over = true;
            ui.render_game(&game)?;
            break;
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
