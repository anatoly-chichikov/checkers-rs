use crossterm::style::Color;
use std::io;

use super::{
    elements::{code::CodeBlock, header::Header, link::Link, list::ListItem, Element},
    style::StyleWriter,
};

pub struct MarkdownRenderer {
    writer: StyleWriter,
}

impl Default for MarkdownRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl MarkdownRenderer {
    pub fn new() -> Self {
        Self {
            writer: StyleWriter::new(),
        }
    }

    pub fn render(&mut self, markdown: &str) -> io::Result<String> {
        let mut chars = markdown.chars().peekable();

        while let Some(&ch) = chars.peek() {
            match ch {
                '#' => {
                    chars.next();
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

                    let header = Header::new(level, text);
                    header.render(&mut self.writer)?;
                }
                '-' => {
                    chars.next();
                    if chars.peek() == Some(&' ') {
                        chars.next();
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

                        let item = ListItem::with_formatting(text, is_bold, is_italic);
                        item.render(&mut self.writer)?;
                    }
                }
                '`' => {
                    chars.next();
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
                            let block = CodeBlock::new_block(content);
                            block.render(&mut self.writer)?;
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
                        let inline = CodeBlock::new_inline(content);
                        inline.render(&mut self.writer)?;
                    }
                }
                '[' => {
                    chars.next();
                    let mut text = String::new();
                    let mut url = String::new();

                    // Parse link text
                    while let Some(&next) = chars.peek() {
                        if next == ']' {
                            chars.next();
                            break;
                        }
                        text.push(chars.next().unwrap());
                    }

                    // Parse URL if present
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

                    let link = Link::new(text, url);
                    link.render(&mut self.writer)?;
                }
                '\n' => {
                    chars.next();
                    self.writer.write_plain("\n")?;
                }
                '*' => {
                    chars.next();
                    if chars.peek() == Some(&'*') {
                        chars.next();
                        let mut text = String::new();
                        let mut nested_italic = false;

                        while let Some(&next) = chars.peek() {
                            match next {
                                '*' => {
                                    chars.next();
                                    if chars.peek() == Some(&'*') {
                                        chars.next();
                                        break;
                                    } else {
                                        nested_italic = !nested_italic;
                                        if nested_italic {
                                            self.writer.write_colored(&text, Color::Green)?;
                                            text.clear();
                                        } else {
                                            self.writer.write_colored(&text, Color::Cyan)?;
                                            text.clear();
                                        }
                                    }
                                }
                                _ => {
                                    text.push(chars.next().unwrap());
                                }
                            }
                        }
                        if !text.is_empty() {
                            self.writer.write_colored(
                                &text,
                                if nested_italic {
                                    Color::Cyan
                                } else {
                                    Color::Green
                                },
                            )?;
                        }
                    } else {
                        let mut text = String::new();
                        while let Some(&next) = chars.peek() {
                            if next == '*' {
                                chars.next();
                                break;
                            }
                            text.push(chars.next().unwrap());
                        }
                        self.writer.write_colored(&text, Color::Cyan)?;
                    }
                }
                ch => {
                    chars.next();
                    self.writer.write_plain(&ch.to_string())?;
                }
            }
        }

        self.writer.clone().into_string()
    }
}
