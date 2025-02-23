use crossterm::{
    cursor,
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use std::io::{self, stdout, Write};

use crate::game::CheckersGame;

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

        // Print column headers
        stdout.write_all(b"   ")?;  // Extra space for alignment
        for col in 0..game.board.size {
            write!(stdout, " {} ", (b'A' + col as u8) as char)?;
        }
        stdout.write_all(b"\n\r")?;

        // Print board with row numbers
        for row in 0..game.board.size {
            write!(stdout, "{:2} ", row + 1)?;
            for col in 0..game.board.size {
                let is_cursor_here = (row, col) == self.cursor_pos;
                let is_selected = game.selected_piece == Some((row, col));

                let cell = match game.board.get_piece(row, col) {
                    Some(piece) => piece.display(),
                    None => '.',
                };

                if is_cursor_here {
                    write!(stdout, "[{}]", cell)?;
                } else if is_selected {
                    write!(stdout, "({0})", cell)?;
                } else {
                    write!(stdout, " {0} ", cell)?;
                }
            }
            stdout.write_all(b"\n\r")?;
        }

        // Print game status
        stdout.write_all(b"\n\rCurrent player: ")?;
        writeln!(stdout, "{:?}", game.current_player)?;
        
        if let Some(selected) = game.selected_piece {
            writeln!(stdout, "Selected piece at: {}", Self::format_position(selected))?;
        }

        if game.is_game_over {
            if let Some(winner) = game.check_winner() {
                writeln!(stdout, "\nGame Over! {:?} wins!", winner)?;
            } else if game.is_stalemate() {
                writeln!(stdout, "\nGame Over! Stalemate!")?;
            }
        }

        stdout.flush()?;
        Ok(())
    }

    fn format_position(pos: (usize, usize)) -> String {
        format!("{}{}", (b'A' + pos.1 as u8) as char, pos.0 + 1)
    }
} 