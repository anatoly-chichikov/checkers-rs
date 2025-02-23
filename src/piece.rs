#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    pub color: Color,
    pub is_king: bool,
}

impl Piece {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            is_king: false,
        }
    }

    pub fn promote_to_king(&mut self) {
        self.is_king = true;
    }

    pub fn display(&self) -> char {
        match (self.color, self.is_king) {
            (Color::White, false) => 'w',
            (Color::White, true) => 'W',
            (Color::Black, false) => 'b',
            (Color::Black, true) => 'B',
        }
    }
} 