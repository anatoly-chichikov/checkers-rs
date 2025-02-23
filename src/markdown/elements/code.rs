use crossterm::style::Color;
use std::io;

use super::Element;
use crate::markdown::style::StyleWriter;

pub struct CodeBlock {
    content: String,
    is_inline: bool,
}

impl CodeBlock {
    pub fn new_inline(content: String) -> Self {
        Self {
            content,
            is_inline: true,
        }
    }

    pub fn new_block(content: String) -> Self {
        Self {
            content,
            is_inline: false,
        }
    }
}

impl Element for CodeBlock {
    fn render(&self, writer: &mut StyleWriter) -> io::Result<()> {
        if self.is_inline {
            writer.write_colored(&self.content, Color::Green)?;
        } else {
            writer.write_plain("\n")?;
            writer.write_colored(&self.content.trim(), Color::Yellow)?;
            writer.write_plain("\n")?;
        }
        Ok(())
    }
} 