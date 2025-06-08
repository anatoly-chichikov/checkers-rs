use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::interface::theme::Theme;

pub struct WelcomeScreen {
    did_you_know: String,
    tip_of_the_day: String,
    todays_challenge: String,
    is_simple_ai: bool,
}

impl WelcomeScreen {
    pub fn new(did_you_know: String, tip_of_the_day: String, todays_challenge: String) -> Self {
        Self {
            did_you_know,
            tip_of_the_day,
            todays_challenge,
            is_simple_ai: false,
        }
    }
    
    pub fn simple_ai(mut self, simple: bool) -> Self {
        self.is_simple_ai = simple;
        self
    }

    fn wrap_text(&self, text: &str, max_width: usize) -> Vec<String> {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut lines = Vec::new();
        let mut current_line = String::new();

        for word in words {
            if current_line.is_empty() {
                current_line = word.to_string();
            } else if current_line.len() + 1 + word.len() <= max_width {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                lines.push(current_line);
                current_line = word.to_string();
            }
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        lines
    }

    fn render_header(&self, area: Rect, buf: &mut Buffer) {
        let header_lines = [
            "╔═╗╦ ╦╔═╗╔═╗╦╔═╔═╗╦═╗╔═╗",
            "║  ╠═╣║╣ ║  ╠╩╗║╣ ╠╦╝╚═╗",
            "╚═╝╩ ╩╚═╝╚═╝╩ ╩╚═╝╩╚═╚═╝",
        ];

        let header_text: Vec<Line> = header_lines
            .iter()
            .map(|line| Line::from(Span::styled(*line, Style::default().fg(Color::Magenta))))
            .collect();

        let header = Paragraph::new(header_text).alignment(Alignment::Center);
        header.render(area, buf);
    }

    fn render_separator(&self, area: Rect, buf: &mut Buffer) {
        let separator = "░".repeat(30);
        let sep_paragraph = Paragraph::new(Line::from(Span::styled(
            separator,
            Style::default().fg(Theme::LOGO), // Keep original color for logo
        )))
        .alignment(Alignment::Center);
        sep_paragraph.render(area, buf);
    }

    fn render_did_you_know(&self, area: Rect, buf: &mut Buffer) {
        // Guard against zero height
        if area.height < 3 {
            return;
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Theme::BORDER))
            .title(Span::styled(
                " Did You Know? ",
                Style::default().fg(Theme::TEXT_ACCENT),
            ));

        // Calculate inner area for padding
        let inner = block.inner(area);
        let padded_area = Rect {
            x: inner.x + 1,
            y: inner.y,
            width: inner.width.saturating_sub(2),
            height: inner.height,
        };

        // First render the block
        block.render(area, buf);

        // Guard against no space for content
        if padded_area.height == 0 || padded_area.width == 0 {
            return;
        }

        // Then render the paragraph without block in the padded area
        let paragraph = Paragraph::new(Text::from(self.did_you_know.as_str()))
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Theme::TEXT_PRIMARY))
            .alignment(Alignment::Left); // Changed to left alignment

        paragraph.render(padded_area, buf);
    }

    fn render_tip_of_the_day(&self, area: Rect, buf: &mut Buffer) {
        let title = vec![
            Span::styled("💡 ", Style::default().fg(Theme::EMOJI)),
            Span::styled("Tip of the Day", Style::default().fg(Theme::TEXT_ACCENT)),
        ];

        let underline = "════════════════";

        // Add horizontal padding
        let padded_area = Rect {
            x: area.x + 1,
            y: area.y,
            width: area.width.saturating_sub(2),
            height: area.height,
        };

        // Split content into lines for proper wrapping with adjusted width
        let wrapped_text = self.wrap_text(&self.tip_of_the_day, padded_area.width as usize);
        let mut content = vec![
            Line::from(title),
            Line::from(Span::styled(
                underline,
                Style::default().fg(Theme::HIGHLIGHT),
            )),
        ];

        for line in wrapped_text {
            content.push(Line::from(Span::styled(
                line,
                Style::default().fg(Theme::TEXT_PRIMARY),
            )));
        }

        let paragraph = Paragraph::new(content).alignment(Alignment::Left); // Changed to left

        paragraph.render(padded_area, buf);
    }

    fn render_todays_challenge(&self, area: Rect, buf: &mut Buffer) {
        let title = vec![
            Span::styled("🎯 ", Style::default().fg(Theme::EMOJI)),
            Span::styled("Today's Challenge", Style::default().fg(Theme::TEXT_ACCENT)),
        ];

        let underline = "═══════════════════";

        // Add horizontal padding
        let padded_area = Rect {
            x: area.x + 1,
            y: area.y,
            width: area.width.saturating_sub(2),
            height: area.height,
        };

        // Split content into lines for proper wrapping with adjusted width
        let wrapped_text = self.wrap_text(&self.todays_challenge, padded_area.width as usize);
        let mut content = vec![
            Line::from(title),
            Line::from(Span::styled(
                underline,
                Style::default().fg(Theme::HIGHLIGHT),
            )),
        ];

        for line in wrapped_text {
            content.push(Line::from(Span::styled(
                line,
                Style::default().fg(Theme::TEXT_PRIMARY),
            )));
        }

        let paragraph = Paragraph::new(content).alignment(Alignment::Left); // Changed to left

        paragraph.render(padded_area, buf);
    }

    fn render_instructions(&self, area: Rect, buf: &mut Buffer) {
        let text = if self.is_simple_ai {
            "Press ENTER to play against Simple AI or Q/ESC to quit..."
        } else {
            "Press ENTER to play against AI or Q/ESC to quit..."
        };
        
        let instructions = Paragraph::new(text)
            .style(Style::default().fg(Theme::TEXT_SECONDARY))
            .alignment(Alignment::Center); // Keep centered for instructions

        instructions.render(area, buf);
    }
}

impl Widget for WelcomeScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Layout for vertical sections
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Space before header
                Constraint::Length(3), // Header
                Constraint::Length(1), // Separator
                Constraint::Length(1), // Space after separator
                Constraint::Length(4), // Did You Know
                Constraint::Length(1), // Space
                Constraint::Length(4), // Tip of the Day
                Constraint::Length(1), // Space
                Constraint::Length(4), // Today's Challenge
                Constraint::Length(1), // Space before instructions
                Constraint::Length(1), // Instructions
                Constraint::Min(0),    // Remaining space
            ])
            .split(area);

        // Render header and separator (keep centered)
        self.render_header(chunks[1], buf);
        self.render_separator(chunks[2], buf);

        // Create centered column for content (same as game board)
        let content_width = 64;
        let content_x = area.x + (area.width.saturating_sub(content_width)) / 2;

        // Did You Know block
        let did_you_know_area = Rect {
            x: content_x,
            y: chunks[4].y,
            width: content_width.min(area.width),
            height: chunks[4].height,
        };
        self.render_did_you_know(did_you_know_area, buf);

        // Tip of the Day (in centered column)
        let tip_area = Rect {
            x: content_x,
            y: chunks[6].y,
            width: content_width.min(area.width),
            height: chunks[6].height,
        };
        self.render_tip_of_the_day(tip_area, buf);

        // Today's Challenge (in centered column)
        let challenge_area = Rect {
            x: content_x,
            y: chunks[8].y,
            width: content_width.min(area.width),
            height: chunks[8].height,
        };
        self.render_todays_challenge(challenge_area, buf);

        // Instructions (keep full width for centering)
        self.render_instructions(chunks[10], buf);
    }
}
