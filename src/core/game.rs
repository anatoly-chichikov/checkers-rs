use crate::core::board::{Board, Grid};
use crate::core::move_history::{Forge, History, HistorySeed, MoveHistory};
use crate::core::piece::{Color, Piece, Side};
use crate::core::{Move, Position};
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
    board: Board,
    turn: Color,
    end: bool,
    history: MoveHistory,
}

pub struct GameSeed {
    pub board: Board,
    pub turn: Color,
}

pub trait Seed {
    /// Creates a game from the supplied seed values
    fn make(self) -> CheckersGame;
}

pub trait Game {
    /// Returns the current board
    fn board(&self) -> &Board;
    /// Returns the active player color
    fn turn(&self) -> Color;
    /// Returns true when the game has ended
    fn end(&self) -> bool;
    /// Returns the move history
    fn history(&self) -> &MoveHistory;
    /// Validates whether a piece can be selected
    fn selection(&self, spot: Position) -> Result<(), GameError>;
    /// Executes a move and returns the updated game and capture status
    fn play(&self, step: Move) -> Result<(CheckersGame, bool), GameError>;
    /// Returns true when the active player has captures available
    fn captures(&self) -> bool;
    /// Returns the winning color when the game is decided
    fn winner(&self) -> Option<Color>;
    /// Returns true when the active player is stalemated
    fn stalemate(&self) -> bool;
    /// Switches the active player
    fn switch(&self) -> CheckersGame;
    /// Marks the game as finished
    fn finish(&self) -> CheckersGame;
}

impl Seed for GameSeed {
    fn make(self) -> CheckersGame {
        let history = HistorySeed { turns: Vec::new() }.make();
        CheckersGame {
            board: self.board,
            turn: self.turn,
            end: false,
            history,
        }
    }
}

impl Game for CheckersGame {
    fn board(&self) -> &Board {
        &self.board
    }
    fn turn(&self) -> Color {
        self.turn
    }
    fn end(&self) -> bool {
        self.end
    }
    fn history(&self) -> &MoveHistory {
        &self.history
    }
    fn selection(&self, spot: Position) -> Result<(), GameError> {
        if !self.board.bounds(spot) {
            return Err(GameError::OutOfBounds);
        }
        match self.board.piece(spot) {
            Some(piece) if piece.color == self.turn => {
                if self.captures() && !self.board.capture(spot) {
                    return Err(GameError::ForcedCaptureAvailable);
                }
                Ok(())
            }
            Some(_) => Err(GameError::WrongPieceColor),
            None => Err(GameError::NoPieceSelected),
        }
    }
    fn play(&self, step: Move) -> Result<(CheckersGame, bool), GameError> {
        let mut game = self.clone();
        if !game.board.bounds(step.target) {
            return Err(GameError::OutOfBounds);
        }
        let piece = game
            .board
            .piece(step.origin)
            .ok_or(GameError::NoPieceSelected)?;
        if game.captures() {
            let span = (step.target.row as i32 - step.origin.row as i32).abs();
            if span != 2 {
                return Err(GameError::ForcedCaptureAvailable);
            }
        }
        if !game.board.validity(step.origin, step.target) {
            return Err(GameError::InvalidMove);
        }
        let span = (step.target.row as i32 - step.origin.row as i32).abs();
        let mut captures = Vec::new();
        if span == 2 {
            let mid = Position {
                row: (step.origin.row + step.target.row) / 2,
                col: (step.origin.col + step.target.col) / 2,
            };
            captures.push(mid);
            game.board.place(mid, None);
        }
        game.board.shift(step.origin, step.target);
        let mut crown = false;
        if !piece.king {
            let edge = game.board.edge();
            let permit = match piece.color {
                Color::White => step.target.row == 0,
                Color::Black => step.target.row == edge - 1,
            };
            if permit {
                if let Some(piece) = game.board.piece(step.target) {
                    let king = Piece {
                        color: piece.color,
                        king: true,
                    };
                    game.board.place(step.target, Some(king));
                    crown = true;
                }
            }
        }
        game.history = game.history.add(step, game.turn, captures, crown);
        let chain = span == 2 && game.board.chain(step.target);
        if !chain {
            game.turn = game.turn.opponent();
        }
        Ok((game, chain))
    }
    fn captures(&self) -> bool {
        self.board.captures(self.turn)
    }
    fn winner(&self) -> Option<Color> {
        self.board.winner()
    }
    fn stalemate(&self) -> bool {
        self.board.stalemate(self.turn)
    }
    fn switch(&self) -> CheckersGame {
        let mut game = self.clone();
        game.turn = game.turn.opponent();
        game
    }
    fn finish(&self) -> CheckersGame {
        let mut game = self.clone();
        game.end = true;
        game
    }
}
