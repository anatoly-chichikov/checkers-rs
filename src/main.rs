mod ai;
mod core;
mod interface;

use crossterm::{
    cursor::MoveTo,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};
use std::io::{self, stdout, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::ai::{explain_rules, get_ai_move, AIError};
use crate::core::game;
use crate::core::piece::Color as PieceColor;
use crate::interface::input;
use crate::interface::input::{CursorDirection, GameInput};
use crate::interface::messages;
use crate::interface::ui::UI;
use crate::interface::welcome_screen::display_welcome_screen;
use std::env;

fn cleanup_terminal() -> io::Result<()> {
    let mut stdout = stdout();
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn display_game_message(stdout: &mut io::Stdout, y_pos: u16, message: &str) -> io::Result<()> {
    stdout.queue(MoveTo(0, y_pos))?;
    let terminal_width = terminal::size().map(|(cols, _)| cols).unwrap_or(80);
    write!(stdout, "{:<width$}", "", width = terminal_width as usize)?;
    stdout.queue(MoveTo(0, y_pos))?;

    writeln!(stdout, "{}", message)?;
    stdout.flush()?;
    Ok(())
}

fn check_and_set_game_over(game: &mut game::CheckersGame) {
    if game.check_winner().is_some() || game.is_stalemate() {
        game.is_game_over = true;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let board_bottom_y = game::CheckersGame::new().board.size as u16;
    const MSG_Y_OFFSET_PLAYER_ERROR: u16 = 3;
    const MSG_Y_OFFSET_AI_STATUS: u16 = 4;

    let welcome_message = match explain_rules().await {
        Ok(rules) => rules,
        Err(AIError::NoApiKey) => {
            eprintln!("Note: Add GEMINI_API_KEY to your .env file to enable AI-powered content.");
            String::from("Did You Know?\n...The game of checkers has been played for thousands of years!\n\nTip of the Day\n...Always try to control the center of the board.\n\nChallenge\n...Try to win a game without losing any pieces!")
        }
        Err(e) => {
            eprintln!("Failed to get checkers content from AI: {}", e);
            String::from("Did You Know?\n...The game of checkers has been played for thousands of years!\n\nTip of the Day\n...Always try to control the center of the board.\n\nChallenge\n...Try to win a game without losing any pieces!")
        }
    };

    display_welcome_screen(&welcome_message)?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let mut stdout = stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;

    let mut game = game::CheckersGame::new();
    let mut ui = UI::new();
    let mut needs_render = true;

    // Check if AI mode is available
    let ai_enabled = env::var("GEMINI_API_KEY").is_ok();

    if !ai_enabled {
        display_game_message(
            &mut stdout,
            board_bottom_y + MSG_Y_OFFSET_AI_STATUS,
            "Two-player mode: Red and Black players take turns",
        )?;
    }

    while running.load(Ordering::SeqCst) {
        if needs_render {
            ui.render_game(&game)?;
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
                                if let Err(e) = select_result {
                                    display_game_message(
                                        &mut stdout,
                                        board_bottom_y + MSG_Y_OFFSET_PLAYER_ERROR,
                                        &format!("{} {}", messages::ERROR_PREFIX, e),
                                    )?;
                                }
                                needs_render = true;
                            }
                            Some(_) => {
                                let move_result = game.make_move(row, col);
                                if let Err(e) = move_result {
                                    display_game_message(
                                        &mut stdout,
                                        board_bottom_y + MSG_Y_OFFSET_PLAYER_ERROR,
                                        &format!("{} {}", messages::ERROR_PREFIX, e),
                                    )?;
                                } else {
                                    check_and_set_game_over(&mut game);
                                }
                                needs_render = true;
                            }
                        }
                    }
                }
                GameInput::Quit => break,
            }
        }

        // AI mode: Black is controlled by AI
        if ai_enabled && game.current_player == PieceColor::Black && !game.is_game_over {
            display_game_message(
                &mut stdout,
                board_bottom_y + MSG_Y_OFFSET_AI_STATUS,
                "Black (AI) is thinking...",
            )?;

            match get_ai_move(&game).await {
                Ok(((from_row, from_col), (to_row, to_col))) => {
                    let select_result = game.select_piece(from_row, from_col);
                    if let Err(e) = select_result {
                        display_game_message(
                            &mut stdout,
                            board_bottom_y + MSG_Y_OFFSET_AI_STATUS,
                            &format!(
                                "{} AI failed to select piece: {}. Skipping turn.",
                                messages::ERROR_PREFIX,
                                e
                            ),
                        )?;
                        game.switch_player();
                    } else {
                        let move_result = game.make_move(to_row, to_col);
                        if let Err(e) = move_result {
                            display_game_message(
                                &mut stdout,
                                board_bottom_y + MSG_Y_OFFSET_AI_STATUS,
                                &format!(
                                    "{} AI failed to make move: {}. Skipping turn.",
                                    messages::ERROR_PREFIX,
                                    e
                                ),
                            )?;
                            if game.current_player == PieceColor::Black {
                                game.switch_player();
                            }
                        } else {
                            check_and_set_game_over(&mut game);
                        }
                    }
                }
                Err(ai_error) => {
                    display_game_message(
                        &mut stdout,
                        board_bottom_y + MSG_Y_OFFSET_AI_STATUS,
                        &format!(
                            "{} AI error: {}. Skipping turn.",
                            messages::ERROR_PREFIX,
                            ai_error
                        ),
                    )?;
                    game.switch_player();
                }
            }
            needs_render = true;
        }

        if game.is_game_over {
            ui.render_game(&game)?;
            break;
        }
    }

    cleanup_terminal()?;
    Ok(())
}
