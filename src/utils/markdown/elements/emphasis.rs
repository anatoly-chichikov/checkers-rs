use crossterm::style::Color;
use std::io;

use super::Element;
use crate::utils::markdown::style::StyleWriter;

pub struct Emphasis {
    text: String,
    is_bold: bool,
}

impl Emphasis {
    pub fn new(text: String, is_bold: bool) -> Self {
        Self { text, is_bold }
    }

    fn parse_nested(&self, text: &str, writer: &mut StyleWriter) -> io::Result<()> {
        let mut chars = text.chars().peekable();
        let mut current = String::new();
        let base_color = if self.is_bold {
            Color::Green
        } else {
            Color::Cyan
        };

        while let Some(ch) = chars.next() {
            match ch {
                '*' => {
                    if !current.is_empty() {
                        writer.write_colored(&current, base_color)?;
                        current.clear();
                    }

                    let mut nested_text = String::new();
                    let is_nested_bold = chars.peek() == Some(&'*');

                    if is_nested_bold {
                        chars.next(); // consume second *
                        while let Some(c) = chars.next() {
                            if c == '*' && chars.peek() == Some(&'*') {
                                chars.next();
                                break;
                            }
                            nested_text.push(c);
                        }
                        writer.write_colored(&nested_text, Color::Green)?;
                    } else {
                        for c in chars.by_ref() {
                            if c == '*' {
                                break;
                            }
                            nested_text.push(c);
                        }
                        writer.write_colored(&nested_text, Color::Cyan)?;
                    }
                }
                _ => current.push(ch),
            }
        }

        if !current.is_empty() {
            writer.write_colored(&current, base_color)?;
        }

        Ok(())
    }
}

impl Element for Emphasis {
    fn render(&self, writer: &mut StyleWriter) -> io::Result<()> {
        if self.text.contains('*') {
            self.parse_nested(&self.text, writer)?;
        } else {
            let color = if self.is_bold {
                Color::Green
            } else {
                Color::Cyan
            };
            writer.write_colored(&self.text, color)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::style::{ResetColor, SetForegroundColor};

    #[test]
    fn test_emphasis_render_bold() -> io::Result<()> {
        let emphasis = Emphasis::new("bold text".to_string(), true);
        let mut writer = StyleWriter::new();
        emphasis.render(&mut writer)?;
        let result = writer.into_string()?;

        assert_eq!(
            result,
            format!(
                "{}bold text{}",
                SetForegroundColor(Color::Green),
                ResetColor
            )
        );
        Ok(())
    }

    #[test]
    fn test_emphasis_render_italic() -> io::Result<()> {
        let emphasis = Emphasis::new("italic text".to_string(), false);
        let mut writer = StyleWriter::new();
        emphasis.render(&mut writer)?;
        let result = writer.into_string()?;

        assert_eq!(
            result,
            format!(
                "{}italic text{}",
                SetForegroundColor(Color::Cyan),
                ResetColor
            )
        );
        Ok(())
    }

    #[test]
    fn test_emphasis_render_nested() -> io::Result<()> {
        let emphasis = Emphasis::new("bold *italic* text".to_string(), true);
        let mut writer = StyleWriter::new();
        emphasis.render(&mut writer)?;
        let result = writer.into_string()?;

        assert_eq!(
            result,
            format!(
                "{}bold {}italic{} text{}",
                SetForegroundColor(Color::Green),
                SetForegroundColor(Color::Cyan),
                SetForegroundColor(Color::Green),
                ResetColor
            )
        );
        Ok(())
    }

    #[test]
    fn test_emphasis_render_nested_bold() -> io::Result<()> {
        let emphasis = Emphasis::new("italic **bold** text".to_string(), false);
        let mut writer = StyleWriter::new();
        emphasis.render(&mut writer)?;
        let result = writer.into_string()?;

        assert_eq!(
            result,
            format!(
                "{}italic {}bold{} text{}",
                SetForegroundColor(Color::Cyan),
                SetForegroundColor(Color::Green),
                SetForegroundColor(Color::Cyan),
                ResetColor
            )
        );
        Ok(())
    }
}
