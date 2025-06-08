pub mod welcome;
pub mod playing;
pub mod piece_selected;
pub mod ai_turn;
pub mod multi_capture;
pub mod game_over;

pub use welcome::{WelcomeState, WelcomeContent};
pub use playing::PlayingState;
pub use piece_selected::PieceSelectedState;
pub use ai_turn::AITurnState;
pub use multi_capture::MultiCaptureState;
pub use game_over::GameOverState;