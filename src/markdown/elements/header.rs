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

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::style::{ResetColor, SetForegroundColor};

    #[test]
    fn test_header_level_1() -> io::Result<()> {
        let header = Header::new(1, "Title".to_string());
        let mut writer = StyleWriter::new();
        header.render(&mut writer)?;
        let result = writer.into_string()?;

        assert_eq!(
            result,
            format!(
                "{}TITLE{}\n",
                SetForegroundColor(Color::Magenta),
                ResetColor
            )
        );
        Ok(())
    }

    #[test]
    fn test_header_level_2() -> io::Result<()> {
        let header = Header::new(2, "Subtitle".to_string());
        let mut writer = StyleWriter::new();
        header.render(&mut writer)?;
        let result = writer.into_string()?;

        assert_eq!(
            result,
            format!(
                "{}  SUBTITLE{}\n",
                SetForegroundColor(Color::Magenta),
                ResetColor
            )
        );
        Ok(())
    }

    #[test]
    fn test_header_with_whitespace() -> io::Result<()> {
        let header = Header::new(1, "  Title with spaces  ".to_string());
        let mut writer = StyleWriter::new();
        header.render(&mut writer)?;
        let result = writer.into_string()?;

        assert_eq!(
            result,
            format!(
                "{}TITLE WITH SPACES{}\n",
                SetForegroundColor(Color::Magenta),
                ResetColor
            )
        );
        Ok(())
    }

    #[test]
    fn test_header_invalid_level() -> io::Result<()> {
        let header = Header::new(7, "Invalid level".to_string());
        let mut writer = StyleWriter::new();
        header.render(&mut writer)?;
        let result = writer.into_string()?;

        assert_eq!(
            result,
            format!(
                "{}INVALID LEVEL{}\n",
                SetForegroundColor(Color::Magenta),
                ResetColor
            )
        );
        Ok(())
    }
}
