use crate::core::piece::Color;
use crate::state::{GameSession, State, StateTransition, ViewData};
use crossterm::event::{KeyCode, KeyEvent};

pub struct PlayingState;

impl Default for PlayingState {
    fn default() -> Self {
        Self
    }
}

impl PlayingState {
    pub fn new() -> Self {
        Self
    }
}

impl State for PlayingState {
    fn handle_input(&mut self, session: &mut GameSession, key: KeyEvent) -> StateTransition {
        // Check if it's AI's turn
        if session.game.current_player == Color::Black {
            return StateTransition::To(Box::new(super::AITurnState::new()));
        }

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
                let cursor_pos = session.ui_state.cursor_pos;
                if let Some(piece) = session.game.board.get_piece(cursor_pos.0, cursor_pos.1) {
                    if piece.color == session.game.current_player
                        && session
                            .game
                            .validate_piece_selection(cursor_pos.0, cursor_pos.1)
                            .is_ok()
                    {
                        return StateTransition::To(Box::new(super::PieceSelectedState::new(
                            cursor_pos,
                        )));
                    }
                }
                StateTransition::None
            }
            KeyCode::Esc | KeyCode::Char('q') => StateTransition::Exit,
            _ => StateTransition::None,
        }
    }

    fn on_enter(&mut self, _session: &mut GameSession) {
        // Nothing to do
    }

    fn on_exit(&mut self, _session: &mut GameSession) {
        // Nothing to do
    }

    fn get_view_data<'a>(&self, session: &'a GameSession) -> ViewData<'a> {
        use crate::core::game_logic::get_pieces_with_captures;

        let pieces_with_captures = if session.game.has_captures_available() {
            get_pieces_with_captures(&session.game.board, session.game.current_player)
        } else {
            Vec::new()
        };

        let status_message = if !pieces_with_captures.is_empty() {
            format!(
                "{} must capture!",
                if session.game.current_player == Color::White {
                    "White"
                } else {
                    "Black"
                }
            )
        } else {
            format!(
                "{}'s turn",
                if session.game.current_player == Color::White {
                    "White"
                } else {
                    "Black"
                }
            )
        };

        ViewData {
            board: &session.game.board,
            current_player: session.game.current_player,
            cursor_pos: session.ui_state.cursor_pos,
            selected_piece: session.ui_state.selected_piece,
            possible_moves: &session.ui_state.possible_moves,
            pieces_with_captures,
            status_message,
            show_ai_thinking: false,
            error_message: None,
            last_move: session.game.move_history.get_last_move(),
            hint: session.hint.as_ref(),
            is_game_over: false,
            welcome_content: None,
        }
    }

    fn name(&self) -> &'static str {
        "PlayingState"
    }
}
