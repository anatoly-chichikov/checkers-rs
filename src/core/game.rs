use crate::core::board::Board;
use crate::core::game_logic::{self, can_piece_capture};
use crate::core::move_history::MoveHistory;
use crate::core::piece::Color;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameError {
    #[error("No piece at selected position")]
    NoPieceSelected,
    #[error("Selected piece belongs to the opponent")]
    #[allow(dead_code)]
    WrongPieceColor,
    #[error("Invalid move")]
    InvalidMove,
    #[error("Forced capture available")]
    ForcedCaptureAvailable,
    #[error("Position out of bounds")]
    OutOfBounds,
}

#[derive(Clone)]
pub struct CheckersGame {
    pub board: Board,
    pub current_player: Color,
    pub is_game_over: bool,
    pub move_history: MoveHistory,
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
            move_history: MoveHistory::new(),
        }
    }

    #[allow(dead_code)]
    pub fn validate_piece_selection(&self, row: usize, col: usize) -> Result<(), GameError> {
        if !self.board.in_bounds(row, col) {
            return Err(GameError::OutOfBounds);
        }

        match self.board.get_piece(row, col) {
            Some(piece) if piece.color == self.current_player => {
                if self.has_captures_available() && !can_piece_capture(&self.board, row, col) {
                    return Err(GameError::ForcedCaptureAvailable);
                }
                Ok(())
            }
            Some(_) => Err(GameError::WrongPieceColor),
            None => Err(GameError::NoPieceSelected),
        }
    }

    pub fn make_move(&mut self, from_row: usize, from_col: usize, to_row: usize, to_col: usize) -> Result<bool, GameError> {

        if !self.board.in_bounds(to_row, to_col) {
            return Err(GameError::OutOfBounds);
        }

        let piece = self
            .board
            .get_piece(from_row, from_col)
            .ok_or(GameError::NoPieceSelected)?;

        if self.has_captures_available() {
            let row_diff = (to_row as i32 - from_row as i32).abs();
            if row_diff != 2 {
                return Err(GameError::ForcedCaptureAvailable);
            }
        }

        if !game_logic::is_valid_move(&self.board, from_row, from_col, to_row, to_col, &piece) {
            return Err(GameError::InvalidMove);
        }

        let row_diff_abs = (to_row as i32 - from_row as i32).abs();
        let mut captured = Vec::new();
        if row_diff_abs == 2 {
            let mid_row = (from_row + to_row) / 2;
            let mid_col = (from_col + to_col) / 2;
            captured.push((mid_row, mid_col));
            self.board.set_piece(mid_row, mid_col, None);
        }

        self.board
            .move_piece((from_row, from_col), (to_row, to_col));

        let mut became_king = false;
        if game_logic::should_promote(&piece, to_row, self.board.size) {
            if let Some(mut promoted_piece) = self.board.get_piece(to_row, to_col) {
                promoted_piece.promote_to_king();
                self.board.set_piece(to_row, to_col, Some(promoted_piece));
                became_king = true;
            }
        }

        // Record the move in history
        self.move_history.add_move(
            (from_row, from_col),
            (to_row, to_col),
            self.current_player,
            captured,
            became_king,
        );

        let continue_capture = row_diff_abs == 2 && game_logic::has_more_captures_for_piece(&self.board, to_row, to_col);
        
        if !continue_capture {
            self.switch_player();
        }
        
        Ok(continue_capture)
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
