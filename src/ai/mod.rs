pub mod error;
pub mod gemini_client;
pub mod models;
pub mod ui;

pub use error::AIError;
pub use gemini_client::explain_rules;
pub use gemini_client::get_ai_move;
