use crate::state::game_session::GameSession;
use crate::state::transition::StateTransition;
use crate::state::view_data::ViewData;
use crossterm::event::KeyEvent;

pub trait State {
    fn handle_input(&mut self, session: &mut GameSession, key: KeyEvent) -> StateTransition;

    fn on_enter(&mut self, session: &mut GameSession);

    fn on_exit(&mut self, session: &mut GameSession);

    fn get_view_data<'a>(&self, session: &'a GameSession) -> ViewData<'a>;

    fn name(&self) -> &'static str;
}

pub struct StateMachine {
    current_state: Box<dyn State>,
}

impl StateMachine {
    pub fn new(initial_state: Box<dyn State>) -> Self {
        Self {
            current_state: initial_state,
        }
    }

    pub fn handle_input(&mut self, session: &mut GameSession, key: KeyEvent) {
        let transition = self.current_state.handle_input(session, key);
        self.process_transition(session, transition);
    }

    pub fn get_view_data<'a>(&self, session: &'a GameSession) -> ViewData<'a> {
        self.current_state.get_view_data(session)
    }

    pub fn current_state_name(&self) -> &'static str {
        self.current_state.name()
    }

    fn process_transition(&mut self, session: &mut GameSession, transition: StateTransition) {
        match transition {
            StateTransition::None => {}
            StateTransition::To(mut new_state) => {
                self.current_state.on_exit(session);
                new_state.on_enter(session);
                self.current_state = new_state;
            }
            StateTransition::Exit => {
                // Handle exit in main loop
            }
        }
    }
}
