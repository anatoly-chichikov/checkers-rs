use crate::core::piece::Color;
use crate::state::{GameSession, State, StateTransition, StateType, ViewData};
use crossterm::event::{KeyCode, KeyEvent};

pub struct GameOverState {
    winner: Option<Color>,
}

impl GameOverState {
    pub fn new(winner: Option<Color>) -> Self {
        Self { winner }
    }
}

impl State for GameOverState {
    fn handle_input(&mut self, _session: &mut GameSession, key: KeyEvent) -> StateTransition {
        match key.code {
            KeyCode::Esc => StateTransition::Exit,
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
        let message = match self.winner {
            Some(Color::White) => "White wins! Press ESC to exit".to_string(),
            Some(Color::Black) => "Black wins! Press ESC to exit".to_string(),
            None => "Stalemate! No possible moves. Press ESC to exit".to_string(),
        };

        ViewData {
            board: &session.game.board,
            current_player: session.game.current_player,
            cursor_pos: session.ui_state.cursor_pos,
            selected_piece: None,
            possible_moves: &[],
            pieces_with_captures: Vec::new(),
            status_message: message,
            show_ai_thinking: false,
            error_message: None,
            is_simple_ai: std::env::var("GEMINI_API_KEY").is_err()
                || std::env::var("GEMINI_MODEL").is_err(),
            hint: None,
            is_game_over: true,
            welcome_content: None,
        }
    }

    fn state_type(&self) -> StateType {
        StateType::GameOver
    }
}
