mod board;
mod game;
mod input;
mod piece;
mod ui;
mod ai;

use crossterm::{
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    cursor::MoveTo,
    ExecutableCommand,
    QueueableCommand,
};
use std::io::{self, stdout, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use input::{CursorDirection, GameInput};
use ui::UI;

fn cleanup_terminal() -> io::Result<()> {
    let mut stdout = stdout();
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get AI explanation of rules
    let rules = match ai::explain_rules().await {
        Ok(rules) => rules,
        Err(e) => {
            eprintln!("Failed to get game rules from AI: {}", e);
            eprintln!("Please check your API key and internet connection.");
            return Ok(());
        }
    };
    
    // Display rules and wait for user input
    println!("Welcome to Checkers!\n");
    println!("Game Rules:\n{}\n", rules);
    println!("Press Enter to start the game...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    // Setup Ctrl+C handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

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
                    match game.selected_piece {
                        None => {
                            if let Err(e) = game.select_piece(row, col) {
                                // Move to bottom of screen and show error
                                stdout.queue(MoveTo(0, game.board.size as u16 + 3))?;
                                writeln!(stdout, "Error: {}", e)?;
                                stdout.flush()?;
                            }
                        }
                        Some(_) => {
                            if let Err(e) = game.make_move(row, col) {
                                // Move to bottom of screen and show error
                                stdout.queue(MoveTo(0, game.board.size as u16 + 3))?;
                                writeln!(stdout, "Error: {}", e)?;
                                stdout.flush()?;
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
        }).expect("Error setting Ctrl-C handler");

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
