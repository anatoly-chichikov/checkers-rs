use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::Widget,
};

use crate::core::{board::Board, piece::Color as PieceColor};

pub struct CheckerBoard<'a> {
    board: &'a Board,
    cursor_pos: (usize, usize),
    selected_square: Option<(usize, usize)>,
    possible_moves: &'a [(usize, usize)],
}

impl<'a> CheckerBoard<'a> {
    pub fn new(board: &'a Board) -> Self {
        Self {
            board,
            cursor_pos: (0, 0),
            selected_square: None,
            possible_moves: &[],
        }
    }

    pub fn cursor_position(mut self, pos: (usize, usize)) -> Self {
        self.cursor_pos = pos;
        self
    }

    pub fn selected_square(mut self, square: Option<(usize, usize)>) -> Self {
        self.selected_square = square;
        self
    }

    pub fn possible_moves(mut self, moves: &'a [(usize, usize)]) -> Self {
        self.possible_moves = moves;
        self
    }

    fn render_cell(&self, buf: &mut Buffer, x: u16, y: u16, row: usize, col: usize) {
        let piece = self.board.get_piece(row, col);
        let is_possible_move = self.possible_moves.contains(&(row, col));

        // Determine cell background
        let cell_style = if is_possible_move {
            Style::default().bg(Color::Red)
        } else {
            Style::default()
        };

        // Check if it's a playable square (dark squares in checkers)
        let is_playable = (row + col) % 2 == 1;

        // Cell content (5 chars wide)
        let content = match piece {
            None => {
                if is_playable {
                    "     ".to_string()
                } else {
                    " ░░░ ".to_string()
                }
            }
            Some(p) => {
                let piece_char = match (p.color, p.is_king) {
                    (PieceColor::Black, false) => "(b)",
                    (PieceColor::Black, true) => "(B)",
                    (PieceColor::White, false) => "(w)",
                    (PieceColor::White, true) => "(W)",
                };
                format!(" {} ", piece_char)
            }
        };

        // Render content
        let content_style = match piece {
            Some(p) if p.color == PieceColor::Black => cell_style.fg(Color::Red),
            Some(p) if p.color == PieceColor::White => cell_style.fg(Color::White),
            None if !is_playable => cell_style.fg(Color::Magenta),
            _ => cell_style,
        };

        buf.set_string(x, y, &content, content_style);
    }

    fn get_border_chars(
        &self,
        row: usize,
        col: usize,
    ) -> (char, char, char, char, char, char, char, char, char) {
        let is_cursor = (row, col) == self.cursor_pos;

        if is_cursor {
            // Double line box for cursor
            ('╔', '═', '╗', '║', '║', '╚', '═', '╝', '─')
        } else {
            // Single line box
            ('┌', '─', '┐', '│', '│', '└', '─', '┘', '─')
        }
    }
}

impl<'a> Widget for CheckerBoard<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Calculate dimensions
        let cell_width = 6; // 5 content + 1 border
        let cell_height = 2; // 1 content + 1 border
        let board_width = cell_width * 8 + 1; // +1 for final border
        let board_height = cell_height * 8 + 1; // +1 for final border

        // Add space for coordinates
        let total_width = board_width + 4; // 4 chars for row numbers ("  8 ")
        let total_height = board_height + 1; // 1 row for column letters (reduced from 2)

        if area.width < total_width || area.height < total_height {
            return; // Not enough space
        }

        // Center the board horizontally, minimal vertical centering
        let x_offset = (area.width - total_width) / 2 + area.x;
        let y_offset = area
            .y
            .saturating_add((area.height.saturating_sub(total_height)) / 4); // Use 1/4 instead of 1/2 for vertical centering

        // Draw column labels (aligned with board cells)
        // Column labels should start where the board cells start (after row numbers)
        let col_labels = "A     B     C     D     E     F     G     H";
        buf.set_string(
            x_offset + 4,  // Same offset as board cells
            y_offset,
            col_labels,
            Style::default().fg(Color::White),
        );

        // Draw the board
        for row in 0..8 {
            let y_pos = y_offset + 1 + row as u16 * cell_height; // Reduced from +2 to +1

            // Draw row number (1 at bottom, 8 at top)
            buf.set_string(
                x_offset,
                y_pos + cell_height / 2,
                &format!("  {} ", 8 - row),
                Style::default().fg(Color::White),
            );

            // Draw cells
            for col in 0..8 {
                let x_pos = x_offset + 4 + col as u16 * cell_width;

                // Draw cell borders
                let (tl, t, tr, l, r, bl, b, br, _) = self.get_border_chars(row, col);
                let border_style = if (row, col) == self.cursor_pos {
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Magenta)
                };

                // Top border
                if row == 0 || (row, col) == self.cursor_pos {
                    buf.set_string(x_pos, y_pos, &tl.to_string(), border_style);
                    for i in 1..cell_width {
                        buf.set_string(x_pos + i, y_pos, &t.to_string(), border_style);
                    }
                    if col == 7 {
                        buf.set_string(x_pos + cell_width, y_pos, &tr.to_string(), border_style);
                    }
                }

                // Side borders and content
                buf.set_string(x_pos, y_pos + 1, &l.to_string(), border_style);
                self.render_cell(buf, x_pos + 1, y_pos + 1, row, col);
                if col == 7 {
                    buf.set_string(x_pos + cell_width, y_pos + 1, &r.to_string(), border_style);
                }

                // Bottom border
                if row == 7 || (row + 1 < 8 && (row + 1, col) == self.cursor_pos) {
                    buf.set_string(x_pos, y_pos + cell_height, &bl.to_string(), border_style);
                    for i in 1..cell_width {
                        buf.set_string(
                            x_pos + i,
                            y_pos + cell_height,
                            &b.to_string(),
                            border_style,
                        );
                    }
                    if col == 7 {
                        buf.set_string(
                            x_pos + cell_width,
                            y_pos + cell_height,
                            &br.to_string(),
                            border_style,
                        );
                    }
                }

                // Intersections
                if row < 7
                    && col < 7
                    && (row, col) != self.cursor_pos
                    && (row + 1, col) != self.cursor_pos
                {
                    buf.set_string(
                        x_pos + cell_width,
                        y_pos + cell_height,
                        "┼",
                        Style::default().fg(Color::Magenta),
                    );
                }
            }
        }
    }
}
