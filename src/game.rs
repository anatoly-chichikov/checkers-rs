use crate::board::Board;
use crate::piece::{Color, Piece};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameError {
    #[error("No piece at selected position")]
    NoPieceSelected,
    #[error("Selected piece belongs to the opponent")]
    WrongPieceColor,
    #[error("Invalid move")]
    InvalidMove,
    #[error("Forced capture available")]
    ForcedCaptureAvailable,
    #[error("Position out of bounds")]
    OutOfBounds,
}

pub struct CheckersGame {
    pub board: Board,
    pub current_player: Color,
    pub is_game_over: bool,
    pub selected_piece: Option<(usize, usize)>,
}

impl Default for CheckersGame {
    fn default() -> Self {
        Self::new()
    }
}

impl CheckersGame {
    pub fn new() -> Self {
        let mut board = Board::new(8);
        board.initialize();
        Self {
            board,
            current_player: Color::White,
            is_game_over: false,
            selected_piece: None,
        }
    }

    pub fn select_piece(&mut self, row: usize, col: usize) -> Result<(), GameError> {
        if !self.board.in_bounds(row, col) {
            return Err(GameError::OutOfBounds);
        }

        // If selecting the same piece that's already selected, deselect it
        if self.selected_piece == Some((row, col)) {
            self.selected_piece = None;
            return Ok(());
        }

        match self.board.get_piece(row, col) {
            Some(piece) if piece.color == self.current_player => {
                self.selected_piece = Some((row, col));
                Ok(())
            }
            Some(_) => Err(GameError::WrongPieceColor),
            None => Err(GameError::NoPieceSelected),
        }
    }

    pub fn make_move(&mut self, to_row: usize, to_col: usize) -> Result<(), GameError> {
        let (from_row, from_col) = self.selected_piece.ok_or(GameError::NoPieceSelected)?;

        if !self.board.in_bounds(to_row, to_col) {
            return Err(GameError::OutOfBounds);
        }

        let piece = self
            .board
            .get_piece(from_row, from_col)
            .ok_or(GameError::NoPieceSelected)?;

        // Check if there are any captures available
        if self.has_captures_available() {
            let row_diff = (to_row as i32 - from_row as i32).abs();
            if row_diff != 2 {
                return Err(GameError::ForcedCaptureAvailable);
            }
        }

        // Check if the move is valid
        if !self.is_valid_move(from_row, from_col, to_row, to_col, &piece) {
            return Err(GameError::InvalidMove);
        }

        // If it's a capture move, remove the captured piece
        let row_diff = (to_row as i32 - from_row as i32).abs();
        if row_diff == 2 {
            let mid_row = (from_row + to_row) / 2;
            let mid_col = (from_col + to_col) / 2;
            self.board.set_piece(mid_row, mid_col, None);
        }

        // Move the piece
        self.board
            .move_piece((from_row, from_col), (to_row, to_col));

        // Check for promotion and promote if necessary
        if self.board.should_promote(&piece, to_row) {
            if let Some(mut promoted_piece) = self.board.get_piece(to_row, to_col) {
                promoted_piece.promote_to_king();
                self.board.set_piece(to_row, to_col, Some(promoted_piece));
            }
        }

        // After a capture, check if there are more captures available for the same piece
        if row_diff == 2 && self.has_more_captures_for_piece(to_row, to_col) {
            self.selected_piece = Some((to_row, to_col));
            return Ok(());
        }

        self.selected_piece = None;
        self.switch_player();
        Ok(())
    }

    fn has_more_captures_for_piece(&self, row: usize, col: usize) -> bool {
        if let Some(piece) = self.board.get_piece(row, col) {
            let directions = if piece.is_king {
                vec![(2, 2), (2, -2), (-2, 2), (-2, -2)]
            } else {
                match piece.color {
                    Color::White => vec![(-2, 2), (-2, -2)],
                    Color::Black => vec![(2, 2), (2, -2)],
                }
            };

            for (row_diff, col_diff) in directions {
                let next_row = (row as i32 + row_diff) as usize;
                let next_col = (col as i32 + col_diff) as usize;

                if self.board.in_bounds(next_row, next_col) && self.is_valid_move(row, col, next_row, next_col, &piece) {
                    return true;
                }
            }
        }
        false
    }

