#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

pub trait Side {
    /// Returns the opposing color
    fn opponent(&self) -> Self;
}

impl Side for Color {
    fn opponent(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    pub color: Color,
    pub king: bool,
}

pub trait Face {
    /// Returns the piece symbol for display
    fn symbol(&self) -> String;
}

impl Face for Piece {
    fn symbol(&self) -> String {
        match (self.color, self.king) {
            (Color::White, false) => "(w)".to_string(),
            (Color::White, true) => "[W]".to_string(),
            (Color::Black, false) => "(b)".to_string(),
            (Color::Black, true) => "[B]".to_string(),
        }
    }
}
