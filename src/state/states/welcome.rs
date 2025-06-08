use crate::state::{GameSession, State, StateTransition, StateType, ViewData};
use crossterm::event::{KeyCode, KeyEvent};

pub struct WelcomeState;

#[derive(Clone)]
pub struct WelcomeContent {
    pub did_you_know: String,
    pub tip_of_the_day: String,
    pub todays_challenge: String,
}

impl Default for WelcomeState {
    fn default() -> Self {
        Self
    }
}

impl WelcomeState {
    pub fn new() -> Self {
        Self
    }
}

impl State for WelcomeState {
    fn handle_input(&mut self, _session: &mut GameSession, key: KeyEvent) -> StateTransition {
        match key.code {
            KeyCode::Enter => {
                // Transition to PlayingState
                StateTransition::To(Box::new(super::PlayingState::new()))
            }
            KeyCode::Esc | KeyCode::Char('q') => StateTransition::Exit,
            _ => StateTransition::None,
        }
    }

    fn on_enter(&mut self, _session: &mut GameSession) {
        // Welcome content is now initialized in Application
    }

    fn on_exit(&mut self, _session: &mut GameSession) {
        // Nothing to do on exit
    }

    fn get_view_data<'a>(&self, session: &'a GameSession) -> ViewData<'a> {
        ViewData {
            board: &session.game.board,
            current_player: session.game.current_player,
            cursor_pos: session.ui_state.cursor_pos,
            selected_piece: None,
            possible_moves: &[],
            pieces_with_captures: Vec::new(),
            status_message: "Welcome to Checkers!".to_string(),
            show_ai_thinking: false,
            error_message: None,
            last_move: None,
            hint: None,
            is_game_over: false,
            welcome_content: session.welcome_content.as_ref().map(|content| {
                (
                    content.did_you_know.as_str(),
                    content.tip_of_the_day.as_str(),
                    content.todays_challenge.as_str(),
                )
            }),
        }
    }

    fn state_type(&self) -> StateType {
        StateType::Welcome
    }
}
