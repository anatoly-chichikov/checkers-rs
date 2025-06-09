pub mod error;
pub mod formatting;
pub mod genai_client;
pub mod hint;
pub mod ui;

pub use error::AIError;
pub use genai_client::explain_rules;
#[allow(unused_imports)]
pub use genai_client::get_ai_move;

// Simple hint structure for UI display
#[derive(Clone)]
pub struct Hint {
    pub hint: String,
}
