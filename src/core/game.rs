use crate::core::board::Board;
use crate::core::game_logic;
use crate::core::piece::Color;
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
        if !game_logic::is_valid_move(&self.board, from_row, from_col, to_row, to_col, &piece) {
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
        if game_logic::should_promote(&piece, to_row, self.board.size) {
            if let Some(mut promoted_piece) = self.board.get_piece(to_row, to_col) {
                promoted_piece.promote_to_king();
                self.board.set_piece(to_row, to_col, Some(promoted_piece));
            }
        }

        // After a capture, check if there are more captures available for the same piece
        if row_diff == 2 && game_logic::has_more_captures_for_piece(&self.board, to_row, to_col) {
            self.selected_piece = Some((to_row, to_col));
            return Ok(());
        }

        self.selected_piece = None;
        self.switch_player();
        Ok(())
    }

    pub fn switch_player(&mut self) {
        self.current_player = self.current_player.opposite();
    }

    pub fn check_winner(&self) -> Option<Color> {
        game_logic::check_winner(&self.board)
    }

    pub fn has_captures_available(&self) -> bool {
        game_logic::has_captures_available(&self.board, self.current_player)
    }

    pub fn is_stalemate(&self) -> bool {
        game_logic::is_stalemate(&self.board, self.current_player)
    }
}
