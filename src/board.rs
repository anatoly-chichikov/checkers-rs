use crate::piece::{Color, Piece};

#[derive(Clone, Debug)]
pub struct Board {
    pub size: usize,
    pub cells: Vec<Vec<Option<Piece>>>,
}

impl Board {
    pub fn new(size: usize) -> Self {
        let cells = vec![vec![None; size]; size];
        Self { size, cells }
    }

    pub fn initialize(&mut self) {
        // Clear the board first
        for row in 0..self.size {
            for col in 0..self.size {
                self.cells[row][col] = None;
            }
        }

        // Place pieces in the standard initial layout
        for row in 0..3 {
            if row % 2 == 0 {
                for col in (1..self.size).step_by(2) {
                    self.cells[row][col] = Some(Piece::new(Color::Black));
                }
            } else {
                for col in (0..self.size).step_by(2) {
                    self.cells[row][col] = Some(Piece::new(Color::Black));
                }
            }
        }

        for row in (self.size - 3)..self.size {
            if row % 2 == 0 {
                for col in (1..self.size).step_by(2) {
                    self.cells[row][col] = Some(Piece::new(Color::White));
                }
            } else {
                for col in (0..self.size).step_by(2) {
                    self.cells[row][col] = Some(Piece::new(Color::White));
                }
            }
        }
    }

    pub fn get_piece(&self, row: usize, col: usize) -> Option<Piece> {
        if self.in_bounds(row, col) {
            self.cells[row][col]
        } else {
            None
        }
    }

    pub fn set_piece(&mut self, row: usize, col: usize, piece: Option<Piece>) -> bool {
        if self.in_bounds(row, col) {
            self.cells[row][col] = piece;
            true
        } else {
            false
        }
    }

    pub fn in_bounds(&self, row: usize, col: usize) -> bool {
        row < self.size && col < self.size
    }

    pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) -> bool {
        if !self.in_bounds(from.0, from.1) || !self.in_bounds(to.0, to.1) {
            return false;
        }

        let piece = self.get_piece(from.0, from.1);
        if piece.is_none() {
            return false;
        }

        self.set_piece(to.0, to.1, piece);
        self.set_piece(from.0, from.1, None);
        true
    }
}
