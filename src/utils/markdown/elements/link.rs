use crossterm::style::Color;
use std::io;

use super::Element;
use crate::utils::markdown::style::StyleWriter;

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
        writer.write_colored(&self.text, Color::Blue)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::style::{ResetColor, SetForegroundColor};

    #[test]
    fn test_link_render() -> io::Result<()> {
        let link = Link::new("Click here".to_string(), "https://example.com".to_string());
        let mut writer = StyleWriter::new();
        link.render(&mut writer)?;
        let result = writer.into_string()?;

        assert_eq!(
            result,
            format!(
                "{}Click here{}",
                SetForegroundColor(Color::Blue),
                ResetColor
            )
        );
        Ok(())
    }

    #[test]
    fn test_link_with_empty_url() -> io::Result<()> {
        let link = Link::new("Text only".to_string(), "".to_string());
        let mut writer = StyleWriter::new();
        link.render(&mut writer)?;
        let result = writer.into_string()?;

        assert_eq!(
            result,
            format!("{}Text only{}", SetForegroundColor(Color::Blue), ResetColor)
        );
        Ok(())
    }

    #[test]
    fn test_link_with_spaces() -> io::Result<()> {
        let link = Link::new("  Link with spaces  ".to_string(), "url".to_string());
        let mut writer = StyleWriter::new();
        link.render(&mut writer)?;
        let result = writer.into_string()?;

        assert_eq!(
            result,
            format!(
                "{}  Link with spaces  {}",
                SetForegroundColor(Color::Blue),
                ResetColor
            )
        );
        Ok(())
    }
}
