use crate::core::piece::Color;
use crate::core::{Move, Position};

#[derive(Debug, Clone, PartialEq)]
pub struct Turn {
    pub step: Move,
    pub player: Color,
    pub captures: Vec<Position>,
    pub crown: bool,
}

#[derive(Debug, Clone)]
pub struct MoveHistory {
    turns: Vec<Turn>,
}

pub struct HistorySeed {
    pub turns: Vec<Turn>,
}

pub trait Forge {
    /// Creates a history from the supplied turns
    fn make(self) -> MoveHistory;
}

pub trait History {
    /// Returns a new history with the supplied turn appended
    fn add(&self, step: Move, player: Color, captures: Vec<Position>, crown: bool) -> MoveHistory;
    /// Returns the move history in notation form
    fn notation(&self) -> String;
}

impl History for MoveHistory {
    fn add(&self, step: Move, player: Color, captures: Vec<Position>, crown: bool) -> MoveHistory {
        let mut turns = self.turns.clone();
        turns.push(Turn {
            step,
            player,
            captures,
            crown,
        });
        MoveHistory { turns }
    }
    fn notation(&self) -> String {
        self.turns
            .iter()
            .enumerate()
            .map(|(index, turn)| {
                let origin = format!(
                    "{}{}",
                    (b'a' + turn.step.origin.col as u8) as char,
                    8 - turn.step.origin.row
                );
                let target = format!(
                    "{}{}",
                    (b'a' + turn.step.target.col as u8) as char,
                    8 - turn.step.target.row
                );
                let mark = if turn.captures.is_empty() { "-" } else { "x" };
                let crown = if turn.crown { "K" } else { "" };
                format!("{}. {}{}{}{}", index + 1, origin, mark, target, crown)
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl Forge for HistorySeed {
    fn make(self) -> MoveHistory {
        MoveHistory { turns: self.turns }
    }
}
