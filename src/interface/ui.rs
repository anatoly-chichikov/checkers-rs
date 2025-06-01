use crossterm::{
    cursor::{self, Hide},
    style::{Attribute, Color, Print, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{size, Clear, ClearType},
    QueueableCommand,
};
use std::io::{self, stdout, Write};

use crate::core::game::CheckersGame;
use crate::core::piece::Color as PieceColor;

pub struct UI {
    cursor_pos: (usize, usize),
    first_render: bool,
}

fn get_board_width() -> usize {
    // Each cell is 5 chars wide plus borders, total 6 chars per cell
    // Plus 3 chars for row numbers on the left
    3 + (6 * 8) + 1 // +1 for the final border
}

fn get_centering_offset() -> usize {
    if let Ok((width, _)) = size() {
        let board_width = get_board_width();
        if width as usize > board_width {
            (width as usize - board_width) / 2
        } else {
            0
        }
    } else {
        0
    }
}

impl Default for UI {
    fn default() -> Self {
        Self::new()
    }
}

impl UI {
    pub fn new() -> Self {
        Self {
            cursor_pos: (0, 0),
            first_render: true,
        }
    }

    pub fn set_cursor(&mut self, row: usize, col: usize) {
        self.cursor_pos = (row, col);
    }

    pub fn get_cursor(&self) -> (usize, usize) {
        self.cursor_pos
    }

    fn should_highlight_horizontal(&self, game: &CheckersGame, cell_pos: (usize, usize)) -> bool {
        // Horizontal lines (top/bottom) for cursor and selected piece
        cell_pos == self.cursor_pos || game.selected_piece == Some(cell_pos)
    }

    fn should_highlight_vertical(&self, game: &CheckersGame, cell_pos: (usize, usize)) -> bool {
        // Vertical lines (left/right) for possible moves
        game.possible_moves
            .as_ref()
            .is_some_and(|m| m.contains(&cell_pos))
    }

    fn render_column_headers(stdout: &mut io::Stdout, board_size: usize) -> io::Result<()> {
        let offset = get_centering_offset();
        if offset > 0 {
            stdout.queue(Print(" ".repeat(offset)))?;
        }

        stdout.queue(SetForegroundColor(Color::Blue))?;
        stdout.queue(Print("   "))?;
        for col in 0..board_size {
            write!(stdout, "   {}  ", (b'A' + col as u8) as char)?;
        }
        stdout.queue(ResetColor)?;
        stdout.write_all(b"\n\r")?;
        Ok(())
    }

    fn render_board_rows(&self, stdout: &mut io::Stdout, game: &CheckersGame) -> io::Result<()> {
        let offset = get_centering_offset();

        // Helper function to get the appropriate junction character
        let get_junction = |row: usize, col: usize| -> &'static str {
            if row == 0 {
                if col == 0 {
                    "┌"
                } else if col == 8 {
                    "┐"
                } else {
                    "┬"
                }
            } else if row == 8 {
                if col == 0 {
                    "└"
                } else if col == 8 {
                    "┘"
                } else {
                    "┴"
                }
            } else if col == 0 {
                "├"
            } else if col == 8 {
                "┤"
            } else {
                "┼"
            }
        };

        // Render each row including its top border
        for row in 0..=game.board.size {
            // Render horizontal border
            if offset > 0 {
                write!(stdout, "{}", " ".repeat(offset))?;
            }
            stdout.write_all(b"   ")?;

            for col in 0..=game.board.size {
                // Draw junction (always use normal color)
                stdout.queue(SetForegroundColor(Color::DarkGrey))?;
                write!(stdout, "{}", get_junction(row, col))?;

                // Draw horizontal line segment
                if col < game.board.size {
                    let mut segment_highlighted = false;
                    if row > 0 {
                        let cell_above = (row - 1, col);
                        if self.should_highlight_horizontal(game, cell_above) {
                            segment_highlighted = true;
                        }
                    }
                    if row < game.board.size {
                        let cell_below = (row, col);
                        if self.should_highlight_horizontal(game, cell_below) {
                            segment_highlighted = true;
                        }
                    }

                    if segment_highlighted {
                        // Draw highlighted segment with spacing
                        stdout.queue(SetForegroundColor(Color::DarkGrey))?;
                        write!(stdout, "─")?;
                        stdout.queue(SetForegroundColor(Color::Grey))?;
                        write!(stdout, "━━━")?;
                        stdout.queue(SetForegroundColor(Color::DarkGrey))?;
                        write!(stdout, "─")?;
                    } else {
                        // Draw normal segment
                        stdout.queue(SetForegroundColor(Color::DarkGrey))?;
                        write!(stdout, "─────")?;
                    }
                }
            }
            stdout.queue(ResetColor)?;
            stdout.write_all(b"\n\r")?;

            // Render cell contents row (if not the last border row)
            if row < game.board.size {
                if offset > 0 {
                    write!(stdout, "{}", " ".repeat(offset))?;
                }
                stdout.queue(SetForegroundColor(Color::Blue))?;
                write!(stdout, "{:2} ", row + 1)?;
                stdout.queue(ResetColor)?;

                for col in 0..=game.board.size {
                    // Draw vertical border
                    let mut border_highlighted = false;
                    if col > 0 {
                        let cell_left = (row, col - 1);
                        if self.should_highlight_vertical(game, cell_left) {
                            border_highlighted = true;
                        }
                    }
                    if col < game.board.size {
                        let cell_right = (row, col);
                        if self.should_highlight_vertical(game, cell_right) {
                            border_highlighted = true;
                        }
                    }

                    let border_color = if border_highlighted {
                        Color::Red
                    } else {
                        Color::DarkGrey
                    };
                    stdout.queue(SetForegroundColor(border_color))?;
                    write!(stdout, "{}", if border_highlighted { "┃" } else { "│" })?;
                    stdout.queue(ResetColor)?;

                    // Draw cell content (if not the last border)
                    if col < game.board.size {
                        let (content_to_display, text_color_for_content, is_bold) =
                            match game.board.get_piece(row, col) {
                                Some(piece) => {
                                    let color = match piece.color {
                                        PieceColor::White => Color::White,
                                        PieceColor::Black => Color::Red,
                                    };
                                    (format!(" {} ", piece.display()), color, piece.is_king)
                                }
                                None => {
                                    if (row + col) % 2 == 0 {
                                        ("     ".to_string(), Color::DarkGrey, false)
                                    } else {
                                        (" ░░░ ".to_string(), Color::DarkGrey, false)
                                    }
                                }
                            };

                        stdout.queue(SetForegroundColor(text_color_for_content))?;
                        if is_bold {
                            stdout.queue(SetAttribute(Attribute::Bold))?;
                        }
                        write!(stdout, "{}", content_to_display)?;
                        if is_bold {
                            stdout.queue(SetAttribute(Attribute::Reset))?;
                        }
                        stdout.queue(ResetColor)?;
                    }
                }
                stdout.write_all(b"\n\r")?;
            }
        }

        Ok(())
    }

    fn render_local_mode_indicator(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        // Calculate position similar to loading animation
        if let Ok((_width, _)) = size() {
            let board_width = get_board_width();
            let board_offset = get_centering_offset();

            let message = "Local Mode (2 Player)";
            let message_len = message.len();
            let x_pos = board_offset + board_width - message_len - 1;
            let y_pos = 1; // Same position as loading animation

            stdout.queue(cursor::SavePosition)?;
            stdout.queue(cursor::MoveTo(x_pos as u16, y_pos))?;
            stdout.queue(Clear(ClearType::UntilNewLine))?;
            stdout.queue(SetForegroundColor(Color::Cyan))?;
            stdout.queue(Print(message))?;
            stdout.queue(ResetColor)?;
            stdout.queue(cursor::RestorePosition)?;
        }
        Ok(())
    }

    fn render_game_status(&self, stdout: &mut io::Stdout, game: &CheckersGame) -> io::Result<()> {
        let offset = get_centering_offset();

        // Game status section with divider - removed initial newline
        if offset > 0 {
            write!(stdout, "{}", " ".repeat(offset))?;
        }
        stdout.queue(SetForegroundColor(Color::DarkGrey))?;
        write!(
            stdout,
            "═══════════════════════════════════════════════════════════"
        )?;
        stdout.queue(ResetColor)?;
        stdout.write_all(b"\n\r")?;

        // Current player
        if offset > 0 {
            write!(stdout, "{}", " ".repeat(offset))?;
        }
        write!(stdout, "Current Turn: ")?;
        let player_color = match game.current_player {
            PieceColor::White => Color::White,
            PieceColor::Black => Color::Red,
        };
        stdout.queue(SetForegroundColor(player_color))?;
        stdout.queue(SetAttribute(Attribute::Bold))?;
        write!(stdout, "{:?}", game.current_player)?;
        stdout.queue(SetAttribute(Attribute::Reset))?;
        stdout.queue(ResetColor)?;

        stdout.write_all(b"\n\r")?;

        Ok(())
    }

    fn render_ai_error(&self, stdout: &mut io::Stdout, error_text: &str) -> io::Result<()> {
        let offset = get_centering_offset();

        // Error section with red color
        stdout.write_all(b"\n\r")?;
        if offset > 0 {
            write!(stdout, "{}", " ".repeat(offset))?;
        }
        stdout.queue(SetForegroundColor(Color::Red))?;
        stdout.queue(SetAttribute(Attribute::Bold))?;
        write!(stdout, "⚠️  AI ERROR")?;
        stdout.queue(SetAttribute(Attribute::Reset))?;
        stdout.queue(ResetColor)?;
        stdout.write_all(b"\n\r")?;

        // Error message
        if offset > 0 {
            write!(stdout, "{}", " ".repeat(offset))?;
        }
        stdout.queue(SetForegroundColor(Color::Red))?;
        write!(stdout, "{}", error_text)?;
        stdout.queue(ResetColor)?;
        stdout.write_all(b"\n\r")?;

        Ok(())
    }

    fn render_controls(&self, stdout: &mut io::Stdout) -> io::Result<()> {
        let offset = get_centering_offset();

        // Controls section
        stdout.write_all(b"\n\r")?;
        if offset > 0 {
            write!(stdout, "{}", " ".repeat(offset))?;
        }
        stdout.queue(SetForegroundColor(Color::DarkGrey))?;
        write!(
            stdout,
            "───────────────────────────────────────────────────────────"
        )?;
        stdout.write_all(b"\n\r")?;
        if offset > 0 {
            write!(stdout, "{}", " ".repeat(offset))?;
        }
        write!(
            stdout,
            "Controls: ↑↓←→ Move | Space/Enter Select | H Hint | Q/Esc Quit"
        )?;
        stdout.write_all(b"\n\r")?;

        Ok(())
    }

    fn render_game_over(&self, stdout: &mut io::Stdout, game: &CheckersGame) -> io::Result<()> {
        if game.is_game_over {
            let offset = get_centering_offset();

            stdout.write_all(b"\n\r")?;
            if offset > 0 {
                write!(stdout, "{}", " ".repeat(offset))?;
            }
            stdout.queue(SetForegroundColor(Color::DarkGrey))?;
            write!(
                stdout,
                "═══════════════════════════════════════════════════════════"
            )?;
            stdout.queue(ResetColor)?;
            stdout.write_all(b"\n\r")?;

            if offset > 0 {
                write!(stdout, "{}", " ".repeat(offset))?;
            }
            stdout.queue(SetForegroundColor(Color::Yellow))?;
            stdout.queue(SetAttribute(Attribute::Bold))?;
            if let Some(winner) = game.check_winner() {
                write!(stdout, "🏆 GAME OVER! {:?} WINS! 🏆", winner)?;
            } else if game.is_stalemate() {
                write!(stdout, "🤝 GAME OVER! STALEMATE! 🤝")?;
            }
            stdout.queue(SetAttribute(Attribute::Reset))?;
            stdout.queue(ResetColor)?;

            stdout.write_all(b"\n\r")?;
            if offset > 0 {
                write!(stdout, "{}", " ".repeat(offset))?;
            }
            stdout.queue(SetForegroundColor(Color::Grey))?;
            write!(stdout, "Press any key to exit...")?;
            stdout.queue(ResetColor)?;
            stdout.write_all(b"\n\r")?;
        }

        Ok(())
    }

    pub fn render_hint(&self, stdout: &mut io::Stdout, hint: &str) -> io::Result<()> {
        let offset = get_centering_offset();

        stdout.write_all(b"\n\r")?;
        if offset > 0 {
            write!(stdout, "{}", " ".repeat(offset))?;
        }
        stdout.queue(SetForegroundColor(Color::DarkGrey))?;
        write!(
            stdout,
            "───────────────────────────────────────────────────────────"
        )?;
        stdout.queue(ResetColor)?;
        stdout.write_all(b"\n\r")?;

        // Split hint into lines if it's too long
        let max_width = 59; // Board width
        let words: Vec<&str> = hint.split_whitespace().collect();
        let mut current_line = String::new();
        let mut lines = Vec::new();

        for word in words {
            if current_line.len() + word.len() + 1 > max_width && !current_line.is_empty() {
                lines.push(current_line);
                current_line = String::new();
            }
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }
        if !current_line.is_empty() {
            lines.push(current_line);
        }

        // Display hint with icon
        for (i, line) in lines.iter().enumerate() {
            if offset > 0 {
                write!(stdout, "{}", " ".repeat(offset))?;
            }
            if i == 0 {
                stdout.queue(SetForegroundColor(Color::Yellow))?;
                write!(stdout, "💡 ")?;
                stdout.queue(SetForegroundColor(Color::Cyan))?;
                write!(stdout, "{}", line)?;
            } else {
                write!(stdout, "   {}", line)?;
            }
            stdout.queue(ResetColor)?;
            stdout.write_all(b"\n\r")?;
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn render_game(&mut self, game: &CheckersGame) -> io::Result<()> {
        self.render_game_with_hint(game, None)
    }

    #[allow(dead_code)]
    pub fn render_game_with_hint(
        &mut self,
        game: &CheckersGame,
        hint: Option<&str>,
    ) -> io::Result<()> {
        self.render_game_with_hint_and_mode(game, hint, false)
    }

    pub fn render_game_with_hint_and_mode(
        &mut self,
        game: &CheckersGame,
        hint: Option<&str>,
        ai_enabled: bool,
    ) -> io::Result<()> {
        let mut stdout = stdout();

        // Queue all operations first to minimize flicker
        stdout.queue(Hide)?;

        if self.first_render {
            stdout.queue(Clear(ClearType::All))?;
            self.first_render = false;
        }

        stdout.queue(cursor::MoveTo(0, 0))?;

        self.render_game_status(&mut stdout, game)?;

        // Show "Local Mode (2 Player)" when AI is not enabled and not thinking
        if !ai_enabled && !game.ai_thinking {
            self.render_local_mode_indicator(&mut stdout)?;
        }

        stdout.write_all(b"\n\r")?;
        Self::render_column_headers(&mut stdout, game.board.size)?;
        self.render_board_rows(&mut stdout, game)?;

        // Show controls only if game is not over
        if !game.is_game_over {
            // Show AI error if available (above controls)
            if let Some(error_text) = &game.ai_error {
                self.render_ai_error(&mut stdout, error_text)?;
            }

            self.render_controls(&mut stdout)?;

            // Show hint if available
            if let Some(hint_text) = hint {
                self.render_hint(&mut stdout, hint_text)?;
            }
        }

        // Show game over message after the board
        self.render_game_over(&mut stdout, game)?;

        // Clear any remaining lines after the content
        stdout.queue(Clear(ClearType::FromCursorDown))?;

        stdout.flush()?;
        Ok(())
    }
}
