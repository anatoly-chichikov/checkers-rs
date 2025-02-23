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
            writer.write_colored(self.content.trim(), Color::Yellow)?;
            writer.write_plain("\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::style::{ResetColor, SetForegroundColor};

    #[test]
    fn test_inline_code() -> io::Result<()> {
        let code = CodeBlock::new_inline("let x = 42;".to_string());
        let mut writer = StyleWriter::new();
        code.render(&mut writer)?;
        let result = writer.into_string()?;

        assert_eq!(
            result,
            format!(
                "{}let x = 42;{}",
                SetForegroundColor(Color::Green),
                ResetColor
            )
        );
        Ok(())
    }

    #[test]
    fn test_block_code() -> io::Result<()> {
        let code = CodeBlock::new_block("fn main() {\n    println!(\"Hello\");\n}".to_string());
        let mut writer = StyleWriter::new();
        code.render(&mut writer)?;
        let result = writer.into_string()?;

        assert_eq!(
            result,
            format!(
                "\n{}fn main() {{\n    println!(\"Hello\");\n}}{}\n",
                SetForegroundColor(Color::Yellow),
                ResetColor
            )
        );
        Ok(())
    }

    #[test]
    fn test_block_code_with_whitespace() -> io::Result<()> {
        let code = CodeBlock::new_block("\n  code with spaces  \n".to_string());
        let mut writer = StyleWriter::new();
        code.render(&mut writer)?;
        let result = writer.into_string()?;

        assert_eq!(
            result,
            format!(
                "\n{}code with spaces{}\n",
                SetForegroundColor(Color::Yellow),
                ResetColor
            )
        );
        Ok(())
    }

    #[test]
    fn test_empty_code_block() -> io::Result<()> {
        let code = CodeBlock::new_block("".to_string());
        let mut writer = StyleWriter::new();
        code.render(&mut writer)?;
        let result = writer.into_string()?;

        assert_eq!(
            result,
            format!("\n{}{}\n", SetForegroundColor(Color::Yellow), ResetColor)
        );
        Ok(())
    }
}
