use crate::core::board::{Board, Grid};
use crate::core::piece::Face;
use crate::core::Position;

/// Converts board coordinates to algebraic square.
pub fn square(row: usize, col: usize) -> String {
    format!("{}{}", (col as u8 + b'A') as char, 8 - row)
}

/// Renders board state into string grid.
pub fn board(state: &Board) -> String {
    let mut board_str = String::new();
    board_str.push_str("  A B C D E F G H\n");
    for r in 0..state.edge() {
        board_str.push_str(&format!("{} ", 8 - r));
        for c in 0..state.edge() {
            let piece_str: String = match state.piece(Position { row: r, col: c }) {
                Some(piece) => piece.symbol(),
                None => ".".to_string(),
            };
            board_str.push_str(&piece_str);
            board_str.push(' ');
        }
        board_str.push('\n');
    }
    board_str
}
