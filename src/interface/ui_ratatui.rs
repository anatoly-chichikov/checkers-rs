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
    core::piece::Color,
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

    pub fn draw_welcome_screen(
        &mut self,
        did_you_know: &str,
        tip_of_the_day: &str,
        todays_challenge: &str,
        is_simple_ai: bool,
    ) -> io::Result<()> {
        self.terminal.draw(|f| {
            let welcome = WelcomeScreen::new(
                did_you_know.to_string(),
                tip_of_the_day.to_string(),
                todays_challenge.to_string(),
            ).simple_ai(is_simple_ai);
            f.render_widget(welcome, f.area());
        })?;
        Ok(())
    }

    pub fn draw_view_data(&mut self, view: &crate::state::ViewData) -> io::Result<()> {
        // Check if it's a welcome screen
        if let Some((did_you_know, tip, challenge)) = view.welcome_content {
            return self.draw_welcome_screen(did_you_know, tip, challenge, view.is_simple_ai);
        }

        // Check if it's game over
        if view.is_game_over {
            let winner = if view.status_message.contains("Black wins") {
                Some(Color::Black)
            } else if view.status_message.contains("White wins") {
                Some(Color::White)
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

            // Calculate dynamic hint height if hint is present
            let hint_height = if let Some(hint) = view.hint {
                use ratatui::widgets::{Paragraph, Wrap};

                // Create a paragraph with the hint text to calculate its wrapped height
                let paragraph = Paragraph::new(hint.hint.as_str()).wrap(Wrap { trim: true });

                // Account for borders (2) + inner padding in HintDisplay (2) = 4
                let available_width = centered_area.width.saturating_sub(4);

                // Use ratatui's built-in line_count method to get accurate height
                let lines = paragraph.line_count(available_width) as u16;

                // Add 2 for border top/bottom
                lines + 2
            } else {
                0 // No hint, no space needed
            };

            // Dynamic layout using modern ratatui best practices
            let mut constraints = vec![
                Constraint::Length(1),  // Top separator ════════════════
                Constraint::Length(1),  // Game status "Current Turn: White"
                Constraint::Length(1),  // One empty line
                Constraint::Length(18), // Board (requires exactly 18 lines)
                Constraint::Length(1),  // Bottom separator ────────────────
                Constraint::Length(1),  // Controls line
            ];

            if hint_height > 0 {
                constraints.push(Constraint::Length(hint_height)); // Dynamic hint area
            }
            constraints.push(Constraint::Fill(1)); // Fill remaining space efficiently

            let chunks = Layout::vertical(constraints).split(centered_area);

            // Top separator
            let separator = "═".repeat(64);
            let sep_widget = Paragraph::new(separator).style(Style::default().fg(Theme::SEPARATOR));
            f.render_widget(sep_widget, chunks[0]);

            // Game status
            let status = GameStatus::new(view.current_player)
                .ai_thinking(view.show_ai_thinking)
                .local_mode(false)
                .ai_error(view.error_message)
                .simple_ai(view.is_simple_ai);
            f.render_widget(status, chunks[1]);

            // chunks[2] is the empty line - leave it empty

            // Board
            let board_widget = CheckerBoard::new(view.board)
                .cursor_position(view.cursor_pos)
                .selected_square(view.selected_piece)
                .possible_moves(view.possible_moves)
                .pieces_with_captures(&view.pieces_with_captures);
            f.render_widget(board_widget, chunks[3]);

            // Bottom separator
            let bottom_sep = "─".repeat(64);
            let bottom_sep_widget =
                Paragraph::new(bottom_sep).style(Style::default().fg(Theme::SEPARATOR));
            f.render_widget(bottom_sep_widget, chunks[4]);

            // Controls
            let controls = ["↑↓←→ Move", "Space/Enter Select", "ESC/Q Quit"];
            let controls_text = controls.join("  •  ");
            let controls_widget = Paragraph::new(controls_text)
                .style(Style::default().fg(Theme::TEXT_PRIMARY))
                .alignment(Alignment::Center);
            f.render_widget(controls_widget, chunks[5]);

            // Hint (if present, render in the dynamic area)
            if let Some(hint) = view.hint {
                let hint_display = HintDisplay::new(Some(&hint.hint));
                // Hint is at index 6 if present
                f.render_widget(hint_display, chunks[6]);
            }
        })?;
        Ok(())
    }

    fn draw_game_over(&mut self, winner: Option<Color>) -> io::Result<()> {
        self.terminal.draw(|f| {
            let message = match winner {
                Some(Color::Black) => "Black wins!",
                Some(Color::White) => "White wins!",
                None => "Stalemate! No possible moves.",
            };

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(RatatuiColor::Magenta));

            let area = centered_rect(50, 20, f.area());

            // Calculate inner area
            let inner = block.inner(area);

            // Create vertical layout for centering
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1), // Top padding
                    Constraint::Length(1), // "Game Over"
                    Constraint::Length(1), // Space
                    Constraint::Length(1), // Winner message
                    Constraint::Length(1), // Space
                    Constraint::Length(1), // "Press ESC to exit..."
                    Constraint::Min(0),    // Bottom padding
                ])
                .split(inner);

            // First render the block
            f.render_widget(block, area);

            // Render each line in its chunk
            let game_over_line = Paragraph::new(Line::from(vec![Span::styled(
                "Game Over",
                Style::default().fg(RatatuiColor::Yellow),
            )]))
            .alignment(Alignment::Center);
            f.render_widget(game_over_line, chunks[1]);

            let winner_line = Paragraph::new(Line::from(vec![Span::styled(
                message,
                Style::default().fg(RatatuiColor::Green),
            )]))
            .alignment(Alignment::Center);
            f.render_widget(winner_line, chunks[3]);

            let exit_line = Paragraph::new(Line::from(vec![Span::styled(
                "Press ESC to exit...",
                Style::default().fg(RatatuiColor::White),
            )]))
            .alignment(Alignment::Center);
            f.render_widget(exit_line, chunks[5]);
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
