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

    pub fn display(&self) -> String {
        match (self.color, self.is_king) {
            (Color::White, false) => "(w)".to_string(),
            (Color::White, true) => "[W]".to_string(),
            (Color::Black, false) => "(b)".to_string(),
            (Color::Black, true) => "[B]".to_string(),
        }
    }
}
