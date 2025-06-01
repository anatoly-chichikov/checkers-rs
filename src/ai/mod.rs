pub mod error;
pub mod genai_client;
pub mod hint;
pub mod ui;

pub use error::AIError;
pub use genai_client::explain_rules;
pub use genai_client::get_ai_move;
