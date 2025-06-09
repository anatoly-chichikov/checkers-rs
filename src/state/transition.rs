use crate::state::machine::State;

use std::fmt;

pub enum StateTransition {
    None,
    To(Box<dyn State + 'static>),
    Exit,
}

impl fmt::Debug for StateTransition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StateTransition::None => write!(f, "StateTransition::None"),
            StateTransition::To(_) => write!(f, "StateTransition::To(...)"),
            StateTransition::Exit => write!(f, "StateTransition::Exit"),
        }
    }
}

impl PartialEq for StateTransition {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (StateTransition::None, StateTransition::None) => true,
            (StateTransition::Exit, StateTransition::Exit) => true,
            (StateTransition::To(_), StateTransition::To(_)) => false, // Can't compare trait objects
            _ => false,
        }
    }
}
