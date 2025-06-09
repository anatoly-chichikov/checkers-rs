use crate::state::game_session::GameSession;
use crate::state::transition::StateTransition;
use crate::state::view_data::ViewData;
use crate::state::StateType;
use crossterm::event::KeyEvent;

pub trait State {
    fn handle_input(&self, session: &GameSession, key: KeyEvent) -> (GameSession, StateTransition);

    fn get_view_data<'a>(&self, session: &'a GameSession) -> ViewData<'a>;

    fn state_type(&self) -> StateType;
}

pub struct StateMachine {
    current_state: Box<dyn State + 'static>,
}

impl StateMachine {
    pub fn new(initial_state: Box<dyn State + 'static>) -> Self {
        Self {
            current_state: initial_state,
        }
    }

    pub fn handle_input(
        &self,
        session: &GameSession,
        key: KeyEvent,
    ) -> (GameSession, StateTransition) {
        self.current_state.handle_input(session, key)
    }

    pub fn get_view_data<'a>(&self, session: &'a GameSession) -> ViewData<'a> {
        self.current_state.get_view_data(session)
    }

    pub fn current_state_type(&self) -> StateType {
        self.current_state.state_type()
    }

    pub fn process_transition(&mut self, transition: StateTransition) {
        match transition {
            StateTransition::None => {}
            StateTransition::To(new_state) => {
                self.current_state = new_state;
            }
            StateTransition::Exit => {
                // Handle exit in main loop
            }
        }
    }
}
