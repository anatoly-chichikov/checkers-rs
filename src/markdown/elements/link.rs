use crossterm::style::Color;
use std::io;

use super::Element;
use crate::markdown::style::StyleWriter;

pub struct Link {
    text: String,
    /// The URL is stored for potential future use (e.g., making links clickable in terminal)
    /// but is currently not rendered in the terminal output
    #[allow(dead_code)]
    url: String,
}

impl Link {
    pub fn new(text: String, url: String) -> Self {
        Self { text, url }
    }
}

impl Element for Link {
    fn render(&self, writer: &mut StyleWriter) -> io::Result<()> {
        // We only render the link text, not the URL
        writer.write_colored(&self.text, Color::Blue)?;
        Ok(())
    }
} 