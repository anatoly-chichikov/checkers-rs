use crossterm::{
    cursor,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use std::io::{self, stdout, Write};

use crate::game::CheckersGame;
use crate::piece::Color as PieceColor;

pub struct UI {
    cursor_pos: (usize, usize),
}

impl UI {
    pub fn new() -> Self {
        Self {
            cursor_pos: (0, 0),
        }
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
        stdout.write_all(b"   ")?;  // Extra space for alignment
        for col in 0..game.board.size {
            write!(stdout, " {} ", (b'A' + col as u8) as char)?;
        }
        stdout.queue(ResetColor)?;
        stdout.write_all(b"\n\r")?;

        // Print board with row numbers
        for row in 0..game.board.size {
            stdout.queue(SetForegroundColor(Color::Blue))?;
            write!(stdout, "{:2} ", row + 1)?;
            stdout.queue(ResetColor)?;
            
            for col in 0..game.board.size {
                let is_cursor_here = (row, col) == self.cursor_pos;
                let is_selected = game.selected_piece == Some((row, col));

                let (piece_char, piece_color) = match game.board.get_piece(row, col) {
                    Some(piece) => (piece.display(), match piece.color {
                        PieceColor::White => Color::White,
                        PieceColor::Black => Color::Red,
                    }),
                    None => ('.', Color::DarkGrey),
                };

                if is_cursor_here {
                    stdout.queue(SetForegroundColor(Color::Yellow))?;
                    write!(stdout, "[{}]", piece_char)?;
                } else if is_selected {
                    stdout.queue(SetForegroundColor(Color::Green))?;
                    write!(stdout, "({0})", piece_char)?;
                } else {
                    stdout.queue(SetForegroundColor(piece_color))?;
                    write!(stdout, " {0} ", piece_char)?;
                }
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
            writeln!(stdout, "Selected piece at: {}", Self::format_position(selected))?;
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