use std::io;

use super::{
    elements::{
        code::CodeBlock, emphasis::Emphasis, header::Header, link::Link, list::ListItem, Element,
    },
    style::StyleWriter,
};

// Trait for parsing markdown elements
pub trait ElementParser {
    fn try_parse(
        &self,
        chars: &mut std::iter::Peekable<std::str::Chars>,
    ) -> Option<Box<dyn Element>>;
}

// Parser for specific element types
pub struct HeaderParser;
pub struct ListItemParser;
pub struct CodeBlockParser;
pub struct LinkParser;
pub struct EmphasisParser;

impl ElementParser for HeaderParser {
    fn try_parse(
        &self,
        chars: &mut std::iter::Peekable<std::str::Chars>,
    ) -> Option<Box<dyn Element>> {
        if chars.peek() != Some(&'#') {
            return None;
        }

        chars.next(); // consume #
        let mut level = 1;
        while chars.peek() == Some(&'#') {
            level += 1;
            chars.next();
        }

        // Skip whitespace
        while chars.peek() == Some(&' ') {
            chars.next();
        }

        let mut text = String::new();
        while let Some(&next) = chars.peek() {
            if next == '\n' {
                chars.next();
                break;
            }
            text.push(chars.next().unwrap());
        }

        Some(Box::new(Header::new(level, text)))
    }
}

impl ElementParser for ListItemParser {
    fn try_parse(
        &self,
        chars: &mut std::iter::Peekable<std::str::Chars>,
    ) -> Option<Box<dyn Element>> {
        if chars.peek() != Some(&'-') {
            return None;
        }

        chars.next(); // consume -
        if chars.peek() != Some(&' ') {
            return None;
        }
        chars.next(); // consume space

        let mut text = String::new();
        let mut is_bold = false;
        let mut is_italic = false;

        while let Some(&next) = chars.peek() {
            if next == '\n' {
                chars.next();
                break;
            }
            match chars.next().unwrap() {
                '*' => {
                    if chars.peek() == Some(&'*') {
                        chars.next();
                        is_bold = !is_bold;
                    } else {
                        is_italic = !is_italic;
                    }
                }
                ch => text.push(ch),
            }
        }

        Some(Box::new(ListItem::with_formatting(
            text, is_bold, is_italic,
        )))
    }
}

impl ElementParser for CodeBlockParser {
    fn try_parse(
        &self,
        chars: &mut std::iter::Peekable<std::str::Chars>,
    ) -> Option<Box<dyn Element>> {
        if chars.peek() != Some(&'`') {
            return None;
        }

        chars.next(); // consume first `
        if chars.peek() == Some(&'`') {
            chars.next();
            if chars.peek() == Some(&'`') {
                chars.next();
                let mut content = String::new();
                while let Some(&next) = chars.peek() {
                    if next == '`' && chars.peek() == Some(&'`') {
                        chars.next();
                        chars.next();
                        if chars.peek() == Some(&'`') {
                            chars.next();
                            break;
                        }
                    }
                    content.push(chars.next().unwrap());
                }
                Some(Box::new(CodeBlock::new_block(content)))
            } else {
                None
            }
        } else {
            let mut content = String::new();
            while let Some(&next) = chars.peek() {
                if next == '`' {
                    chars.next();
                    break;
                }
                content.push(chars.next().unwrap());
            }
            Some(Box::new(CodeBlock::new_inline(content)))
        }
    }
}

impl ElementParser for LinkParser {
    fn try_parse(
        &self,
        chars: &mut std::iter::Peekable<std::str::Chars>,
    ) -> Option<Box<dyn Element>> {
        if chars.peek() != Some(&'[') {
            return None;
        }

        chars.next(); // consume [
        let mut text = String::new();
        let mut url = String::new();

        while let Some(&next) = chars.peek() {
            if next == ']' {
                chars.next();
                break;
            }
            text.push(chars.next().unwrap());
        }

        if chars.peek() == Some(&'(') {
            chars.next();
            while let Some(&next) = chars.peek() {
                if next == ')' {
                    chars.next();
                    break;
                }
                url.push(chars.next().unwrap());
            }
        }

        Some(Box::new(Link::new(text, url)))
    }
}

