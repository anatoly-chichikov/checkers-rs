pub mod gemini_client;
pub mod error;
pub mod models;
pub mod ui;

pub use gemini_client::explain_rules;
pub use gemini_client::get_ai_move;
pub use error::AIError;
