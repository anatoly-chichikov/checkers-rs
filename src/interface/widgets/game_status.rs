use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::core::piece::Color;
use crate::interface::theme::Theme;

pub struct GameStatus<'a> {
    current_player: Color,
    ai_thinking: bool,
    is_local_mode: bool,
    ai_error: Option<&'a str>,
    is_simple_ai: bool,
}

impl<'a> GameStatus<'a> {
    pub fn new(current_player: Color) -> Self {
        Self {
            current_player,
            ai_thinking: false,
            is_local_mode: false,
            ai_error: None,
            is_simple_ai: false,
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

    pub fn simple_ai(mut self, simple: bool) -> Self {
        self.is_simple_ai = simple;
        self
    }
}

impl<'a> Widget for GameStatus<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let turn_text = if self.ai_thinking {
            if self.is_simple_ai {
                "Simple AI is thinking..."
            } else {
                "AI is thinking..."
            }
        } else {
            match self.current_player {
                Color::White => "Current Turn: White",
                Color::Black => {
                    if self.is_simple_ai {
                        "Current Turn: Black (Simple AI)"
                    } else {
                        "Current Turn: Black"
                    }
                }
            }
        };

        let line = Line::from(Span::styled(
            turn_text,
            Style::default().fg(Theme::TEXT_PRIMARY),
        ));

        let paragraph = Paragraph::new(vec![line]).alignment(Alignment::Left);
        paragraph.render(area, buf);
    }
}
