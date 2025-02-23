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
    pub fn new(text: String) -> Self {
        Self {
            text,
            is_bold: false,
            is_italic: false,
        }
    }

    pub fn with_formatting(text: String, is_bold: bool, is_italic: bool) -> Self {
        let mut item = Self::new(text);
        item.is_bold = is_bold;
        item.is_italic = is_italic;
        item
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

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::style::{ResetColor, SetForegroundColor};

    #[test]
    fn test_plain_list_item() -> io::Result<()> {
        let item = ListItem::new("Plain item".to_string());
        let mut writer = StyleWriter::new();
        item.render(&mut writer)?;
        let result = writer.into_string()?;

        assert_eq!(
            result,
            format!(
                "{}Plain item{}\n",
                SetForegroundColor(Color::White),
                ResetColor
            )
        );
        Ok(())
    }

    #[test]
    fn test_bold_list_item() -> io::Result<()> {
        let item = ListItem::with_formatting("Bold item".to_string(), true, false);
        let mut writer = StyleWriter::new();
        item.render(&mut writer)?;
        let result = writer.into_string()?;

        assert_eq!(
            result,
            format!(
                "{}Bold item{}\n",
                SetForegroundColor(Color::Green),
                ResetColor
            )
        );
        Ok(())
    }

    #[test]
    fn test_italic_list_item() -> io::Result<()> {
        let item = ListItem::with_formatting("Italic item".to_string(), false, true);
        let mut writer = StyleWriter::new();
        item.render(&mut writer)?;
        let result = writer.into_string()?;

        assert_eq!(
            result,
            format!(
                "{}Italic item{}\n",
                SetForegroundColor(Color::Cyan),
                ResetColor
            )
        );
        Ok(())
    }

    #[test]
    fn test_bold_and_italic_list_item() -> io::Result<()> {
        let item = ListItem::with_formatting("Bold and italic".to_string(), true, true);
        let mut writer = StyleWriter::new();
        item.render(&mut writer)?;
        let result = writer.into_string()?;

        // Bold takes precedence over italic
        assert_eq!(
            result,
            format!(
                "{}Bold and italic{}\n",
                SetForegroundColor(Color::Green),
                ResetColor
            )
        );
        Ok(())
    }

    #[test]
    fn test_list_item_with_whitespace() -> io::Result<()> {
        let item = ListItem::new("  Item with spaces  ".to_string());
        let mut writer = StyleWriter::new();
        item.render(&mut writer)?;
        let result = writer.into_string()?;

        assert_eq!(
            result,
            format!(
                "{}  Item with spaces  {}\n",
                SetForegroundColor(Color::White),
                ResetColor
            )
        );
        Ok(())
    }
}
