use crate::core::board::Board;
use crate::core::piece::{Color, Piece};

/// Checks if a piece can be promoted to king based on its position
pub fn should_promote(piece: &Piece, row: usize, board_size: usize) -> bool {
    match piece.color {
        Color::White => row == 0,
        Color::Black => row == board_size - 1,
    }
}

/// Checks if a move is valid according to checkers rules
pub fn is_valid_move(
    board: &Board,
    from_row: usize,
    from_col: usize,
    to_row: usize,
    to_col: usize,
    piece: &Piece,
) -> bool {
    // Basic checks
    if !board.in_bounds(to_row, to_col) {
        return false;
    }

    if board.get_piece(to_row, to_col).is_some() {
        return false;
    }

    let row_diff = to_row as i32 - from_row as i32;
    let col_diff = to_col as i32 - from_col as i32;

    // Check diagonal movement
    if col_diff.abs() != row_diff.abs() {
        return false;
    }

    // Regular move
    if row_diff.abs() == 1 {
        return piece.is_king
            || match piece.color {
                Color::White => row_diff < 0,
                Color::Black => row_diff > 0,
            };
    }

    // Capture move
    if row_diff.abs() == 2 && col_diff.abs() == 2 {
        let mid_row = ((from_row as i32 + to_row as i32) / 2) as usize;
        let mid_col = ((from_col as i32 + to_col as i32) / 2) as usize;

        // Check if there's an opponent's piece to capture
        if let Some(captured_piece) = board.get_piece(mid_row, mid_col) {
            if captured_piece.color == piece.color {
                return false;
            }

            return piece.is_king
                || match piece.color {
                    Color::White => row_diff < 0,
                    Color::Black => row_diff > 0,
                };
        }
    }

    false
}

/// Checks if a piece has more captures available
pub fn has_more_captures_for_piece(board: &Board, row: usize, col: usize) -> bool {
    if let Some(piece) = board.get_piece(row, col) {
        let directions = if piece.is_king {
            vec![(2, 2), (2, -2), (-2, 2), (-2, -2)]
        } else {
            match piece.color {
                Color::White => vec![(-2, 2), (-2, -2)],
                Color::Black => vec![(2, 2), (2, -2)],
            }
        };

        for (row_diff, col_diff) in directions {
            let next_row = match (row as i32 + row_diff).try_into() {
                Ok(val) => val,
                Err(_) => continue,
            };
            let next_col = match (col as i32 + col_diff).try_into() {
                Ok(val) => val,
                Err(_) => continue,
            };

            if board.in_bounds(next_row, next_col)
                && is_valid_move(board, row, col, next_row, next_col, &piece)
            {
                return true;
            }
        }
    }
    false
}

/// Checks if any piece of the given color has captures available
pub fn has_captures_available(board: &Board, current_player: Color) -> bool {
    for row in 0..board.size {
        for col in 0..board.size {
            if let Some(piece) = board.get_piece(row, col) {
                if piece.color == current_player {
                    let directions = if piece.is_king {
                        vec![(2, 2), (2, -2), (-2, 2), (-2, -2)]
                    } else {
                        match piece.color {
                            Color::White => vec![(-2, 2), (-2, -2)],
                            Color::Black => vec![(2, 2), (2, -2)],
                        }
                    };

                    for (row_diff, col_diff) in directions {
                        let to_row = match (row as i32 + row_diff).try_into() {
                            Ok(val) => val,
                            Err(_) => continue,
                        };
                        let to_col = match (col as i32 + col_diff).try_into() {
                            Ok(val) => val,
                            Err(_) => continue,
                        };

                        if board.in_bounds(to_row, to_col)
                            && is_valid_move(board, row, col, to_row, to_col, &piece)
                        {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

/// Checks if the current player is in a stalemate (no valid moves)
pub fn is_stalemate(board: &Board, current_player: Color) -> bool {
    // First check if there are any captures available
    if has_captures_available(board, current_player) {
        return false;
    }

    // If no captures are available, check for regular moves
    for row in 0..board.size {
        for col in 0..board.size {
            if let Some(piece) = board.get_piece(row, col) {
                if piece.color == current_player {
                    let directions = if piece.is_king {
                        vec![(1, 1), (1, -1), (-1, 1), (-1, -1)]
                    } else {
                        match piece.color {
                            Color::White => vec![(-1, 1), (-1, -1)],
                            Color::Black => vec![(1, 1), (1, -1)],
                        }
                    };

                    for (row_diff, col_diff) in directions {
                        let to_row = match (row as i32 + row_diff).try_into() {
                            Ok(val) => val,
                            Err(_) => continue,
                        };
                        let to_col = match (col as i32 + col_diff).try_into() {
                            Ok(val) => val,
                            Err(_) => continue,
                        };

                        if board.in_bounds(to_row, to_col)
                            && is_valid_move(board, row, col, to_row, to_col, &piece)
                        {
                            return false;
                        }
                    }
                }
            }
        }
    }
    true
}

/// Checks if a player has won the game
pub fn check_winner(board: &Board) -> Option<Color> {
    let mut white_pieces = false;
    let mut black_pieces = false;

    for row in 0..board.size {
        for col in 0..board.size {
            if let Some(piece) = board.get_piece(row, col) {
                match piece.color {
                    Color::White => white_pieces = true,
                    Color::Black => black_pieces = true,
                }
                if white_pieces && black_pieces {
                    return None;
                }
            }
        }
    }

    match (white_pieces, black_pieces) {
        (true, false) => Some(Color::White),
        (false, true) => Some(Color::Black),
        _ => None,
    }
}
