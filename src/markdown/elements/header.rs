use crossterm::style::Color;
use std::io;

use super::Element;
use crate::markdown::style::StyleWriter;

pub struct Header {
    level: u8,
    text: String,
}

impl Header {
    pub fn new(level: u8, text: String) -> Self {
        Self { level, text }
    }

    fn indentation(&self) -> &'static str {
        match self.level {
            1 => "",
            2 => "  ",
            3 => "    ",
            4 => "      ",
            5 => "        ",
            6 => "          ",
            _ => "",
        }
    }
}

impl Element for Header {
    fn render(&self, writer: &mut StyleWriter) -> io::Result<()> {
        let text = if self.level == 1 {
            self.text.trim().to_uppercase()
        } else {
            format!("{}{}", self.indentation(), self.text.trim().to_uppercase())
        };

        writer.write_colored(&text, Color::Magenta)?;
        writer.write_plain("\n")?;
        Ok(())
    }
}
