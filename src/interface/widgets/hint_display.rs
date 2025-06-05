use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::interface::theme::Theme;

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
                Span::styled("💡 ", Style::default().fg(Theme::EMOJI)),
                Span::styled("Hint", Style::default().fg(Theme::TEXT_ACCENT)),
            ];

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Theme::BORDER))
                .title(Line::from(title));

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

            // Then render the paragraph without block in the padded area
            let paragraph = Paragraph::new(Text::from(hint_text))
                .wrap(Wrap { trim: true })
                .style(Style::default().fg(Theme::TEXT_PRIMARY))
                .alignment(Alignment::Left);

            paragraph.render(padded_area, buf);
        }
    }
}
