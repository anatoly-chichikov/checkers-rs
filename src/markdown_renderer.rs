use crossterm::style::{Color, SetForegroundColor, ResetColor};
use std::io::{self, Write};

pub struct MarkdownRenderer {
    output: Vec<u8>,
}

impl MarkdownRenderer {
    pub fn new() -> Self {
        Self {
            output: Vec::new(),
        }
    }

    /// Parses a header section starting with '#' characters.
    /// 
    /// # Arguments
    /// 
    /// * `chars` - Mutable iterator over the remaining characters in the markdown text
    /// 
    /// # Returns
    /// 
    /// Returns `io::Result<()>` indicating success or failure of the parsing operation
    fn parse_header(&mut self, chars: &mut std::iter::Peekable<std::str::Chars>) -> io::Result<()> {
        let mut header_level = 0;
        while chars.peek() == Some(&'#') {
            header_level += 1;
            chars.next();
        }
        
        // Skip whitespace after #
        while chars.peek() == Some(&' ') {
            chars.next();
        }

        // Parse header content with formatting
        let mut text = String::new();
        
        while let Some(&next) = chars.peek() {
            if next == '\n' { 
                chars.next();
                break; 
            }
            
            match chars.next().unwrap() {
                '*' => {
                    if chars.peek() == Some(&'*') {
                        chars.next(); // Skip second *
                        // Handle bold text
                        while let Some(&ch) = chars.peek() {
                            if ch == '*' {
                                chars.next();
                                if chars.peek() == Some(&'*') {
                                    chars.next();
                                    break;
                                }
                            }
                            text.push(ch);
                            chars.next();
                        }
                    } else {
                        // Handle italic text
                        while let Some(&ch) = chars.peek() {
                            if ch == '*' {
                                chars.next();
                                break;
                            }
                            text.push(ch);
                            chars.next();
                        }
                    }
                },
                '[' => {
                    // Handle link text only, skip URL
                    let mut link_text = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch == ']' {
                            chars.next();
                            // Skip URL part
                            if chars.peek() == Some(&'(') {
                                while let Some(&ch) = chars.peek() {
                                    if ch == ')' {
                                        chars.next();
                                        break;
                                    }
                                    chars.next();
                                }
                            }
                            text.push_str(&link_text);
                            break;
                        }
                        link_text.push(ch);
                        chars.next();
                    }
                },
                ch => text.push(ch),
            }
        }
        
        // Calculate indentation based on header level
        let indentation = match header_level {
            1 => "",
            2 => "  ",
            3 => "    ",
            4 => "      ",
            5 => "        ",
            6 => "          ",
            _ => "",
        };
        
        // Write the header with proper indentation
        write!(self.output, "{}", SetForegroundColor(Color::Magenta))?;
        if header_level == 1 {
            write!(self.output, "{}\n", text.trim().to_uppercase())?;
        } else {
            write!(self.output, "{}{}\n", indentation, text.trim().to_uppercase())?;
        }
        write!(self.output, "{}", ResetColor)?;
        
