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

fn format_position(pos: (usize, usize)) -> String {
    format!("{}{}", (b'A' + pos.1 as u8) as char, pos.0 + 1)
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

    fn get_cell_border_style(&self, game: &CheckersGame, cell_pos: (usize, usize)) -> Color {
        if cell_pos == self.cursor_pos {
            Color::Yellow
        } else if game.selected_piece == Some(cell_pos) {
            Color::Green
        } else if game
            .possible_moves
            .as_ref()
            .is_some_and(|m| m.contains(&cell_pos))
        {
            Color::Cyan
        } else {
            Color::DarkGrey
        }
    }

    fn render_column_headers(stdout: &mut io::Stdout, board_size: usize) -> io::Result<()> {
        stdout.queue(SetForegroundColor(Color::Blue))?;
        stdout.write_all(b"   ")?;
        for col in 0..board_size {
            write!(stdout, "   {}   ", (b'A' + col as u8) as char)?;
        }
        stdout.queue(ResetColor)?;
        stdout.write_all(b"\n\r")?;
        Ok(())
    }

    fn render_board_rows(&self, stdout: &mut io::Stdout, game: &CheckersGame) -> io::Result<()> {
        for row in 0..game.board.size {
            stdout.write_all(b"   ")?;
            for col in 0..game.board.size {
                let cell_border_color = self.get_cell_border_style(game, (row, col));
                stdout.queue(SetForegroundColor(cell_border_color))?;
                stdout.write_all(b"+-----+")?;
                stdout.queue(ResetColor)?;
            }
            stdout.write_all(b"\n\r")?;

            stdout.queue(SetForegroundColor(Color::Blue))?;
            write!(stdout, "{:2} ", row + 1)?;
            stdout.queue(ResetColor)?;

            for col in 0..game.board.size {
                let cell_border_color = self.get_cell_border_style(game, (row, col));

                let (content_to_display, text_color_for_content) =
                    match game.board.get_piece(row, col) {
                        Some(piece) => (
                            format!(" {} ", piece.display()),
                            match piece.color {
                                PieceColor::White => Color::White,
                                PieceColor::Black => Color::Red,
                            },
                        ),
                        None => {
                            if (row + col) % 2 == 0 {
                                ("     ".to_string(), Color::DarkGrey)
                            } else {
                                (" ░░░ ".to_string(), Color::DarkGrey)
                            }
                        }
                    };

                stdout.queue(SetForegroundColor(cell_border_color))?;
                write!(stdout, "|")?;
                stdout.queue(ResetColor)?;

                stdout.queue(SetForegroundColor(text_color_for_content))?;
                write!(stdout, "{}", content_to_display)?;
                stdout.queue(ResetColor)?;

                stdout.queue(SetForegroundColor(cell_border_color))?;
                write!(stdout, "|")?;
                stdout.queue(ResetColor)?;
            }
            stdout.write_all(b"\n\r")?;

            stdout.write_all(b"   ")?;
            for col in 0..game.board.size {
                let cell_border_color = self.get_cell_border_style(game, (row, col));
                stdout.queue(SetForegroundColor(cell_border_color))?;
                stdout.write_all(b"+-----+")?;
                stdout.queue(ResetColor)?;
            }
            stdout.write_all(b"\n\r")?;
        }
        Ok(())
    }

    fn render_game_status(&self, stdout: &mut io::Stdout, game: &CheckersGame) -> io::Result<()> {
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
            writeln!(stdout, "Selected piece at: {}", format_position(selected))?;
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

        stdout.write_all(b"\n\r")?;
        stdout.queue(SetForegroundColor(Color::DarkGrey))?;
        writeln!(
            stdout,
            "Controls: ↑↓←→ Move | Space/Enter Select | Q/Esc Quit"
        )?;
        stdout.queue(ResetColor)?;

        Ok(())
    }

    pub fn render_game(&self, game: &CheckersGame) -> io::Result<()> {
        let mut stdout = stdout();

        stdout.queue(Clear(ClearType::All))?;
        stdout.queue(cursor::MoveTo(0, 0))?;

        Self::render_column_headers(&mut stdout, game.board.size)?;
        self.render_board_rows(&mut stdout, game)?;
        self.render_game_status(&mut stdout, game)?;

        stdout.flush()?;
        Ok(())
    }
}
