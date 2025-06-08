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
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl UI {
    pub fn new() -> io::Result<Self> {
        let backend = CrosstermBackend::new(io::stdout());
        let terminal = Terminal::new(backend)?;

        Ok(Self { terminal })
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

    #[allow(dead_code)]
    pub fn clear(&mut self) -> io::Result<()> {
        self.terminal.clear()?;
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

    #[allow(dead_code)]
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
                .cursor_position((0, 0)) // Temporary placeholder
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

    pub fn draw_view_data(&mut self, view: &crate::state::ViewData) -> io::Result<()> {
        // Check if it's a welcome screen
        if let Some((did_you_know, tip, challenge)) = view.welcome_content {
            return self.draw_welcome_screen(did_you_know, tip, challenge);
        }

        // Check if it's game over
        if view.is_game_over {
            let winner = if view.status_message.contains("Black wins") {
                Some(Color::White)
            } else if view.status_message.contains("White wins") {
                Some(Color::Black)
            } else {
                None
            };
            return self.draw_game_over(winner);
        }

        self.terminal.draw(|f| {
            // First, create a centered column of fixed width
            let main_width = 64;
            let centered_area = if f.area().width >= main_width {
                Rect {
                    x: (f.area().width - main_width) / 2,
                    y: f.area().y,
                    width: main_width,
                    height: f.area().height,
                }
            } else {
                f.area()
            };

            // Main layout within the centered area
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1), // Top separator
                    Constraint::Length(2), // Game status
                    Constraint::Min(18),   // Board - use Min to ensure at least 18
                    Constraint::Length(1), // Bottom separator
                    Constraint::Length(1), // Controls
                    Constraint::Max(6),    // Hint area - use Max to limit if needed
                ])
                .split(centered_area);

            // Top separator
            let separator = "═".repeat(64);
            let sep_widget = Paragraph::new(separator).style(Style::default().fg(Theme::SEPARATOR));
            f.render_widget(sep_widget, chunks[0]);

            // Game status
            let status = GameStatus::new(view.current_player)
                .ai_thinking(view.show_ai_thinking)
                .local_mode(false)
                .ai_error(view.error_message);
            f.render_widget(status, chunks[1]);

            // Board
            let board_widget = CheckerBoard::new(view.board)
                .cursor_position(view.cursor_pos)
                .selected_square(view.selected_piece)
                .possible_moves(view.possible_moves);
            f.render_widget(board_widget, chunks[2]);

            // Bottom separator
            let bottom_sep = "─".repeat(64);
            let bottom_sep_widget =
                Paragraph::new(bottom_sep).style(Style::default().fg(Theme::SEPARATOR));
            f.render_widget(bottom_sep_widget, chunks[3]);

            // Controls
            let controls = ["↑↓←→ Move", "Space/Enter Select", "ESC/Q Quit"];
            let controls_text = controls.join("  •  ");
            let controls_widget = Paragraph::new(controls_text)
                .style(Style::default().fg(Theme::TEXT_PRIMARY))
                .alignment(Alignment::Center);
            f.render_widget(controls_widget, chunks[4]);

            // Hint
            if let Some(hint) = view.hint {
                let hint_display = HintDisplay::new(Some(&hint.hint));
                f.render_widget(hint_display, chunks[5]);
            }
        })?;
        Ok(())
    }

    fn draw_game_over(&mut self, winner: Option<Color>) -> io::Result<()> {
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

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(RatatuiColor::Magenta));

            let area = centered_rect(50, 40, f.area());

            // Calculate inner area for padding
            let inner = block.inner(area);
            let padded_area = Rect {
                x: inner.x + 1,
                y: inner.y,
                width: inner.width.saturating_sub(2),
                height: inner.height,
            };

            // First render the block
            f.render_widget(block, area);

            // Then render the paragraph without block in the padded area
            let paragraph = Paragraph::new(text).alignment(Alignment::Center);

            f.render_widget(paragraph, padded_area);
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

    pub fn poll_input(&self) -> io::Result<Option<Input>> {
        if event::poll(std::time::Duration::from_millis(0))? {
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
                    _ => return Ok(None),
                };
                return Ok(Some(input));
            }
        }
        Ok(None)
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
