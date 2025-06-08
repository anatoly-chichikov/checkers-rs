use crate::core::piece::Color;
use crate::state::{GameSession, State, StateTransition, ViewData};
use crossterm::event::KeyEvent;

pub struct GameOverState {
    winner: Option<Color>,
}

impl GameOverState {
    pub fn new(winner: Option<Color>) -> Self {
        Self { winner }
    }
}

impl State for GameOverState {
    fn handle_input(&mut self, _session: &mut GameSession, _key: KeyEvent) -> StateTransition {
        // Exit on any key press
        StateTransition::Exit
    }

    fn on_enter(&mut self, _session: &mut GameSession) {
        // Nothing to do
    }

    fn on_exit(&mut self, _session: &mut GameSession) {
        // Nothing to do
    }

    fn get_view_data<'a>(&self, session: &'a GameSession) -> ViewData<'a> {
        let message = match self.winner {
            Some(Color::White) => "Black wins!".to_string(),
            Some(Color::Black) => "White wins!".to_string(),
            None => "Stalemate! No possible moves.".to_string(),
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
            last_move: session.game.move_history.get_last_move(),
            hint: None,
            is_game_over: true,
            welcome_content: None,
        }
    }

    fn name(&self) -> &'static str {
        "GameOverState"
    }
}