impl ElementParser for EmphasisParser {
    fn try_parse(
        &self,
        chars: &mut std::iter::Peekable<std::str::Chars>,
    ) -> Option<Box<dyn Element>> {
        if chars.peek() != Some(&'*') {
            return None;
        }

        chars.next(); // consume first *
        let mut text = String::new();
        let is_bold = chars.peek() == Some(&'*');

        if is_bold {
            chars.next(); // consume second *
        }

        while let Some(&next) = chars.peek() {
            match next {
                '*' => {
                    chars.next();
                    if chars.peek() == Some(&'*') {
                        chars.next();
                        if is_bold {
                            break; // End of bold
                        }
                        text.push_str("**"); // Preserve nested bold
                    } else {
                        if !is_bold {
                            break; // End of italic
                        }
                        text.push('*'); // Preserve nested italic
                    }
                }
                _ => text.push(chars.next().unwrap()),
            }
        }

        Some(Box::new(Emphasis::new(text, is_bold)))
    }
}

pub struct MarkdownRenderer {
    writer: StyleWriter,
    parsers: Vec<Box<dyn ElementParser>>,
}

impl Default for MarkdownRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl MarkdownRenderer {
    pub fn new() -> Self {
        let parsers: Vec<Box<dyn ElementParser>> = vec![
            Box::new(HeaderParser),
            Box::new(ListItemParser),
            Box::new(CodeBlockParser),
            Box::new(LinkParser),
            Box::new(EmphasisParser),
        ];

        Self {
            writer: StyleWriter::new(),
            parsers,
        }
    }