    fn is_valid_move(
        &self,
        from_row: usize,
        from_col: usize,
        to_row: usize,
        to_col: usize,
        piece: &Piece,
    ) -> bool {
        // Basic checks
        if !self.board.in_bounds(to_row, to_col) {
            return false;
        }

        if self.board.get_piece(to_row, to_col).is_some() {
            return false;
        }

        let row_diff = to_row as i32 - from_row as i32;
        let col_diff = to_col as i32 - from_col as i32;

        // Check diagonal movement
        if col_diff.abs() != row_diff.abs() {
            return false;
        }

        // Regular move
        if row_diff.abs() == 1 {
            return piece.is_king
                || match piece.color {
                    Color::White => row_diff < 0,
                    Color::Black => row_diff > 0,
                };
        }

        // Capture move
        if row_diff.abs() == 2 && col_diff.abs() == 2 {
            let mid_row = ((from_row as i32 + to_row as i32) / 2) as usize;
            let mid_col = ((from_col as i32 + to_col as i32) / 2) as usize;

            // Check if there's an opponent's piece to capture
            if let Some(captured_piece) = self.board.get_piece(mid_row, mid_col) {
                if captured_piece.color == piece.color {
                    return false;
                }

                return piece.is_king
                    || match piece.color {
                        Color::White => row_diff < 0,
                        Color::Black => row_diff > 0,
                    };
            }
        }

        false
    }

    pub fn switch_player(&mut self) {
        self.current_player = self.current_player.opposite();
    }

    pub fn check_winner(&self) -> Option<Color> {
        let mut white_pieces = false;
        let mut black_pieces = false;

        for row in 0..self.board.size {
            for col in 0..self.board.size {
                if let Some(piece) = self.board.get_piece(row, col) {
                    match piece.color {
                        Color::White => white_pieces = true,
                        Color::Black => black_pieces = true,
                    }
                    if white_pieces && black_pieces {
                        return None;
                    }
                }
            }
        }

        match (white_pieces, black_pieces) {
            (true, false) => Some(Color::White),
            (false, true) => Some(Color::Black),
            _ => None,
        }
    }

    pub fn has_captures_available(&self) -> bool {
        for row in 0..self.board.size {
            for col in 0..self.board.size {
                if let Some(piece) = self.board.get_piece(row, col) {
                    if piece.color == self.current_player {
                        let directions = if piece.is_king {
                            vec![(2, 2), (2, -2), (-2, 2), (-2, -2)]
                        } else {
                            match piece.color {
                                Color::White => vec![(-2, 2), (-2, -2)],
                                Color::Black => vec![(2, 2), (2, -2)],
                            }
                        };

                        for (row_diff, col_diff) in directions {
                            let to_row = (row as i32 + row_diff) as usize;
                            let to_col = (col as i32 + col_diff) as usize;

                            if self.board.in_bounds(to_row, to_col) && self.is_valid_move(row, col, to_row, to_col, &piece) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    pub fn is_stalemate(&self) -> bool {
        // First check if there are any captures available
        if self.has_captures_available() {
            return false;
        }

        // If no captures are available, check for regular moves
        for row in 0..self.board.size {
            for col in 0..self.board.size {
                if let Some(piece) = self.board.get_piece(row, col) {
                    if piece.color == self.current_player {
                        let directions = if piece.is_king {
                            vec![(1, 1), (1, -1), (-1, 1), (-1, -1)]
                        } else {
                            match piece.color {
                                Color::White => vec![(-1, 1), (-1, -1)],
                                Color::Black => vec![(1, 1), (1, -1)],
                            }
                        };

                        for (row_diff, col_diff) in directions {
                            let to_row = match (row as i32 + row_diff).try_into() {
                                Ok(val) => val,
                                Err(_) => continue,
                            };
                            let to_col = match (col as i32 + col_diff).try_into() {
                                Ok(val) => val,
                                Err(_) => continue,
                            };

                            if self.board.in_bounds(to_row, to_col) && self.is_valid_move(row, col, to_row, to_col, &piece) {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        true
    }
}
