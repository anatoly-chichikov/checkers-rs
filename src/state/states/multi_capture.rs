use crate::state::{GameSession, State, StateTransition, StateType, ViewData};
use crossterm::event::{KeyCode, KeyEvent};

pub struct MultiCaptureState {
    capturing_piece: (usize, usize),
}

impl MultiCaptureState {
    pub fn new(capturing_piece: (usize, usize)) -> Self {
        Self { capturing_piece }
    }
}

impl State for MultiCaptureState {
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
            KeyCode::Char(' ') | KeyCode::Enter => {
                let cursor = session.ui_state.cursor_pos;
                if session.ui_state.possible_moves.contains(&cursor) {
                    match session.try_multicapture_move(cursor.0, cursor.1) {
                        Ok((continue_capture, _positions)) => {
                            // Check if capture continues
                            if continue_capture {
                                self.capturing_piece = cursor;
                                self.on_enter(session); // Update possible moves
                                StateTransition::None
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
        session
            .ui_state
            .select_piece(self.capturing_piece, &session.game.board);
    }

    fn on_exit(&mut self, session: &mut GameSession) {
        session.ui_state.clear_selection();
    }

    fn get_view_data<'a>(&self, session: &'a GameSession) -> ViewData<'a> {
        ViewData {
            board: &session.game.board,
            current_player: session.game.current_player,
            cursor_pos: session.ui_state.cursor_pos,
            selected_piece: Some(self.capturing_piece),
            possible_moves: &session.ui_state.possible_moves,
            pieces_with_captures: Vec::new(),
            status_message: "You must continue capturing!".to_string(),
            show_ai_thinking: false,
            error_message: None,
            is_simple_ai: std::env::var("GEMINI_API_KEY").is_err() || std::env::var("GEMINI_MODEL").is_err(),
            last_move: session.game.move_history.get_last_move(),
            hint: session.hint.as_ref(),
            is_game_over: false,
            welcome_content: None,
        }
    }

    fn state_type(&self) -> StateType {
        StateType::MultiCapture
    }
}
