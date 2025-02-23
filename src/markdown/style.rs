use crossterm::style::{Color, ResetColor, SetForegroundColor};
use std::io::{self, Write};

#[derive(Clone)]
pub struct StyleWriter {
    output: Vec<u8>,
    current_color: Option<Color>,
}

impl Default for StyleWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl StyleWriter {
    pub fn new() -> Self {
        Self {
            output: Vec::new(),
            current_color: None,
        }
    }

    pub fn write_colored(&mut self, text: &str, color: Color) -> io::Result<()> {
        if self.current_color != Some(color) {
            write!(self.output, "{}", SetForegroundColor(color))?;
            self.current_color = Some(color);
        }
        write!(self.output, "{}", text)?;
        Ok(())
    }

    pub fn write_plain(&mut self, text: &str) -> io::Result<()> {
        if self.current_color.is_some() {
            write!(self.output, "{}", ResetColor)?;
            self.current_color = None;
        }
        write!(self.output, "{}", text)
    }

    pub fn into_string(mut self) -> io::Result<String> {
        if self.current_color.is_some() {
            write!(self.output, "{}", ResetColor)?;
        }
        String::from_utf8(self.output).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}
