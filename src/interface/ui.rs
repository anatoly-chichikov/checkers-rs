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

    fn get_cell_border_style(&self, game: &CheckersGame, cell_pos: (usize, usize)) -> Color {
        if cell_pos == self.cursor_pos {
            Color::White
        } else if game.selected_piece == Some(cell_pos)
            || game
                .possible_moves
                .as_ref()
                .is_some_and(|m| m.contains(&cell_pos))
        {
            Color::Grey
        } else {
            Color::DarkGrey
        }
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

        // Top border
        if offset > 0 {
            write!(stdout, "{}", " ".repeat(offset))?;
        }
        stdout.write_all(b"   ")?;
        stdout.queue(SetForegroundColor(Color::DarkGrey))?;
        write!(stdout, "┌─────┬─────┬─────┬─────┬─────┬─────┬─────┬─────┐")?;
        stdout.queue(ResetColor)?;
        stdout.write_all(b"\n\r")?;

        for row in 0..game.board.size {
            // Cell contents
            if offset > 0 {
                write!(stdout, "{}", " ".repeat(offset))?;
            }
            stdout.queue(SetForegroundColor(Color::Blue))?;
            write!(stdout, "{:2} ", row + 1)?;
            stdout.queue(ResetColor)?;

            for col in 0..game.board.size {
                let cell_border_color = self.get_cell_border_style(game, (row, col));

                // Left border of the cell
                stdout.queue(SetForegroundColor(cell_border_color))?;
                write!(stdout, "│")?;
                stdout.queue(ResetColor)?;

                let (content_to_display, text_color_for_content, is_bold) =
                    match game.board.get_piece(row, col) {
                        Some(piece) => {
                            let color = match piece.color {
                                PieceColor::White => Color::White,
                                PieceColor::Black => Color::Red,
                            };
                            (
                                format!(" {} ", piece.display()),
                                color,
                                piece.is_king, // Kings are bold
                            )
                        }
                        None => {
                            if (row + col) % 2 == 0 {
                                ("     ".to_string(), Color::DarkGrey, false)
                            } else {
                                ("░░░░░".to_string(), Color::DarkGrey, false)
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

            // Final right border
            let last_cell_border_color =
                self.get_cell_border_style(game, (row, game.board.size - 1));
            stdout.queue(SetForegroundColor(last_cell_border_color))?;
            write!(stdout, "│")?;
            stdout.queue(ResetColor)?;
            stdout.write_all(b"\n\r")?;

            // Horizontal separator (except after last row)
            if row < game.board.size - 1 {
                if offset > 0 {
                    write!(stdout, "{}", " ".repeat(offset))?;
                }
                stdout.write_all(b"   ")?;
                stdout.queue(SetForegroundColor(Color::DarkGrey))?;
                write!(stdout, "├─────┼─────┼─────┼─────┼─────┼─────┼─────┼─────┤")?;
                stdout.queue(ResetColor)?;
                stdout.write_all(b"\n\r")?;
            }
        }

        // Bottom border
        if offset > 0 {
            write!(stdout, "{}", " ".repeat(offset))?;
        }
        stdout.write_all(b"   ")?;
        stdout.queue(SetForegroundColor(Color::DarkGrey))?;
        write!(stdout, "└─────┴─────┴─────┴─────┴─────┴─────┴─────┴─────┘")?;
        stdout.queue(ResetColor)?;
        stdout.write_all(b"\n\r")?;

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

        // Current player and piece counts
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

        // Removed AI thinking message due to rendering issues
        stdout.write_all(b"\n\r")?;

        // Game over message
        if game.is_game_over {
            stdout.write_all(b"\n\r")?;
            if offset > 0 {
                write!(stdout, "{}", " ".repeat(offset))?;
            }
            stdout.queue(SetForegroundColor(Color::Yellow))?;
            stdout.queue(SetAttribute(Attribute::Bold))?;
            if let Some(winner) = game.check_winner() {
                write!(stdout, "🏆 GAME OVER! {:?} WINS! 🏆", winner)?;
                stdout.write_all(b"\n\r")?;
            } else if game.is_stalemate() {
                write!(stdout, "🤝 GAME OVER! STALEMATE! 🤝")?;
                stdout.write_all(b"\n\r")?;
            }
            stdout.queue(SetAttribute(Attribute::Reset))?;
            stdout.queue(ResetColor)?;
        }

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
            "Controls: ↑↓←→ Move | Space/Enter Select | Q/Esc Quit"
        )?;
        stdout.write_all(b"\n\r")?;

        Ok(())
    }

    pub fn render_game(&mut self, game: &CheckersGame) -> io::Result<()> {
        let mut stdout = stdout();

        // Queue all operations first to minimize flicker
        stdout.queue(Hide)?;

        if self.first_render {
            stdout.queue(Clear(ClearType::All))?;
            self.first_render = false;
        }

        stdout.queue(cursor::MoveTo(0, 0))?;

        self.render_game_status(&mut stdout, game)?;
        stdout.write_all(b"\n\r")?;
        Self::render_column_headers(&mut stdout, game.board.size)?;
        self.render_board_rows(&mut stdout, game)?;
        self.render_controls(&mut stdout)?;

        // Clear any remaining lines after the content
        stdout.queue(Clear(ClearType::FromCursorDown))?;

        stdout.flush()?;
        Ok(())
    }
}
