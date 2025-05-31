use crossterm::{
    cursor,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use std::io::{self, stdout, Write};

use crate::core::game::CheckersGame;
use crate::core::piece::Color as PieceColor;

pub struct UI {
    cursor_pos: (usize, usize),
}

impl Default for UI {
    fn default() -> Self {
        Self::new()
    }
}

impl UI {
    pub fn new() -> Self {
        Self { cursor_pos: (0, 0) }
    }

    pub fn set_cursor(&mut self, row: usize, col: usize) {
        self.cursor_pos = (row, col);
    }

    pub fn get_cursor(&self) -> (usize, usize) {
        self.cursor_pos
    }

    pub fn render_game(&self, game: &CheckersGame) -> io::Result<()> {
        let mut stdout = stdout();

        // Clear screen and move cursor to top-left
        stdout.queue(Clear(ClearType::All))?;
        stdout.queue(cursor::MoveTo(0, 0))?;

        // Print column headers with blue color
        stdout.queue(SetForegroundColor(Color::Blue))?;
        stdout.write_all(b"   ")?; // Extra space for alignment
        for col in 0..game.board.size {
            // Adjust for wider cells: "+-----+" is 7 chars wide
            write!(stdout, "   {}   ", (b'A' + col as u8) as char)?;
        }
        stdout.queue(ResetColor)?;
        stdout.write_all(b"\n\r")?;

        // Print board with row numbers
        for row in 0..game.board.size {
            // Top border of the cell row
            stdout.write_all(b"   ")?; // Align with row numbers
            for col in 0..game.board.size {
                let is_cursor_here = (row, col) == self.cursor_pos;
                let is_selected = game.selected_piece == Some((row, col));
                let mut is_possible_move = false;
                if let Some(possible_moves_vec) = &game.possible_moves {
                    if possible_moves_vec.contains(&(row, col)) {
                        is_possible_move = true;
                    }
                }
                let cell_border_color = if is_cursor_here {
                    Color::Yellow
                } else if is_selected {
                    Color::Green
                } else if is_possible_move {
                    Color::Cyan
                } else {
                    Color::DarkGrey
                };
                stdout.queue(SetForegroundColor(cell_border_color))?;
                stdout.write_all(b"+-----+")?;
                stdout.queue(ResetColor)?;
            }
            stdout.write_all(b"\n\r")?;

            // Content line of the cell row
            stdout.queue(SetForegroundColor(Color::Blue))?;
            write!(stdout, "{:2} ", row + 1)?;
            stdout.queue(ResetColor)?;

            for col in 0..game.board.size {
                let is_cursor_here = (row, col) == self.cursor_pos;
                let is_selected = game.selected_piece == Some((row, col));
                let mut is_possible_move = false;
                if let Some(possible_moves_vec) = &game.possible_moves {
                    if possible_moves_vec.contains(&(row, col)) {
                        is_possible_move = true;
                    }
                }

                // Get the piece representation and its color
                // piece.display() now returns a 3-char String, e.g., "(w)"
                // For empty cells, we'll use "  _  "
                let (content_to_display, text_color_for_content) =
                    match game.board.get_piece(row, col) {
                        Some(piece) => {
                            let piece_repr = piece.display(); // This is a 3-char String
                            (
                                format!(" {} ", piece_repr), // Pad to 5 chars, e.g., " (w) "
                                match piece.color {
                                    PieceColor::White => Color::White,
                                    PieceColor::Black => Color::Red,
                                },
                            )
                        }
                        None => ("  _  ".to_string(), Color::DarkGrey), // Placeholder is already 5 chars
                    };

                let cell_bg_color = if is_cursor_here {
                    Color::Yellow
                } else if is_selected {
                    Color::Green
                } else if is_possible_move {
                    Color::Cyan
                } else {
                    // For default cells, we might not want a specific background for the whole cell,
                    // but the borders should be distinct. Using DarkGrey for borders of normal cells.
                    Color::DarkGrey
                };

                // Draw the left border of the cell
                stdout.queue(SetForegroundColor(cell_bg_color))?;
                write!(stdout, "|")?;
                stdout.queue(ResetColor)?;

                // Render the 5-character content string (already padded)
                stdout.queue(SetForegroundColor(text_color_for_content))?;
                write!(stdout, "{}", content_to_display)?;
                stdout.queue(ResetColor)?;

                // Draw the right border of the cell
                stdout.queue(SetForegroundColor(cell_bg_color))?;
                write!(stdout, "|")?;
                stdout.queue(ResetColor)?; // Reset color after cell
            }
            stdout.write_all(b"\n\r")?;

            // Bottom border of the cell row
            stdout.write_all(b"   ")?; // Align with row numbers
            for col in 0..game.board.size {
                let is_cursor_here = (row, col) == self.cursor_pos;
                let is_selected = game.selected_piece == Some((row, col));
                let mut is_possible_move = false;
                if let Some(possible_moves_vec) = &game.possible_moves {
                    if possible_moves_vec.contains(&(row, col)) {
                        is_possible_move = true;
                    }
                }
                let cell_border_color = if is_cursor_here {
                    Color::Yellow
                } else if is_selected {
                    Color::Green
                } else if is_possible_move {
                    Color::Cyan
                } else {
                    Color::DarkGrey
                };
                stdout.queue(SetForegroundColor(cell_border_color))?;
                stdout.write_all(b"+-----+")?;
                stdout.queue(ResetColor)?;
            }
            stdout.write_all(b"\n\r")?;
        }

        // Print game status
        stdout.write_all(b"\n\rCurrent player: ")?;
        let player_color = match game.current_player {
            PieceColor::White => Color::White,
            PieceColor::Black => Color::Red,
        };
        stdout.queue(SetForegroundColor(player_color))?;
        writeln!(stdout, "{:?}", game.current_player)?;
        stdout.queue(ResetColor)?;

        if let Some(selected) = game.selected_piece {
            stdout.queue(SetForegroundColor(Color::Green))?;
            writeln!(
                stdout,
                "Selected piece at: {}",
                Self::format_position(selected)
            )?;
            stdout.queue(ResetColor)?;
        }

        if game.is_game_over {
            if let Some(winner) = game.check_winner() {
                stdout.queue(SetForegroundColor(Color::Yellow))?;
                writeln!(stdout, "\nGame Over! {:?} wins!", winner)?;
                stdout.queue(ResetColor)?;
            } else if game.is_stalemate() {
                stdout.queue(SetForegroundColor(Color::Yellow))?;
                writeln!(stdout, "\nGame Over! Stalemate!")?;
                stdout.queue(ResetColor)?;
            }
        }

        stdout.flush()?;
        Ok(())
    }

    fn format_position(pos: (usize, usize)) -> String {
        format!("{}{}", (b'A' + pos.1 as u8) as char, pos.0 + 1)
    }
}
