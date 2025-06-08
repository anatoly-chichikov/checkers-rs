use crate::core::piece::Color as PieceColor;

#[derive(Debug, Clone, PartialEq)]
pub struct Move {
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub player: PieceColor,
    pub captured: Vec<(usize, usize)>,
    pub became_king: bool,
}

#[derive(Debug, Clone)]
pub struct MoveHistory {
    moves: Vec<Move>,
}

impl MoveHistory {
    pub fn new() -> Self {
        MoveHistory { moves: Vec::new() }
    }

    pub fn add_move(
        &mut self,
        from: (usize, usize),
        to: (usize, usize),
        player: PieceColor,
        captured: Vec<(usize, usize)>,
        became_king: bool,
    ) {
        self.moves.push(Move {
            from,
            to,
            player,
            captured,
            became_king,
        });
    }

    pub fn to_notation(&self) -> String {
        self.moves
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let from = format!("{}{}", (b'a' + m.from.1 as u8) as char, 8 - m.from.0);
                let to = format!("{}{}", (b'a' + m.to.1 as u8) as char, 8 - m.to.0);
                let capture = if m.captured.is_empty() { "-" } else { "x" };
                let king = if m.became_king { "K" } else { "" };
                format!("{}. {}{}{}{}", i + 1, from, capture, to, king)
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl Default for MoveHistory {
    fn default() -> Self {
        Self::new()
    }
}
