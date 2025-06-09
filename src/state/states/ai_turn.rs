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

    fn with_move_requested(&self) -> Self {
        Self {
            move_requested: true,
        }
    }
}

impl State for AITurnState {
    fn handle_input(
        &self,
        session: &GameSession,
        _key: KeyEvent,
    ) -> (GameSession, StateTransition) {
        // Make AI move if not done yet
        if !self.move_requested {
            // Start thinking
            let mut new_session = session.clone();
            new_session.ai_state = new_session.ai_state.start_thinking();

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
                        match new_session
                            .game
                            .make_move(from_row, from_col, to_row, to_col)
                        {
                            Ok((updated_game, _)) => {
                                new_session.game = updated_game;
                                new_session.ai_state = new_session.ai_state.clear_error();

                                // Update hint after AI move
                                if let Some(ref provider) = new_session.hint_provider {
                                    if session.game.current_player == Color::White
                                        && !session.game.is_game_over
                                    {
                                        let hint_result = tokio::task::block_in_place(|| {
                                            tokio::runtime::Handle::current().block_on(
                                                provider.get_hint(
                                                    &new_session.game.board,
                                                    Color::White,
                                                    &new_session.game.move_history,
                                                ),
                                            )
                                        });

                                        match hint_result {
                                            Ok(hint_text) => {
                                                new_session.hint =
                                                    Some(crate::ai::Hint { hint: hint_text });
                                            }
                                            Err(_) => {
                                                new_session.hint = None;
                                            }
                                        }
                                    }
                                }

                                // Check for game over
                                let winner = new_session.game.check_winner();
                                if winner.is_some() {
                                    new_session.game.is_game_over = true;
                                    return (
                                        new_session,
                                        StateTransition::To(Box::new(super::GameOverState::new(
                                            winner,
                                        ))),
                                    );
                                } else if new_session.game.is_stalemate() {
                                    // If current player has no moves, the other player wins
                                    let winner = Some(new_session.game.current_player.opposite());
                                    new_session.game.is_game_over = true;
                                    return (
                                        new_session,
                                        StateTransition::To(Box::new(super::GameOverState::new(
                                            winner,
                                        ))),
                                    );
                                }

                                // Transition back to playing state
                                return (
                                    new_session,
                                    StateTransition::To(Box::new(super::PlayingState::new())),
                                );
                            }
                            Err(e) => {
                                new_session.ai_state = new_session
                                    .ai_state
                                    .set_error(format!("AI failed to move: {}", e));
                                new_session.game = new_session.game.with_switched_player();
                                return (
                                    new_session,
                                    StateTransition::To(Box::new(super::PlayingState::new())),
                                );
                            }
                        }
                    }
                    Err(e) => {
                        new_session.ai_state =
                            new_session.ai_state.set_error(format!("AI Error: {}", e));
                        new_session.game = new_session.game.with_switched_player();
                        return (
                            new_session,
                            StateTransition::To(Box::new(super::PlayingState::new())),
                        );
                    }
                }
            } else {
                // Fallback to simple AI for tests
                use crate::core::game_logic;

                let all_moves = game_logic::get_all_valid_moves_for_player(
                    &new_session.game.board,
                    Color::Black,
                );

                if all_moves.is_empty() {
                    // No valid moves - game over
                    new_session.game.is_game_over = true;
                    return (
                        new_session,
                        StateTransition::To(Box::new(super::GameOverState::new(Some(
                            Color::White,
                        )))),
                    );
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

                    match new_session
                        .game
                        .make_move(from_row, from_col, to_row, to_col)
                    {
                        Ok((updated_game, _)) => {
                            new_session.game = updated_game;
                            new_session.ai_state = new_session.ai_state.clear_error();

                            // Check for game over
                            let winner = new_session.game.check_winner();
                            if winner.is_some() {
                                new_session.game.is_game_over = true;
                                return (
                                    new_session,
                                    StateTransition::To(Box::new(super::GameOverState::new(
                                        winner,
                                    ))),
                                );
                            } else if new_session.game.is_stalemate() {
                                // If current player has no moves, the other player wins
                                let winner = Some(new_session.game.current_player.opposite());
                                new_session.game.is_game_over = true;
                                return (
                                    new_session,
                                    StateTransition::To(Box::new(super::GameOverState::new(
                                        winner,
                                    ))),
                                );
                            }

                            return (
                                new_session,
                                StateTransition::To(Box::new(super::PlayingState::new())),
                            );
                        }
                        Err(e) => {
                            new_session.ai_state =
                                new_session.ai_state.set_error(format!("AI error: {}", e));
                            return (
                                new_session,
                                StateTransition::To(Box::new(super::PlayingState::new())),
                            );
                        }
                    }
                }
            }

            // Return with move requested state to prevent re-execution
            return (
                new_session,
                StateTransition::To(Box::new(self.with_move_requested())),
            );
        }

        (session.clone(), StateTransition::None)
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
            is_simple_ai: std::env::var("GEMINI_API_KEY").is_err()
                || std::env::var("GEMINI_MODEL").is_err(),
            hint: session.hint.as_ref(),
            is_game_over: false,
            welcome_content: None,
        }
    }

    fn state_type(&self) -> StateType {
        StateType::AITurn
    }
}
