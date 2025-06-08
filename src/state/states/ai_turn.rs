use crate::ai::genai_client::get_ai_move;
use crate::core::piece::Color;
use crate::state::{GameSession, State, StateTransition, StateType, ViewData};
use crossterm::event::KeyEvent;

#[derive(Default)]
pub struct AITurnState {
    move_requested: bool,
}

impl AITurnState {
    pub fn new() -> Self {
        Self::default()
    }
}

impl State for AITurnState {
    fn handle_input(&mut self, session: &mut GameSession, _key: KeyEvent) -> StateTransition {
        // Make AI move if not done yet
        if !self.move_requested {
            self.move_requested = true;

            // Check if we should use real AI or test fallback
            let use_real_ai = std::env::var("AI_TEST_MODE").is_err()
                && std::env::var("GEMINI_API_KEY").is_ok()
                && std::env::var("GEMINI_MODEL").is_ok()
                && tokio::runtime::Handle::try_current().is_ok();

            if use_real_ai {
                // Use real AI with async calls
                let ai_result = tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current().block_on(get_ai_move(&session.game))
                });

                match ai_result {
                    Ok(((from_row, from_col), (to_row, to_col))) => {
                        match session.game.make_move(from_row, from_col, to_row, to_col) {
                            Ok(_) => {
                                session.ai_state.clear_error();

                                // Update hint after AI move
                                if let Some(ref provider) = session.hint_provider {
                                    if session.game.current_player == Color::White
                                        && !session.game.is_game_over
                                    {
                                        let hint_result = tokio::task::block_in_place(|| {
                                            tokio::runtime::Handle::current().block_on(
                                                provider.get_hint(
                                                    &session.game.board,
                                                    Color::White,
                                                    &session.game.move_history,
                                                ),
                                            )
                                        });

                                        match hint_result {
                                            Ok(hint_text) => {
                                                session.hint =
                                                    Some(crate::ai::Hint { hint: hint_text });
                                            }
                                            Err(_) => {
                                                session.hint = None;
                                            }
                                        }
                                    }
                                }

                                // Check for game over
                                if session.game.check_winner().is_some() {
                                    session.game.is_game_over = true;
                                    return StateTransition::To(Box::new(
                                        super::GameOverState::new(session.game.check_winner()),
                                    ));
                                } else if session.game.is_stalemate() {
                                    // If current player has no moves, the other player wins
                                    session.game.is_game_over = true;
                                    return StateTransition::To(Box::new(
                                        super::GameOverState::new(Some(
                                            session.game.current_player.opposite(),
                                        )),
                                    ));
                                }

                                // Transition back to playing state
                                return StateTransition::To(Box::new(super::PlayingState::new()));
                            }
                            Err(e) => {
                                session
                                    .ai_state
                                    .set_error(format!("AI failed to move: {}", e));
                                session.game.switch_player();
                                return StateTransition::To(Box::new(super::PlayingState::new()));
                            }
                        }
                    }
                    Err(e) => {
                        session.ai_state.set_error(format!("AI Error: {}", e));
                        session.game.switch_player();
                        return StateTransition::To(Box::new(super::PlayingState::new()));
                    }
                }
            } else {
                // Fallback to simple AI for tests
                use crate::core::game_logic;

                let all_moves =
                    game_logic::get_all_valid_moves_for_player(&session.game.board, Color::Black);

                if all_moves.is_empty() {
                    // No valid moves - game over
                    session.game.is_game_over = true;
                    return StateTransition::To(Box::new(super::GameOverState::new(Some(
                        Color::White,
                    ))));
                }

                // Separate captures from regular moves
                let captures: Vec<_> = all_moves
                    .iter()
                    .filter(|(_, _, is_capture)| *is_capture)
                    .cloned()
                    .collect();

                let moves_to_consider = if !captures.is_empty() {
                    captures // Must make a capture if available
                } else {
                    all_moves
                };

                if !moves_to_consider.is_empty() {
                    // Pick first available move (simple AI)
                    let ((from_row, from_col), (to_row, to_col), _) = moves_to_consider[0];

                    match session.game.make_move(from_row, from_col, to_row, to_col) {
                        Ok(_) => {
                            session.ai_state.clear_error();

                            // Check for game over
                            if session.game.check_winner().is_some() {
                                session.game.is_game_over = true;
                                return StateTransition::To(Box::new(super::GameOverState::new(
                                    session.game.check_winner(),
                                )));
                            } else if session.game.is_stalemate() {
                                // If current player has no moves, the other player wins
                                session.game.is_game_over = true;
                                return StateTransition::To(Box::new(super::GameOverState::new(
                                    Some(session.game.current_player.opposite()),
                                )));
                            }

                            return StateTransition::To(Box::new(super::PlayingState::new()));
                        }
                        Err(e) => {
                            session.ai_state.set_error(format!("AI error: {}", e));
                            return StateTransition::To(Box::new(super::PlayingState::new()));
                        }
                    }
                }
            }
        }

        StateTransition::None
    }

    fn on_enter(&mut self, session: &mut GameSession) {
        session.ai_state.start_thinking();
    }

    fn on_exit(&mut self, session: &mut GameSession) {
        session.ai_state.stop_thinking();
    }

    fn get_view_data<'a>(&self, session: &'a GameSession) -> ViewData<'a> {
        ViewData {
            board: &session.game.board,
            current_player: session.game.current_player,
            cursor_pos: session.ui_state.cursor_pos,
            selected_piece: None,
            possible_moves: &[],
            pieces_with_captures: Vec::new(),
            status_message: "AI is thinking...".to_string(),
            show_ai_thinking: true,
            error_message: session.ai_state.last_error.as_deref(),
            is_simple_ai: std::env::var("GEMINI_API_KEY").is_err() || std::env::var("GEMINI_MODEL").is_err(),
            hint: session.hint.as_ref(),
            is_game_over: false,
            welcome_content: None,
        }
    }

    fn state_type(&self) -> StateType {
        StateType::AITurn
    }
}
