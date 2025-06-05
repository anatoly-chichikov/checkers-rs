use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color as RatatuiColor, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use crate::{
    ai::Hint,
    core::{game::CheckersGame, piece::Color},
    interface::{
        theme::Theme,
        widgets::{CheckerBoard, GameStatus, HintDisplay, WelcomeScreen},
    },
};

#[derive(Debug, PartialEq)]
pub enum Input {
    Up,
    Down,
    Left,
    Right,
    Select,
    Quit,
}

pub struct UI {
    cursor_pos: (usize, usize),
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl UI {
    pub fn new() -> io::Result<Self> {
        let backend = CrosstermBackend::new(io::stdout());
        let terminal = Terminal::new(backend)?;

        Ok(Self {
            cursor_pos: (5, 0), // Start at bottom left (black's starting position)
            terminal,
        })
    }

    pub fn init(&mut self) -> io::Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(
            io::stdout(),
            crossterm::terminal::EnterAlternateScreen,
            crossterm::event::EnableMouseCapture
        )?;
        self.terminal.clear()?;
        Ok(())
    }

    pub fn restore(&mut self) -> io::Result<()> {
        crossterm::terminal::disable_raw_mode()?;
        crossterm::execute!(
            io::stdout(),
            crossterm::terminal::LeaveAlternateScreen,
            crossterm::event::DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    pub fn draw_welcome_screen(
        &mut self,
        did_you_know: &str,
        tip_of_the_day: &str,
        todays_challenge: &str,
    ) -> io::Result<()> {
        self.terminal.draw(|f| {
            let welcome = WelcomeScreen::new(
                did_you_know.to_string(),
                tip_of_the_day.to_string(),
                todays_challenge.to_string(),
            );
            f.render_widget(welcome, f.area());
        })?;
        Ok(())
    }

    pub fn draw_game(
        &mut self,
        game: &CheckersGame,
        selected_square: Option<(usize, usize)>,
        possible_moves: &[(usize, usize)],
        hint: Option<&Hint>,
        ai_thinking: bool,
        ai_error: Option<&str>,
    ) -> io::Result<()> {
        self.terminal.draw(|f| {
            // First, create a centered column of fixed width
            let main_width = 64; // Увеличили ширину для большего "воздуха"
            let centered_area = if f.area().width >= main_width {
                Rect {
                    x: (f.area().width - main_width) / 2,
                    y: f.area().y,
                    width: main_width,
                    height: f.area().height,
                }
            } else {
                f.area() // Fallback if terminal is too narrow
            };

            // Main layout within the centered area
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),  // Top separator
                    Constraint::Length(2),  // Game status (reduced from 3)
                    Constraint::Length(18), // Board (fixed height instead of Min)
                    Constraint::Length(1),  // Bottom separator
                    Constraint::Length(1),  // Controls
                    Constraint::Length(6),  // Hint area (fixed small height)
                    Constraint::Min(0),     // Remaining space
                ])
                .split(centered_area);

            // Top separator
            let separator = "═".repeat(64);
            let sep_widget = Paragraph::new(separator).style(Style::default().fg(Theme::SEPARATOR));
            f.render_widget(sep_widget, chunks[0]);

            // Game status - no padding, aligned within the 60-char column
            let status = GameStatus::new(game.current_player)
                .ai_thinking(ai_thinking)
                .local_mode(false) // Will be determined by caller
                .ai_error(ai_error);
            f.render_widget(status, chunks[1]);

            // Board
            let board_widget = CheckerBoard::new(&game.board)
                .cursor_position(self.cursor_pos)
                .selected_square(selected_square)
                .possible_moves(possible_moves);
            f.render_widget(board_widget, chunks[2]);

            // Bottom separator
            let bottom_sep = "─".repeat(64);
            let bottom_sep_widget =
                Paragraph::new(bottom_sep).style(Style::default().fg(Theme::SEPARATOR));
            f.render_widget(bottom_sep_widget, chunks[3]);

            // Controls
            let controls = "Controls: ↑↓←→ Move | Space/Enter Select | Q/Esc Quit";
            let controls_widget = Paragraph::new(controls)
                .style(Style::default().fg(Theme::TEXT_SECONDARY))
                .alignment(Alignment::Center);
            f.render_widget(controls_widget, chunks[4]);

            // Hint
            if let Some(hint) = hint {
                let hint_display = HintDisplay::new(Some(&hint.hint));
                f.render_widget(hint_display, chunks[5]);
            }
        })?;
        Ok(())
    }

    pub fn draw_game_over(&mut self, winner: Option<Color>) -> io::Result<()> {
        self.terminal.draw(|f| {
            let message = match winner {
                Some(Color::Black) => "White wins!",
                Some(Color::White) => "Black wins!",
                None => "Stalemate! No possible moves.",
            };

            let text = vec![
                Line::from(""),
                Line::from(Span::styled(
                    "Game Over",
                    Style::default().fg(RatatuiColor::Yellow),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    message,
                    Style::default().fg(RatatuiColor::Green),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "Press any key to exit...",
                    Style::default().fg(RatatuiColor::White),
                )),
            ];

            let paragraph = Paragraph::new(text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(RatatuiColor::Magenta)),
                )
                .alignment(Alignment::Center);

            let area = centered_rect(50, 40, f.area());
            f.render_widget(paragraph, area);
        })?;
        Ok(())
    }

    pub fn get_input(&self) -> io::Result<Input> {
        loop {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                let input = match code {
                    KeyCode::Up => Input::Up,
                    KeyCode::Down => Input::Down,
                    KeyCode::Left => Input::Left,
                    KeyCode::Right => Input::Right,
                    KeyCode::Char(' ') | KeyCode::Enter => Input::Select,
                    KeyCode::Esc => Input::Quit,
                    KeyCode::Char('q') | KeyCode::Char('Q') => Input::Quit,
                    KeyCode::Char('й') | KeyCode::Char('Й') => Input::Quit,
                    _ => continue,
                };
                return Ok(input);
            }
        }
    }

    pub fn move_cursor(&mut self, direction: Input) {
        let (row, col) = self.cursor_pos;

        match direction {
            Input::Up if row > 0 => {
                self.cursor_pos = (row - 1, col);
            }
            Input::Down if row < 7 => {
                self.cursor_pos = (row + 1, col);
            }
            Input::Left if col > 0 => {
                self.cursor_pos = (row, col - 1);
            }
            Input::Right if col < 7 => {
                self.cursor_pos = (row, col + 1);
            }
            _ => {}
        }
    }

    pub fn get_cursor_position(&self) -> (usize, usize) {
        self.cursor_pos
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
