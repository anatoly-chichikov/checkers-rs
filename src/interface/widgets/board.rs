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
        // Grid dimensions
        const CELL_WIDTH: u16 = 6;  // 5 content + 1 border
        const CELL_HEIGHT: u16 = 2; // 1 content + 1 border
        const LABEL_WIDTH: u16 = 4; // Row labels width
        
        // Calculate total grid size (9x9 including labels)
        let grid_width = LABEL_WIDTH + CELL_WIDTH * 8 + 1; // +1 for final border
        let grid_height = 1 + CELL_HEIGHT * 8 + 1; // 1 for column labels, +1 for final border
        
        if area.width < grid_width || area.height < grid_height {
            return; // Not enough space
        }
        
        // Center the entire grid
        let x_start = (area.width.saturating_sub(grid_width)) / 2 + area.x;
        let y_start = (area.height.saturating_sub(grid_height)) / 4 + area.y; // 1/4 vertical offset
        
        // Draw column labels row
        buf.set_string(
            x_start,
            y_start,
            "    ", // Empty space for row label column
            Style::default(),
        );
        
        for col in 0..8 {
            let x = x_start + LABEL_WIDTH + col * CELL_WIDTH + CELL_WIDTH / 2;
            buf.set_string(
                x,
                y_start,
                &format!("{}", (b'A' + col as u8) as char),
                Style::default().fg(Color::White),
            );
        }
        
        // Draw board rows with labels
        for row in 0..8 {
            let y_base = y_start + 1 + row * CELL_HEIGHT;
            
            // Draw row label
            let row_label = format!("{:>2} ", 8 - row);
            buf.set_string(
                x_start,
                y_base + CELL_HEIGHT / 2,
                &row_label,
                Style::default().fg(Color::White),
            );
            
            // Draw cells in this row
            for col in 0..8 {
                let x_pos = x_start + LABEL_WIDTH + col * CELL_WIDTH;
                let y_pos = y_base;

                // Draw cell borders
                let (tl, t, tr, l, r, bl, b, br, _) = self.get_border_chars(row as usize, col as usize);
                let border_style = if (row as usize, col as usize) == self.cursor_pos {
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Magenta)
                };

                // Top border
                if row == 0 || (row as usize, col as usize) == self.cursor_pos {
                    buf.set_string(x_pos, y_pos, &tl.to_string(), border_style);
                    for i in 1..CELL_WIDTH {
                        buf.set_string(x_pos + i, y_pos, &t.to_string(), border_style);
                    }
                    if col == 7 {
                        buf.set_string(x_pos + CELL_WIDTH, y_pos, &tr.to_string(), border_style);
                    }
                }

                // Side borders and content
                buf.set_string(x_pos, y_pos + 1, &l.to_string(), border_style);
                self.render_cell(buf, x_pos + 1, y_pos + 1, row as usize, col as usize);
                if col == 7 {
                    buf.set_string(x_pos + CELL_WIDTH, y_pos + 1, &r.to_string(), border_style);
                }

                // Bottom border
                if row == 7 || (row + 1 < 8 && (row as usize + 1, col as usize) == self.cursor_pos) {
                    buf.set_string(x_pos, y_pos + CELL_HEIGHT, &bl.to_string(), border_style);
                    for i in 1..CELL_WIDTH {
                        buf.set_string(
                            x_pos + i,
                            y_pos + CELL_HEIGHT,
                            &b.to_string(),
                            border_style,
                        );
                    }
                    if col == 7 {
                        buf.set_string(
                            x_pos + CELL_WIDTH,
                            y_pos + CELL_HEIGHT,
                            &br.to_string(),
                            border_style,
                        );
                    }
                }

                // Intersections
                if row < 7
                    && col < 7
                    && (row as usize, col as usize) != self.cursor_pos
                    && (row as usize + 1, col as usize) != self.cursor_pos
                {
                    buf.set_string(
                        x_pos + CELL_WIDTH,
                        y_pos + CELL_HEIGHT,
                        "┼",
                        Style::default().fg(Color::Magenta),
                    );
                }
            }
        }
    }
}
