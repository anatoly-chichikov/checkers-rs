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

    pub fn make_move(
        &self,
        from_row: usize,
        from_col: usize,
        to_row: usize,
        to_col: usize,
    ) -> Result<(Self, bool), GameError> {
        let mut new_game = self.clone();

        if !new_game.board.in_bounds(to_row, to_col) {
            return Err(GameError::OutOfBounds);
        }

        let piece = new_game
            .board
            .get_piece(from_row, from_col)
            .ok_or(GameError::NoPieceSelected)?;

        if new_game.has_captures_available() {
            let row_diff = (to_row as i32 - from_row as i32).abs();
            if row_diff != 2 {
                return Err(GameError::ForcedCaptureAvailable);
            }
        }

        if !game_logic::is_valid_move(&new_game.board, from_row, from_col, to_row, to_col, &piece) {
            return Err(GameError::InvalidMove);
        }

        let row_diff_abs = (to_row as i32 - from_row as i32).abs();
        let mut captured = Vec::new();
        if row_diff_abs == 2 {
            let mid_row = (from_row + to_row) / 2;
            let mid_col = (from_col + to_col) / 2;
            captured.push((mid_row, mid_col));
            new_game.board.set_piece(mid_row, mid_col, None);
        }

        new_game
            .board
            .move_piece((from_row, from_col), (to_row, to_col));

        let mut became_king = false;
        if game_logic::should_promote(&piece, to_row, new_game.board.size) {
            if let Some(mut promoted_piece) = new_game.board.get_piece(to_row, to_col) {
                promoted_piece.promote_to_king();
                new_game
                    .board
                    .set_piece(to_row, to_col, Some(promoted_piece));
                became_king = true;
            }
        }

        // Record the move in history
        new_game.move_history.add_move(
            (from_row, from_col),
            (to_row, to_col),
            new_game.current_player,
            captured,
            became_king,
        );

        let continue_capture = row_diff_abs == 2
            && game_logic::has_more_captures_for_piece(&new_game.board, to_row, to_col);

        if !continue_capture {
            new_game.current_player = new_game.current_player.opposite();
        }

        Ok((new_game, continue_capture))
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

    pub fn with_switched_player(&self) -> Self {
        let mut new_game = self.clone();
        new_game.current_player = new_game.current_player.opposite();
        new_game
    }
}
