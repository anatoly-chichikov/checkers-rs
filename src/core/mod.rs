pub mod board;
pub mod game;
pub mod game_logic;
pub mod move_history;
pub mod piece;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

impl From<(usize, usize)> for Position {
    fn from((row, col): (usize, usize)) -> Self {
        Self { row, col }
    }
}

impl From<Position> for (usize, usize) {
    fn from(pos: Position) -> Self {
        (pos.row, pos.col)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameMove {
    pub from: Position,
    pub to: Position,
}

impl GameMove {
    pub fn from_tuples(from: (usize, usize), to: (usize, usize)) -> Self {
        Self {
            from: Position::from(from),
            to: Position::from(to),
        }
    }
}
