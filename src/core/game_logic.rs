use crate::core::board::Board;
use crate::core::piece::{Color, Piece};

/// Type alias for a move with its origin position, destination position, and whether it's a capture
pub type Move = ((usize, usize), (usize, usize), bool);

pub fn should_promote(piece: &Piece, row: usize, board_size: usize) -> bool {
    match piece.color {
        Color::White => row == 0,
        Color::Black => row == board_size - 1,
    }
}

pub fn get_all_valid_moves_for_player(board: &Board, player_color: Color) -> Vec<Move> {
    let mut capture_moves: Vec<Move> = Vec::new();
    let mut regular_moves: Vec<Move> = Vec::new();

    for r in 0..board.size {
        for c in 0..board.size {
            if let Some(piece) = board.get_piece(r, c) {
                if piece.color == player_color {
                    let possible_destinations = get_all_possible_moves(board, r, c);
                    for (to_r, to_c) in possible_destinations {
                        let is_capture = (to_r as i32 - r as i32).abs() == 2;
                        let is_col_capture_distance = (to_c as i32 - c as i32).abs() == 2;

                        if is_capture && is_col_capture_distance {
                            capture_moves.push(((r, c), (to_r, to_c), true));
                        } else if (to_r as i32 - r as i32).abs() == 1
                            && (to_c as i32 - c as i32).abs() == 1
                        {
                            regular_moves.push(((r, c), (to_r, to_c), false));
                        }
                    }
                }
            }
        }
    }

    if !capture_moves.is_empty() {
        capture_moves
    } else {
        regular_moves
    }
}

fn find_capture_moves_recursive(
    board: &Board,
    current_row: usize,
    current_col: usize,
    piece: &Piece,
    current_path: Vec<(usize, usize)>,
    all_capture_paths: &mut Vec<Vec<(usize, usize)>>,
) {
    let mut found_next_capture = false;

    let directions = if piece.is_king {
        vec![(-2, -2), (-2, 2), (2, -2), (2, 2)]
    } else {
        match piece.color {
            Color::White => vec![(-2, -2), (-2, 2)],
            Color::Black => vec![(2, -2), (2, 2)],
        }
    };

    for (row_offset, col_offset) in directions {
        let to_row_i32 = current_row as i32 + row_offset;
        let to_col_i32 = current_col as i32 + col_offset;

        if to_row_i32 < 0
            || to_row_i32 >= board.size as i32
            || to_col_i32 < 0
            || to_col_i32 >= board.size as i32
        {
            continue;
        }
        let to_row = to_row_i32 as usize;
        let to_col = to_col_i32 as usize;

        let mid_row = ((current_row as i32 + to_row_i32) / 2) as usize;
        let mid_col = ((current_col as i32 + to_col_i32) / 2) as usize;

        if board.get_piece(to_row, to_col).is_none() {
            if let Some(mid_piece) = board.get_piece(mid_row, mid_col) {
                if mid_piece.color != piece.color {
                    let is_capture_valid_for_piece = if piece.is_king {
                        true
                    } else {
                        match piece.color {
                            Color::White => to_row_i32 < current_row as i32,
                            Color::Black => to_row_i32 > current_row as i32,
                        }
                    };

                    if is_capture_valid_for_piece {
                        let mut new_board = board.clone();
                        new_board.set_piece(mid_row, mid_col, None);

                        let mut next_path = current_path.clone();
                        next_path.push((to_row, to_col));
                        found_next_capture = true;

                        find_capture_moves_recursive(
                            &new_board,
                            to_row,
                            to_col,
                            piece,
                            next_path,
                            all_capture_paths,
                        );
                    }
                }
            }
        }
    }

    if !found_next_capture && !current_path.is_empty() {
        all_capture_paths.push(current_path);
    }
}

pub fn find_capture_path(
    board: &Board,
    from_row: usize,
    from_col: usize,
    to_row: usize,
    to_col: usize,
) -> Option<Vec<(usize, usize)>> {
    let piece = board.get_piece(from_row, from_col)?;

    let mut all_paths = Vec::new();
    find_capture_moves_recursive(
        board,
        from_row,
        from_col,
        &piece,
        Vec::new(),
        &mut all_paths,
    );

    // Find a path that ends at the target position
    for path in all_paths {
        if let Some(&last_pos) = path.last() {
            if last_pos == (to_row, to_col) {
                return Some(path);
            }
        }
    }

    None
}