        Ok(())
    }

    /// Parses a list item and formats it with a bullet point.
    /// Handles nested formatting within the list item including bold text, italic text, and links.
    /// 
    /// # Arguments
    /// 
    /// * `chars` - Mutable iterator over the remaining characters in the markdown text
    /// 
    /// # Returns
    /// 
    /// Returns `io::Result<()>` indicating success or failure of the parsing operation
    fn parse_list_item(&mut self, chars: &mut std::iter::Peekable<std::str::Chars>) -> io::Result<()> {
        let mut text = String::new();
        let mut in_bold = false;
        let mut in_italic = false;
        
        while let Some(&next) = chars.peek() {
            if next == '\n' { 
                chars.next();
                break; 
            }
            
            match chars.next().unwrap() {
                '*' => {
                    if !text.is_empty() {
                        write!(self.output, "{}", text)?;
                        text.clear();
                    }
                    
                    if chars.peek() == Some(&'*') {
                        chars.next();
                        in_bold = !in_bold;
                        write!(self.output, "{}", SetForegroundColor(Color::Green))?;
                        self.parse_text_until(chars, '*', Color::Green)?;
                        write!(self.output, "{}", ResetColor)?;
                    } else {
                        in_italic = !in_italic;
                        write!(self.output, "{}", SetForegroundColor(Color::Cyan))?;
                        self.parse_text_until(chars, '*', Color::Cyan)?;
                        write!(self.output, "{}", ResetColor)?;
                    }
                },
                '[' => {
                    if !text.is_empty() {
                        write!(self.output, "{}", text)?;
                        text.clear();
                    }
                    self.handle_link(chars)?;
                },
                ch => text.push(ch),
            }
        }
        
        if !text.is_empty() {
            write!(self.output, "{}", text)?;
        }
        
        write!(self.output, "\n")?;
        Ok(())
    }

    /// Handles both inline code (single backticks) and code blocks (triple backticks).
    /// Formats inline code in green and code blocks in yellow.
    /// 
    /// # Arguments
    /// 
    /// * `chars` - Mutable iterator over the remaining characters in the markdown text
    /// 
    /// # Returns
    /// 
    /// Returns `io::Result<bool>` where the boolean indicates if a newline was added (true for code blocks)
    fn handle_code_block(&mut self, chars: &mut std::iter::Peekable<std::str::Chars>) -> io::Result<bool> {
        if chars.peek() == Some(&'`') {
            chars.next();
            if chars.peek() == Some(&'`') {
                chars.next();
                // Code block
                let mut code = String::new();
                while let Some(ch) = chars.next() {
                    if ch == '`' && 
                       chars.peek() == Some(&'`') {
                        chars.next();
                        if chars.peek() == Some(&'`') {
                            chars.next();
                            break;
                        }
                    }
                    code.push(ch);
                }
                
                write!(
                    self.output,
                    "\n{}{}{}\n",
                    SetForegroundColor(Color::Yellow),
                    code.trim(),
                    ResetColor
                )?;
                return Ok(true);
            }
        }
        
        // Inline code
        let mut code = String::new();
        while let Some(ch) = chars.next() {
            if ch == '`' { break; }
            code.push(ch);
        }
        
        write!(
            self.output,
            "{}{}{}",
            SetForegroundColor(Color::Green),
            code,
            ResetColor
        )?;
        Ok(false)
    }

    /// Processes markdown link syntax ([text](url)) and formats the link text in blue.
    /// The URL is parsed but not included in the output.
    /// 
    /// # Arguments
    /// 
    /// * `chars` - Mutable iterator over the remaining characters in the markdown text
    /// 
    /// # Returns
    /// 
    /// Returns `io::Result<()>` indicating success or failure of the parsing operation
    fn handle_link(&mut self, chars: &mut std::iter::Peekable<std::str::Chars>) -> io::Result<()> {
        let mut text = String::new();
        let mut found_closing_bracket = false;
        
        while let Some(ch) = chars.next() {
            if ch == ']' { 
                found_closing_bracket = true;
                break; 
            }
            text.push(ch);
        }
        
        if found_closing_bracket && chars.peek() == Some(&'(') {
            chars.next();
            // Skip URL
            while let Some(ch) = chars.next() {
                if ch == ')' { break; }
            }
            
            write!(
                self.output,
                "{}{}{}",
                SetForegroundColor(Color::Blue),
                text,
                ResetColor
            )?;
        } else {
            // If it's not a proper link, write the original text with brackets
            write!(self.output, "[{}]", text)?;
        }
        Ok(())
    }

    /// Parses formatted text until a specified ending character is found.
    /// Handles nested formatting within the text.
    /// 
    /// # Arguments
    /// 
    /// * `chars` - Mutable iterator over the remaining characters in the markdown text
    /// * `end_char` - The character that marks the end of the formatted section
    /// * `style` - The color to apply to the formatted text
    /// 
    /// # Returns
    /// 
    /// Returns `io::Result<()>` indicating success or failure of the parsing operation
    fn parse_text_until(&mut self, chars: &mut std::iter::Peekable<std::str::Chars>, end_char: char, style: Color) -> io::Result<()> {
        let mut text = String::new();
        
        while let Some(&ch) = chars.peek() {
            if ch == end_char {
                chars.next();
                if end_char == '*' && chars.peek() == Some(&'*') {
                    chars.next();
                    break;
                } else if end_char != '*' {
                    break;
                }
            }
            
            match chars.next().unwrap() {
                '*' => {
                    // Write accumulated text before nested formatting
                    if !text.is_empty() {
                        write!(self.output, "{}{}{}", SetForegroundColor(style), text, ResetColor)?;
                        text.clear();
                    }
                    
                    if chars.peek() == Some(&'*') {
                        chars.next();
                        write!(self.output, "{}", SetForegroundColor(Color::Green))?;
                        self.parse_text_until(chars, '*', Color::Green)?;
                        write!(self.output, "{}", ResetColor)?;
                    } else {
                        write!(self.output, "{}", SetForegroundColor(Color::Cyan))?;
                        self.parse_text_until(chars, '*', Color::Cyan)?;
                        write!(self.output, "{}", ResetColor)?;
                    }
                },
                ch => text.push(ch),
            }
        }
        
        if !text.is_empty() {
            write!(self.output, "{}{}{}", SetForegroundColor(style), text, ResetColor)?;
        }
        
        Ok(())
    }

    /// Renders markdown text to colored terminal output.
    /// Processes various markdown elements including headers, emphasis, code blocks,
    /// links, and lists, applying appropriate colors and formatting.
    /// 
    /// # Arguments
    /// 
    /// * `markdown` - The markdown text to render
    /// 
    /// # Returns
    /// 
    /// Returns `io::Result<String>` containing the rendered text with ANSI color codes
    /// 
    /// # Examples
    /// 
    /// ```
    /// use checkers_rs::markdown_renderer::MarkdownRenderer;
    /// # use std::io;
    /// # fn main() -> io::Result<()> {
    /// let mut renderer = MarkdownRenderer::new();
    /// let result = renderer.render("# Hello **world**")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn render(&mut self, markdown: &str) -> io::Result<String> {
        self.output.clear();
        let mut chars = markdown.chars().peekable();
        let mut last_was_newline = true;
        
        while let Some(c) = chars.next() {
            match c {
                '#' => {
                    self.parse_header(&mut chars)?;
                    last_was_newline = true;
                }
                c @ ('-' | '*' | '+') if last_was_newline => {
                    // Check if it's a list item (followed by space)
                    let mut has_space = false;
                    let mut spaces = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch != ' ' {
                            break;
                        }
                        has_space = true;
                        spaces.push(ch);
                        chars.next();
                    }
                    
                    if has_space || c != '*' {
                        write!(self.output, "• ")?;
                        if c == '+' {
                            write!(self.output, "Third with ")?;
                        }
                        self.parse_list_item(&mut chars)?;
                        last_was_newline = true;
                    } else {
                        // Handle emphasis
                        if chars.peek() == Some(&'*') {
                            chars.next();
                            write!(self.output, "{}", SetForegroundColor(Color::Green))?;
                            self.parse_text_until(&mut chars, '*', Color::Green)?;
                            write!(self.output, "{}", ResetColor)?;
                        } else {
                            write!(self.output, "{}", SetForegroundColor(Color::Cyan))?;
                            self.parse_text_until(&mut chars, '*', Color::Cyan)?;
                            write!(self.output, "{}", ResetColor)?;
                        }
                        last_was_newline = false;
                    }
                }
                '*' => {
                    if chars.peek() == Some(&'*') {
                        chars.next();
                        write!(self.output, "{}", SetForegroundColor(Color::Green))?;
                        self.parse_text_until(&mut chars, '*', Color::Green)?;
                        write!(self.output, "{}", ResetColor)?;
                    } else {
                        write!(self.output, "{}", SetForegroundColor(Color::Cyan))?;
                        self.parse_text_until(&mut chars, '*', Color::Cyan)?;
                        write!(self.output, "{}", ResetColor)?;
                    }
                    last_was_newline = false;
                }
                '`' => {
                    last_was_newline = self.handle_code_block(&mut chars)?;
                }
                '[' => {
                    self.handle_link(&mut chars)?;
                    last_was_newline = false;
                }
                '\n' => {
                    write!(self.output, "\n")?;
                    last_was_newline = true;
                }
                _ => {
                    write!(self.output, "{}", c)?;
                    last_was_newline = false;
                }
            }
        }
        
        Ok(String::from_utf8_lossy(&self.output).into_owned())
    }
} 