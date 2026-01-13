pub mod config;
pub mod contract;
pub mod error;
pub mod formatting;
pub mod genai_client;
pub mod ui;

#[allow(unused_imports)]
pub use config::{load, AIConfig};
#[allow(unused_imports)]
pub use contract::{Agent, HintGiver, MoveChooser, RulesExplainer};
pub use error::AIError;
pub use genai_client::GeminiAI;

#[derive(Clone)]
pub struct Hint {
    pub hint: String,
}
