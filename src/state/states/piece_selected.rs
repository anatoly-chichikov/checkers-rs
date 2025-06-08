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
    fn handle_input(&mut self, session: &mut GameSession, key: KeyEvent) -> StateTransition {
        match key.code {
            KeyCode::Up => {
                session.ui_state.move_cursor_up();
                StateTransition::None
            }
            KeyCode::Down => {
                session.ui_state.move_cursor_down(7);
                StateTransition::None
            }
            KeyCode::Left => {
                session.ui_state.move_cursor_left();
                StateTransition::None
            }
            KeyCode::Right => {
                session.ui_state.move_cursor_right(7);
                StateTransition::None
            }
            KeyCode::Esc => StateTransition::To(Box::new(super::PlayingState::new())),
            KeyCode::Char(' ') | KeyCode::Enter => {
                let cursor = session.ui_state.cursor_pos;

                // Deselect if same piece
                if cursor == self.selected_pos {
                    return StateTransition::To(Box::new(super::PlayingState::new()));
                }

                // Try move
                if session.ui_state.possible_moves.contains(&cursor) {
                    match session.try_multicapture_move(cursor.0, cursor.1) {
                        Ok((continue_capture, _positions)) => {
                            // Clear hint after player move
                            session.hint = None;

                            // Check if multi-capture continues
                            if continue_capture {
                                StateTransition::To(Box::new(super::MultiCaptureState::new(cursor)))
                            } else if session.game.check_winner().is_some() {
                                session.game.is_game_over = true;
                                StateTransition::To(Box::new(super::GameOverState::new(
                                    session.game.check_winner(),
                                )))
                            } else if session.game.is_stalemate() {
                                // If current player has no moves, the other player wins
                                session.game.is_game_over = true;
                                StateTransition::To(Box::new(super::GameOverState::new(Some(
                                    session.game.current_player.opposite(),
                                ))))
                            } else {
                                StateTransition::To(Box::new(super::PlayingState::new()))
                            }
                        }
                        Err(_) => StateTransition::None,
                    }
                } else {
                    StateTransition::None
                }
            }
            _ => StateTransition::None,
        }
    }

    fn on_enter(&mut self, session: &mut GameSession) {
        // Use GameSession method instead of direct field access
        let _ = session.select_piece(self.selected_pos.0, self.selected_pos.1);
    }

    fn on_exit(&mut self, session: &mut GameSession) {
        if session.is_piece_selected(self.selected_pos) {
            session.deselect_piece();
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
            last_move: session.game.move_history.get_last_move(),
            hint: session.hint.as_ref(),
            is_game_over: false,
            welcome_content: None,
        }
    }

    fn state_type(&self) -> StateType {
        StateType::PieceSelected
    }
}
