use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

pub struct HintDisplay<'a> {
    hint: Option<&'a str>,
}

impl<'a> HintDisplay<'a> {
    pub fn new(hint: Option<&'a str>) -> Self {
        Self { hint }
    }
}

impl<'a> Widget for HintDisplay<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if let Some(hint_text) = self.hint {
            let title = vec![
                Span::styled("💡 ", Style::default().fg(Color::Yellow)),
                Span::styled("Hint", Style::default().fg(Color::Cyan)),
            ];

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(Line::from(title));

            let paragraph = Paragraph::new(Text::from(hint_text))
                .block(block)
                .wrap(Wrap { trim: true })
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Left);

            paragraph.render(area, buf);
        }
    }
}
