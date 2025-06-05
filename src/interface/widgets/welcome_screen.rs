use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

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

    fn render_header(&self, area: Rect, buf: &mut Buffer) {
        let header_lines = vec![
            "╔═╗╦ ╦╔═╗╔═╗╦╔═╔═╗╦═╗╔═╗",
            "║ ╠═╣║╣ ║ ╠╩╗║╣ ╠╦╝╚═╗",
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
            Style::default().fg(Color::Magenta),
        )))
        .alignment(Alignment::Center);
        sep_paragraph.render(area, buf);
    }

    fn render_did_you_know(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Magenta))
            .title(Span::styled(
                " Did You Know? ",
                Style::default().fg(Color::Cyan),
            ));

        let paragraph = Paragraph::new(Text::from(self.did_you_know.as_str()))
            .block(block)
            .wrap(Wrap { trim: true })
            .style(Style::default().fg(Color::White));

        paragraph.render(area, buf);
    }

    fn render_tip_of_the_day(&self, area: Rect, buf: &mut Buffer) {
        let title = vec![
            Span::styled("💡 ", Style::default().fg(Color::Yellow)),
            Span::styled("Tip of the Day", Style::default().fg(Color::Cyan)),
        ];

        let underline = "════════════════";

        let content = vec![
            Line::from(title),
            Line::from(Span::styled(underline, Style::default().fg(Color::Blue))),
            Line::from(""),
            Line::from(self.tip_of_the_day.as_str()),
        ];

        let paragraph = Paragraph::new(content)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center);

        paragraph.render(area, buf);
    }

    fn render_todays_challenge(&self, area: Rect, buf: &mut Buffer) {
        let title = vec![
            Span::styled("🎯 ", Style::default().fg(Color::Yellow)),
            Span::styled("Today's Challenge", Style::default().fg(Color::Cyan)),
        ];

        let underline = "═══════════════════";

        let content = vec![
            Line::from(title),
            Line::from(Span::styled(underline, Style::default().fg(Color::Blue))),
            Line::from(""),
            Line::from(self.todays_challenge.as_str()),
        ];

        let paragraph = Paragraph::new(content)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center);

        paragraph.render(area, buf);
    }

    fn render_instructions(&self, area: Rect, buf: &mut Buffer) {
        let instructions = Paragraph::new("Press ENTER to begin or Q/ESC to quit...")
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Center);

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
                Constraint::Length(2), // Space
                Constraint::Length(5), // Did You Know
                Constraint::Length(2), // Space
                Constraint::Length(5), // Tip of the Day
                Constraint::Length(2), // Space
                Constraint::Length(5), // Today's Challenge
                Constraint::Min(1),    // Flexible space
                Constraint::Length(1), // Instructions
                Constraint::Length(2), // Bottom padding
            ])
            .split(area);

        // Render each section
        self.render_header(chunks[0], buf);
        self.render_separator(chunks[1], buf);

        // Center the content blocks horizontally
        let content_width = 65;
        let did_you_know_area = Rect {
            x: area.x + (area.width.saturating_sub(content_width)) / 2,
            y: chunks[3].y,
            width: content_width.min(area.width),
            height: chunks[3].height,
        };
        self.render_did_you_know(did_you_know_area, buf);

        self.render_tip_of_the_day(chunks[5], buf);
        self.render_todays_challenge(chunks[7], buf);
        self.render_instructions(chunks[9], buf);
    }
}
