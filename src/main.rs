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

fn display_game_message(stdout: &mut io::Stdout, y_pos: u16, message: &str) -> io::Result<()> {
    stdout.queue(MoveTo(0, y_pos))?;
    let terminal_width = terminal::size().map(|(cols, _)| cols).unwrap_or(80);
    write!(stdout, "{:<width$}", "", width = terminal_width as usize)?;
    stdout.queue(MoveTo(0, y_pos))?;

    let formatted_message = format_model_output(message)?;
    writeln!(stdout, "{}", formatted_message)?;
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
        Ok(rules) => format!("# Welcome to Checkers!\n\n{}", rules),
        Err(AIError::NoApiKey) => String::from(messages::WELCOME_MESSAGE_NO_API),
        Err(e) => {
            eprintln!("Failed to get checkers story from AI: {}", e);
            String::from(messages::WELCOME_MESSAGE_ERROR)
        }
    };

    let formatted_message = format_model_output(&welcome_message)?;

    write!(io::stdout(), "{}\n\n", formatted_message)?;
    io::stdout().flush()?;

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
                                    display_game_message(&mut stdout, board_bottom_y + MSG_Y_OFFSET_PLAYER_ERROR, &format!("{} {}", messages::ERROR_PREFIX, e))?;
                                }
                                needs_render = true;
                            }
                            Some(_) => {
                                let move_result = game.make_move(row, col);
                                if let Err(e) = move_result {
                                    display_game_message(&mut stdout, board_bottom_y + MSG_Y_OFFSET_PLAYER_ERROR, &format!("{} {}", messages::ERROR_PREFIX, e))?;
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

        if game.current_player == PieceColor::Black && !game.is_game_over {
            display_game_message(&mut stdout, board_bottom_y + MSG_Y_OFFSET_AI_STATUS, "Black (AI) is thinking...")?;

            match get_ai_move(&game).await {
                Ok(((from_row, from_col), (to_row, to_col))) => {
                    let select_result = game.select_piece(from_row, from_col);
                    if let Err(e) = select_result {
                        display_game_message(&mut stdout, board_bottom_y + MSG_Y_OFFSET_AI_STATUS, &format!("{} AI failed to select piece: {}. Skipping turn.", messages::ERROR_PREFIX, e))?;
                        game.switch_player();
                    } else {
                        let move_result = game.make_move(to_row, to_col);
                        if let Err(e) = move_result {
                            display_game_message(&mut stdout, board_bottom_y + MSG_Y_OFFSET_AI_STATUS, &format!("{} AI failed to make move: {}. Skipping turn.", messages::ERROR_PREFIX, e))?;
                            if game.current_player == PieceColor::Black { game.switch_player(); }
                        } else {
                            check_and_set_game_over(&mut game);
                        }
                    }
                }
                Err(ai_error) => {
                    display_game_message(&mut stdout, board_bottom_y + MSG_Y_OFFSET_AI_STATUS, &format!("{} AI error: {}. Skipping turn.", messages::ERROR_PREFIX, ai_error))?;
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
