use crate::state::{GameSession, State, StateTransition, StateType, ViewData};
use crossterm::event::{KeyCode, KeyEvent};

pub struct PieceSelectedState {
    selected_pos: (usize, usize),
}

impl PieceSelectedState {
    pub fn new(selected_pos: (usize, usize)) -> Self {
        Self { selected_pos }
    }
}

impl State for PieceSelectedState {
    fn handle_input(&self, session: &GameSession, key: KeyEvent) -> (GameSession, StateTransition) {
        match key.code {
            KeyCode::Up => {
                let new_ui = session.ui_state.move_cursor_up();
                (session.with_ui_state(new_ui), StateTransition::None)
            }
            KeyCode::Down => {
                let new_ui = session.ui_state.move_cursor_down(7);
                (session.with_ui_state(new_ui), StateTransition::None)
            }
            KeyCode::Left => {
                let new_ui = session.ui_state.move_cursor_left();
                (session.with_ui_state(new_ui), StateTransition::None)
            }
            KeyCode::Right => {
                let new_ui = session.ui_state.move_cursor_right(7);
                (session.with_ui_state(new_ui), StateTransition::None)
            }
            KeyCode::Esc => {
                let deselected_session = session.deselect_piece();
                (
                    deselected_session,
                    StateTransition::To(Box::new(super::PlayingState::new())),
                )
            }
            KeyCode::Char(' ') | KeyCode::Enter => {
                let cursor = session.ui_state.cursor_pos;

                // Deselect if same piece
                if cursor == self.selected_pos {
                    let deselected_session = session.deselect_piece();
                    return (
                        deselected_session,
                        StateTransition::To(Box::new(super::PlayingState::new())),
                    );
                }

                // Try move
                if session.ui_state.possible_moves.contains(&cursor) {
                    match session.try_multicapture_move(cursor.0, cursor.1) {
                        Ok((mut updated_session, continue_capture, _positions)) => {
                            // Clear hint after player move
                            updated_session.hint = None;

                            // Check if multi-capture continues
                            if continue_capture {
                                // Already selected in updated_session
                                (
                                    updated_session,
                                    StateTransition::To(Box::new(super::MultiCaptureState::new(
                                        cursor,
                                    ))),
                                )
                            } else if updated_session.game.check_winner().is_some() {
                                let mut game_over_session = updated_session.clone();
                                game_over_session.game.is_game_over = true;
                                (
                                    game_over_session,
                                    StateTransition::To(Box::new(super::GameOverState::new(
                                        updated_session.game.check_winner(),
                                    ))),
                                )
                            } else if updated_session.game.is_stalemate() {
                                // If current player has no moves, the other player wins
                                let mut game_over_session = updated_session.clone();
                                game_over_session.game.is_game_over = true;
                                (
                                    game_over_session,
                                    StateTransition::To(Box::new(super::GameOverState::new(Some(
                                        updated_session.game.current_player.opposite(),
                                    )))),
                                )
                            } else {
                                (
                                    updated_session,
                                    StateTransition::To(Box::new(super::PlayingState::new())),
                                )
                            }
                        }
                        Err(_) => (session.clone(), StateTransition::None),
                    }
                } else {
                    (session.clone(), StateTransition::None)
                }
            }
            _ => (session.clone(), StateTransition::None),
        }
    }

    fn get_view_data<'a>(&self, session: &'a GameSession) -> ViewData<'a> {
        ViewData {
            board: &session.game.board,
            current_player: session.game.current_player,
            cursor_pos: session.ui_state.cursor_pos,
            selected_piece: Some(self.selected_pos),
            possible_moves: &session.ui_state.possible_moves,
            pieces_with_captures: Vec::new(),
            status_message: "Select a square to move to".to_string(),
            show_ai_thinking: false,
            error_message: None,
            is_simple_ai: std::env::var("GEMINI_API_KEY").is_err()
                || std::env::var("GEMINI_MODEL").is_err(),
            hint: session.hint.as_ref(),
            is_game_over: false,
            welcome_content: None,
        }
    }

    fn state_type(&self) -> StateType {
        StateType::PieceSelected
    }
}
