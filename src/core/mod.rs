pub mod board;
pub mod game;
pub mod move_history;
pub mod piece;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub origin: Position,
    pub target: Position,
}