pub fn get_all_possible_moves(
    board: &Board,
    piece_row: usize,
    piece_col: usize,
) -> Vec<(usize, usize)> {
    let piece = match board.get_piece(piece_row, piece_col) {
        Some(p) => p,
        None => return vec![],
    };

    let mut all_capture_final_positions: Vec<(usize, usize)> = Vec::new();
    let mut capture_paths: Vec<Vec<(usize, usize)>> = Vec::new();

    find_capture_moves_recursive(
        board,
        piece_row,
        piece_col,
        &piece,
        Vec::new(),
        &mut capture_paths,
    );

    if !capture_paths.is_empty() {
        for path in capture_paths {
            if let Some(last_pos) = path.last() {
                if !all_capture_final_positions.contains(last_pos) {
                    all_capture_final_positions.push(*last_pos);
                }
            }
        }
        return all_capture_final_positions;
    }

    let mut regular_moves: Vec<(usize, usize)> = Vec::new();
    let move_offsets = if piece.is_king {
        vec![(-1, -1), (-1, 1), (1, -1), (1, 1)]
    } else {
        match piece.color {
            Color::White => vec![(-1, -1), (-1, 1)],
            Color::Black => vec![(1, -1), (1, 1)],
        }
    };

    for (row_offset, col_offset) in move_offsets {
        let to_row_i32 = piece_row as i32 + row_offset;
        let to_col_i32 = piece_col as i32 + col_offset;

        if to_row_i32 < 0
            || to_row_i32 >= board.size as i32
            || to_col_i32 < 0
            || to_col_i32 >= board.size as i32
        {
            continue;
        }
        let to_row = to_row_i32 as usize;
        let to_col = to_col_i32 as usize;

        if is_valid_move(board, piece_row, piece_col, to_row, to_col, &piece) {
            regular_moves.push((to_row, to_col));
        }
    }

    regular_moves
}

pub fn is_valid_move(
    board: &Board,
    from_row: usize,
    from_col: usize,
    to_row: usize,
    to_col: usize,
    piece: &Piece,
) -> bool {
    if !board.in_bounds(to_row, to_col) {
        return false;
    }
    if board.get_piece(to_row, to_col).is_some() {
        return false;
    }

    let row_diff = to_row as i32 - from_row as i32;
    let col_diff = to_col as i32 - from_col as i32;

    if col_diff.abs() != row_diff.abs() {
        return false;
    }

    if piece.is_king {
        if row_diff.abs() == 2 {
            let mid_row = ((from_row as i32 + to_row as i32) / 2) as usize;
            let mid_col = ((from_col as i32 + to_col as i32) / 2) as usize;
            if let Some(mid_piece) = board.get_piece(mid_row, mid_col) {
                if mid_piece.color != piece.color {
                    return true;
                }
            }
            return false;
        }
        if row_diff.abs() == 1 {
            return true;
        }
        return false;
    } else {
        if row_diff.abs() == 1 {
            return match piece.color {
                Color::White => row_diff < 0,
                Color::Black => row_diff > 0,
            };
        }
        if row_diff.abs() == 2 {
            let mid_row = ((from_row as i32 + to_row as i32) / 2) as usize;
            let mid_col = ((from_col as i32 + to_col as i32) / 2) as usize;
            if let Some(mid_piece) = board.get_piece(mid_row, mid_col) {
                if mid_piece.color != piece.color {
                    return match piece.color {
                        Color::White => row_diff < 0,
                        Color::Black => row_diff > 0,
                    };
                }
            }
            return false;
        }
    }
    false
}

/// Checks if a piece can make a capture
#[allow(dead_code)]
pub fn can_piece_capture(board: &Board, piece_row: usize, piece_col: usize) -> bool {
    if let Some(piece) = board.get_piece(piece_row, piece_col) {
        let directions = if piece.is_king {
            vec![(-2, -2), (-2, 2), (2, -2), (2, 2)]
        } else {
            match piece.color {
                Color::White => vec![(-2, -2), (-2, 2)],
                Color::Black => vec![(2, -2), (2, 2)],
            }
        };

        for (row_offset, col_offset) in directions {
            let to_row_i32 = piece_row as i32 + row_offset;
            let to_col_i32 = piece_col as i32 + col_offset;

            if to_row_i32 < 0
                || to_row_i32 >= board.size as i32
                || to_col_i32 < 0
                || to_col_i32 >= board.size as i32
            {
                continue;
            }
            let to_row = to_row_i32 as usize;
            let to_col = to_col_i32 as usize;

            let mid_row = (piece_row as i32 + to_row_i32) / 2;
            let mid_col = (piece_col as i32 + to_col_i32) / 2;

            if mid_row < 0
                || mid_row >= board.size as i32
                || mid_col < 0
                || mid_col >= board.size as i32
            {
                continue;
            }
            let mid_row_usize = mid_row as usize;
            let mid_col_usize = mid_col as usize;

            if board.get_piece(to_row, to_col).is_none() {
                if let Some(mid_piece) = board.get_piece(mid_row_usize, mid_col_usize) {
                    if mid_piece.color != piece.color {
                        return true;
                    }
                }
            }
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
    if has_captures_available(board, current_player) {
        return false;
    }

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

/// Returns all pieces of the given color that can capture
pub fn get_pieces_with_captures(board: &Board, color: Color) -> Vec<(usize, usize)> {
    let mut pieces_with_captures = Vec::new();

    for row in 0..board.size {
        for col in 0..board.size {
            if let Some(piece) = board.get_piece(row, col) {
                if piece.color == color && can_piece_capture(board, row, col) {
                    pieces_with_captures.push((row, col));
                }
            }
        }
    }

    pieces_with_captures
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