    pub fn render(&mut self, markdown: &str) -> io::Result<String> {
        let mut chars = markdown.chars().peekable();

        while let Some(&ch) = chars.peek() {
            let mut handled = false;

            // Try each parser in turn
            for parser in &self.parsers {
                if let Some(element) = parser.try_parse(&mut chars) {
                    element.render(&mut self.writer)?;
                    handled = true;
                    break;
                }
            }

            // If no parser handled it, treat as plain text
            if !handled {
                chars.next();
                self.writer.write_plain(&ch.to_string())?;
            }
        }

        self.writer.clone().into_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::style::{Color, ResetColor, SetForegroundColor};

    fn strip_color_codes(s: &str) -> String {
        // First remove the reset codes
        let without_reset = s.replace(&format!("{}", ResetColor), "");

        // Then remove all color codes
        without_reset
            .replace(&format!("{}", SetForegroundColor(Color::Magenta)), "")
            .replace(&format!("{}", SetForegroundColor(Color::White)), "")
            .replace(&format!("{}", SetForegroundColor(Color::Cyan)), "")
            .replace(&format!("{}", SetForegroundColor(Color::Yellow)), "")
            .replace(&format!("{}", SetForegroundColor(Color::Green)), "")
            .replace(&format!("{}", SetForegroundColor(Color::Blue)), "")
    }

    #[test]
    fn test_header_parser() -> io::Result<()> {
        let parser = HeaderParser;
        let mut chars = "# Header 1\n".chars().peekable();
        let element = parser.try_parse(&mut chars).unwrap();
        let mut writer = StyleWriter::new();
        element.render(&mut writer)?;
        let result = strip_color_codes(&writer.into_string()?);
        assert_eq!(result, "HEADER 1\n");
        Ok(())
    }

    #[test]
    fn test_list_item_parser() -> io::Result<()> {
        let parser = ListItemParser;
        let mut chars = "- Simple list item\n".chars().peekable();
        let element = parser.try_parse(&mut chars).unwrap();
        let mut writer = StyleWriter::new();
        element.render(&mut writer)?;
        let result = strip_color_codes(&writer.into_string()?);
        assert_eq!(result, "Simple list item\n");
        Ok(())
    }

    #[test]
    fn test_code_block_parser_inline() -> io::Result<()> {
        let parser = CodeBlockParser;
        let mut chars = "`inline code`".chars().peekable();
        let element = parser.try_parse(&mut chars).unwrap();
        let mut writer = StyleWriter::new();
        element.render(&mut writer)?;
        let result = strip_color_codes(&writer.into_string()?);
        assert_eq!(result, "inline code");
        Ok(())
    }

    #[test]
    fn test_code_block_parser_block() -> io::Result<()> {
        let parser = CodeBlockParser;
        let mut chars = "```\ncode block\n```".chars().peekable();
        let element = parser.try_parse(&mut chars).unwrap();
        let mut writer = StyleWriter::new();
        element.render(&mut writer)?;
        let result = strip_color_codes(&writer.into_string()?);
        assert_eq!(result, "\ncode block\n");
        Ok(())
    }

    #[test]
    fn test_link_parser() -> io::Result<()> {
        let parser = LinkParser;
        let mut chars = "[link text](https://example.com)".chars().peekable();
        let element = parser.try_parse(&mut chars).unwrap();
        let mut writer = StyleWriter::new();
        element.render(&mut writer)?;
        let result = strip_color_codes(&writer.into_string()?);
        assert_eq!(result, "link text");
        Ok(())
    }

    #[test]
    fn test_emphasis_parser_bold() -> io::Result<()> {
        let parser = EmphasisParser;
        let mut chars = "**bold text**".chars().peekable();
        let element = parser.try_parse(&mut chars).unwrap();
        let mut writer = StyleWriter::new();
        element.render(&mut writer)?;
        let result = strip_color_codes(&writer.into_string()?);
        assert_eq!(result, "bold text");
        Ok(())
    }

    #[test]
    fn test_emphasis_parser_italic() -> io::Result<()> {
        let parser = EmphasisParser;
        let mut chars = "*italic text*".chars().peekable();
        let element = parser.try_parse(&mut chars).unwrap();
        let mut writer = StyleWriter::new();
        element.render(&mut writer)?;
        let result = strip_color_codes(&writer.into_string()?);
        assert_eq!(result, "italic text");
        Ok(())
    }

    #[test]
    fn test_markdown_renderer_mixed() -> io::Result<()> {
        let mut renderer = MarkdownRenderer::new();
        let markdown = "# Title\n\n**Bold** and *italic* and `code`\n\n```\ncode block\n```\n\n- List item\n[Link](url)";
        let result = strip_color_codes(&renderer.render(markdown)?);

        assert!(result.contains("TITLE"));
        assert!(result.contains("Bold"));
        assert!(result.contains("italic"));
        assert!(result.contains("code"));
        assert!(result.contains("code block"));
        assert!(result.contains("List item"));
        assert!(result.contains("Link"));
        assert!(!result.contains("url"));
        Ok(())
    }

    #[test]
    fn test_markdown_renderer_nested_formatting() -> io::Result<()> {
        let mut renderer = MarkdownRenderer::new();
        let markdown = "- List with **bold** and *italic*\n";
        let result = strip_color_codes(&renderer.render(markdown)?);

        assert!(result.contains("List with bold and italic"));
        assert!(!result.contains('*'));
        assert!(!result.contains('-'));
        Ok(())
    }

    #[test]
    fn test_markdown_renderer_empty() -> io::Result<()> {
        let mut renderer = MarkdownRenderer::new();
        let result = renderer.render("")?;
        assert_eq!(strip_color_codes(&result), "");
        Ok(())
    }

    #[test]
    fn test_markdown_renderer_plain_text() -> io::Result<()> {
        let mut renderer = MarkdownRenderer::new();
        let markdown = "Just some plain text\nwith a newline";
        let result = strip_color_codes(&renderer.render(markdown)?);
        assert_eq!(result, "Just some plain text\nwith a newline");
        Ok(())
    }
}
