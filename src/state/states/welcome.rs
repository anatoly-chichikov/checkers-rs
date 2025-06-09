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
    fn handle_input(&self, session: &GameSession, key: KeyEvent) -> (GameSession, StateTransition) {
        let transition = match key.code {
            KeyCode::Enter => {
                // Transition to PlayingState
                StateTransition::To(Box::new(super::PlayingState::new()))
            }
            KeyCode::Esc | KeyCode::Char('q') => StateTransition::Exit,
            _ => StateTransition::None,
        };

        (session.clone(), transition)
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
            is_simple_ai: std::env::var("GEMINI_API_KEY").is_err()
                || std::env::var("GEMINI_MODEL").is_err(),
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
