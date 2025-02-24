// Re-export public API
pub mod client;
pub mod error;
pub mod models;
pub mod ui;

pub use client::explain_rules;
pub use error::AIError;
