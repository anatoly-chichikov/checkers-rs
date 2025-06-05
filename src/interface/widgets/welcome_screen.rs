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
}

impl WelcomeScreen {
    pub fn new(did_you_know: String, tip_of_the_day: String, todays_challenge: String) -> Self {
        Self {
            did_you_know,
            tip_of_the_day,
            todays_challenge,
        }
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
        let header_lines = vec![
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
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Theme::BORDER))
            .title(Span::styled(
                " Did You Know? ",
                Style::default().fg(Theme::TEXT_ACCENT),
            ));

        let paragraph = Paragraph::new(Text::from(self.did_you_know.as_str()))
            .block(block)
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Theme::TEXT_PRIMARY))
            .alignment(Alignment::Left); // Changed to left alignment

        paragraph.render(area, buf);
    }

    fn render_tip_of_the_day(&self, area: Rect, buf: &mut Buffer) {
        let title = vec![
            Span::styled("💡 ", Style::default().fg(Theme::EMOJI)),
            Span::styled("Tip of the Day", Style::default().fg(Theme::TEXT_ACCENT)),
        ];

        let underline = "════════════════";

        // Split content into lines for proper wrapping
        let wrapped_text = self.wrap_text(&self.tip_of_the_day, 60);
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

        paragraph.render(area, buf);
    }

    fn render_todays_challenge(&self, area: Rect, buf: &mut Buffer) {
        let title = vec![
            Span::styled("🎯 ", Style::default().fg(Theme::EMOJI)),
            Span::styled("Today's Challenge", Style::default().fg(Theme::TEXT_ACCENT)),
        ];

        let underline = "═══════════════════";

        // Split content into lines for proper wrapping
        let wrapped_text = self.wrap_text(&self.todays_challenge, 60);
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

        paragraph.render(area, buf);
    }

    fn render_instructions(&self, area: Rect, buf: &mut Buffer) {
        let instructions = Paragraph::new("Press ENTER to begin or Q/ESC to quit...")
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
                Constraint::Length(3), // Header
                Constraint::Length(1), // Separator
                Constraint::Length(1), // Space after separator
                Constraint::Length(5), // Did You Know
                Constraint::Length(2), // Space
                Constraint::Length(6), // Tip of the Day (with underline)
                Constraint::Length(2), // Space
                Constraint::Length(6), // Today's Challenge (with underline)
                Constraint::Min(1),    // Flexible space
                Constraint::Length(1), // Instructions
                Constraint::Length(2), // Bottom padding
            ])
            .split(area);

        // Render header and separator (keep centered)
        self.render_header(chunks[0], buf);
        self.render_separator(chunks[1], buf);

        // Create centered column for content (same as game board)
        let content_width = 64;
        let content_x = area.x + (area.width.saturating_sub(content_width)) / 2;

        // Did You Know block
        let did_you_know_area = Rect {
            x: content_x,
            y: chunks[3].y,
            width: content_width.min(area.width),
            height: chunks[3].height,
        };
        self.render_did_you_know(did_you_know_area, buf);

        // Tip of the Day (in centered column)
        let tip_area = Rect {
            x: content_x,
            y: chunks[5].y,
            width: content_width.min(area.width),
            height: chunks[5].height,
        };
        self.render_tip_of_the_day(tip_area, buf);

        // Today's Challenge (in centered column)
        let challenge_area = Rect {
            x: content_x,
            y: chunks[7].y,
            width: content_width.min(area.width),
            height: chunks[7].height,
        };
        self.render_todays_challenge(challenge_area, buf);

        // Instructions (keep full width for centering)
        self.render_instructions(chunks[9], buf);
    }
}
