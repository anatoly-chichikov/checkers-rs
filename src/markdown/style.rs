use crossterm::style::{Color, ResetColor, SetForegroundColor};
use std::io::{self, Write};

#[derive(Clone)]
pub struct StyleWriter {
    output: Vec<u8>,
}

impl StyleWriter {
    pub fn new() -> Self {
        Self { output: Vec::new() }
    }

    pub fn write_colored(&mut self, text: &str, color: Color) -> io::Result<()> {
        write!(self.output, "{}", SetForegroundColor(color))?;
        write!(self.output, "{}", text)?;
        write!(self.output, "{}", ResetColor)?;
        Ok(())
    }

    pub fn write_plain(&mut self, text: &str) -> io::Result<()> {
        write!(self.output, "{}", text)
    }

    pub fn into_string(self) -> io::Result<String> {
        String::from_utf8(self.output)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
} 