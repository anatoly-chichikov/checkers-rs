use crate::core::board::Board;

pub fn format_square(row: usize, col: usize) -> String {
    // Convert internal row (0=top, 7=bottom) to display row (8=top, 1=bottom)
    format!("{}{}", (col as u8 + b'A') as char, 8 - row)
}

pub fn format_board(board: &Board) -> String {
    let mut board_str = String::new();
    board_str.push_str("  A B C D E F G H\n");
    for r in 0..board.size {
        // Convert internal row (0=top, 7=bottom) to display row (8=top, 1=bottom)
        board_str.push_str(&format!("{} ", 8 - r));
        for c in 0..board.size {
            let piece_str: String = match board.get_piece(r, c) {
                Some(piece) => piece.display(),
                None => ".".to_string(),
            };
            board_str.push_str(&piece_str);
            board_str.push(' ');
        }
        board_str.push('\n');
    }
    board_str
}
