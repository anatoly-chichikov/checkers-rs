use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color as RatatuiColor, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::core::piece::Color;

pub struct GameStatus<'a> {
    current_player: Color,
    ai_thinking: bool,
    is_local_mode: bool,
    ai_error: Option<&'a str>,
}

impl<'a> GameStatus<'a> {
    pub fn new(current_player: Color) -> Self {
        Self {
            current_player,
            ai_thinking: false,
            is_local_mode: false,
            ai_error: None,
        }
    }

    pub fn ai_thinking(mut self, thinking: bool) -> Self {
        self.ai_thinking = thinking;
        self
    }

    pub fn local_mode(mut self, local: bool) -> Self {
        self.is_local_mode = local;
        self
    }

    pub fn ai_error(mut self, error: Option<&'a str>) -> Self {
        self.ai_error = error;
        self
    }
}

impl<'a> Widget for GameStatus<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut lines = vec![];

        // Current turn indicator - Fixed: show actual current player
        let turn_text = match self.current_player {
            Color::White => "Current Turn: White",
            Color::Black => "Current Turn: Black",
        };
        lines.push(Line::from(Span::styled(
            turn_text,
            Style::default().fg(RatatuiColor::White),
        )));

        // Local mode indicator
        if self.is_local_mode {
            lines.push(Line::from(Span::styled(
                "[LOCAL MODE - Playing against another human]",
                Style::default().fg(RatatuiColor::Cyan),
            )));
        }

        // AI error message
        if let Some(error) = self.ai_error {
            lines.push(Line::from(Span::styled(
                format!("AI Error: {}", error),
                Style::default().fg(RatatuiColor::Red),
            )));
        }

        let paragraph = Paragraph::new(lines).alignment(Alignment::Left);
        paragraph.render(area, buf);
    }
}
