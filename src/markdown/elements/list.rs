use crossterm::style::Color;
use std::io;

use super::Element;
use crate::markdown::style::StyleWriter;

pub struct ListItem {
    text: String,
    is_bold: bool,
    is_italic: bool,
}

impl ListItem {
    #[allow(dead_code)]
    pub fn new(text: String) -> Self {
        Self {
            text,
            is_bold: false,
            is_italic: false,
        }
    }

    pub fn with_formatting(text: String, is_bold: bool, is_italic: bool) -> Self {
        Self {
            text,
            is_bold,
            is_italic,
        }
    }
}

impl Element for ListItem {
    fn render(&self, writer: &mut StyleWriter) -> io::Result<()> {
        let color = if self.is_bold {
            Color::Green
        } else if self.is_italic {
            Color::Cyan
        } else {
            Color::White
        };

        writer.write_colored(&self.text, color)?;
        writer.write_plain("\n")?;
        Ok(())
    }
} 