pub mod ai_state;
pub mod game_session;
pub mod machine;
pub mod transition;
pub mod ui_state;
pub mod view_data;

pub mod states;

pub use game_session::GameSession;
pub use machine::{State, StateMachine};
pub use transition::StateTransition;
pub use view_data::ViewData;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateType {
    Welcome,
    Playing,
    PieceSelected,
    AITurn,
    MultiCapture,
    GameOver,
}
