use crate::core::board::Board;
use crate::core::piece::{Color, Piece};

// Checks if a piece can be promoted to king based on its position
pub fn should_promote(piece: &Piece, row: usize, board_size: usize) -> bool {
    match piece.color {
        Color::White => row == 0,
        Color::Black => row == board_size - 1,
    }
}

// Helper function to find all capture sequences for a piece
fn find_capture_moves_recursive(
    board: &Board,
    current_row: usize,
    current_col: usize,
    piece: &Piece, // Pass the original piece to maintain its properties (king status, color)
    current_path: Vec<(usize, usize)>, // Stores the sequence of positions in a multi-jump
    all_capture_paths: &mut Vec<Vec<(usize, usize)>>, // Stores all found capture paths (sequences of positions)
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

        // Bounds check for target square
        if to_row_i32 < 0 || to_row_i32 >= board.size as i32 || 
           to_col_i32 < 0 || to_col_i32 >= board.size as i32 {
            continue;
        }
        let to_row = to_row_i32 as usize;
        let to_col = to_col_i32 as usize;

        // Calculate midpoint for capture
        let mid_row = ((current_row as i32 + to_row_i32) / 2) as usize;
        let mid_col = ((current_col as i32 + to_col_i32) / 2) as usize;

        // Check if the move is a valid capture
        if board.get_piece(to_row, to_col).is_none() { // Target square must be empty
            if let Some(mid_piece) = board.get_piece(mid_row, mid_col) {
                if mid_piece.color != piece.color { // Must capture an opponent's piece
                    // Check if this jump is valid based on piece type and direction
                    let is_capture_valid_for_piece = if piece.is_king {
                        true // Kings can capture in any diagonal direction
                    } else {
                        // Regular pieces have direction-specific captures
                        match piece.color {
                            Color::White => to_row_i32 < current_row as i32, // White captures "up"
                            Color::Black => to_row_i32 > current_row as i32, // Black captures "down"
                        }
                    };

                    if is_capture_valid_for_piece {
                        let mut new_board = board.clone(); // Create a new board state for this path
                        new_board.set_piece(mid_row, mid_col, None); // Remove the captured piece
                        // Temporarily move the piece on the new board to explore further jumps
                        // The original piece object (with its king status) is carried through.
                        // We don't actually place it on new_board for this check, as we only need its properties.

                        let mut next_path = current_path.clone();
                        next_path.push((to_row, to_col));
                        found_next_capture = true;

                        // Recursively check for more captures from the new position
                        find_capture_moves_recursive(&new_board, to_row, to_col, piece, next_path, all_capture_paths);
                    }
                }
            }
        }
    }

    // If no further captures were found from this position, this path ends.
    // If the current_path is not empty (i.e., it's a result of at least one jump), add it.
    if !found_next_capture && !current_path.is_empty() {
        all_capture_paths.push(current_path);
    }
}


pub fn get_all_possible_moves(
    board: &Board,
    piece_row: usize,
    piece_col: usize,
) -> Vec<(usize, usize)> {
    let piece = match board.get_piece(piece_row, piece_col) {
        Some(p) => p.clone(), // Clone the piece to avoid ownership issues
        None => return vec![], // No piece at the given position
    };

    let mut all_capture_final_positions: Vec<(usize, usize)> = Vec::new();
    let mut capture_paths: Vec<Vec<(usize, usize)>> = Vec::new();

    // Start recursive search for captures.
    // The initial "path" for the recursion is just the starting point, but find_capture_moves_recursive
    // expects paths of actual jumps. So, we initiate it with an empty path, and it will add
    // sequences of jumps.
    find_capture_moves_recursive(board, piece_row, piece_col, &piece, Vec::new(), &mut capture_paths);

    if !capture_paths.is_empty() {
        // If there are capture paths, return the final landing spots of these paths.
        for path in capture_paths {
            if let Some(last_pos) = path.last() {
                if !all_capture_final_positions.contains(last_pos) {
                     all_capture_final_positions.push(*last_pos);
                }
            }
        }
        return all_capture_final_positions;
    }

    // If no captures are available, find regular moves
    let mut regular_moves: Vec<(usize, usize)> = Vec::new();
    let move_offsets = if piece.is_king {
        // Kings can move in all four diagonal directions
        vec![(-1, -1), (-1, 1), (1, -1), (1, 1)]
    } else {
        // Non-king pieces have specific move directions based on color
        match piece.color {
            Color::White => vec![(-1, -1), (-1, 1)], // White moves "up"
            Color::Black => vec![(1, -1), (1, 1)],   // Black moves "down"
        }
    };

    for (row_offset, col_offset) in move_offsets {
        let to_row_i32 = piece_row as i32 + row_offset;
        let to_col_i32 = piece_col as i32 + col_offset;

        if to_row_i32 < 0 || to_row_i32 >= board.size as i32 || 
           to_col_i32 < 0 || to_col_i32 >= board.size as i32 {
            continue; // Target square is out of bounds
        }
        let to_row = to_row_i32 as usize;
        let to_col = to_col_i32 as usize;

        if is_valid_move(board, piece_row, piece_col, to_row, to_col, &piece) {
            regular_moves.push((to_row, to_col));
        }
    }

    regular_moves
}

// Checks if a move is valid according to checkers rules
pub fn is_valid_move(
    board: &Board,
    from_row: usize,
    from_col: usize,
    to_row: usize,
    to_col: usize,
    piece: &Piece,
) -> bool {
    // Basic checks for bounds and target empty
    if !board.in_bounds(to_row, to_col) { return false; }
    if board.get_piece(to_row, to_col).is_some() { return false; }

    let row_diff = to_row as i32 - from_row as i32;
    let col_diff = to_col as i32 - from_col as i32;

    // Must be diagonal
    if col_diff.abs() != row_diff.abs() { return false; }

    // KING MOVEMENT LOGIC
    if piece.is_king {
        // King capture (2 steps)
        if row_diff.abs() == 2 { // col_diff.abs() will also be 2 due to prior diagonal check
            let mid_row = ((from_row as i32 + to_row as i32) / 2) as usize;
            let mid_col = ((from_col as i32 + to_col as i32) / 2) as usize;
            // Check for opponent piece in middle for capture
            if let Some(mid_piece) = board.get_piece(mid_row, mid_col) {
                if mid_piece.color != piece.color { // It's an opponent's piece
                    return true; // Valid king capture
                }
            }
            return false; // Invalid king capture (middle not opponent, or empty, or own piece)
        }
        // King regular move (1 step)
        if row_diff.abs() == 1 { // col_diff.abs() will also be 1
            return true; // Valid king regular move (any diagonal direction)
        }
        // Not a 1-step or 2-step diagonal move for a king
        return false;
    } else {
        // REGULAR PIECE LOGIC
        // Regular piece regular move (1 step)
        if row_diff.abs() == 1 { // col_diff.abs() will also be 1
            // Check direction based on color
            return match piece.color {
                Color::White => row_diff < 0, // White moves "up" (row index decreases)
                Color::Black => row_diff > 0, // Black moves "down" (row index increases)
            };
        }
        // Regular piece capture (2 steps)
        if row_diff.abs() == 2 { // col_diff.abs() will also be 2
            let mid_row = ((from_row as i32 + to_row as i32) / 2) as usize;
            let mid_col = ((from_col as i32 + to_col as i32) / 2) as usize;
            // Check for opponent piece in middle for capture
            if let Some(mid_piece) = board.get_piece(mid_row, mid_col) {
                if mid_piece.color != piece.color { // It's an opponent's piece
                    // Check direction based on color for capture
                    return match piece.color {
                        Color::White => row_diff < 0, // White captures "up"
                        Color::Black => row_diff > 0, // Black captures "down"
                    };
                }
            }
            return false; // Invalid regular piece capture (middle not opponent, or empty, or own piece)
        }
    }
    // If move is not 1-step or 2-step diagonal (should be caught by diagonal check or above logic)
    false
}

/// Checks if a piece can make a capture
pub fn can_piece_capture(board: &Board, piece_row: usize, piece_col: usize) -> bool {
    if let Some(piece) = board.get_piece(piece_row, piece_col) {
        let directions = if piece.is_king {
            // Kings can capture in all four diagonal directions
            vec![(-2, -2), (-2, 2), (2, -2), (2, 2)]
        } else {
            // Non-king pieces have specific capture directions based on color
            match piece.color {
                Color::White => vec![(-2, -2), (-2, 2)], // White captures "up"
                Color::Black => vec![(2, -2), (2, 2)],   // Black captures "down"
            }
        };

        for (row_offset, col_offset) in directions {
            // Ensure `piece_row + row_offset` and `piece_col + col_offset` don't underflow/overflow
            let to_row_i32 = piece_row as i32 + row_offset;
            let to_col_i32 = piece_col as i32 + col_offset;

            if to_row_i32 < 0 || to_row_i32 >= board.size as i32 || 
               to_col_i32 < 0 || to_col_i32 >= board.size as i32 {
                continue; // Target square is out of bounds
            }
            let to_row = to_row_i32 as usize;
            let to_col = to_col_i32 as usize;
            
            // Middle square calculation
            // These calculations are safe because row_offset/col_offset is always +/-2
            let mid_row = (piece_row as i32 + to_row_i32) / 2;
            let mid_col = (piece_col as i32 + to_col_i32) / 2;
            
            // Check if middle square is within bounds (it should be if to_row/to_col is)
            if mid_row < 0 || mid_row >= board.size as i32 || 
               mid_col < 0 || mid_col >= board.size as i32 {
                continue; 
            }
            let mid_row_usize = mid_row as usize;
            let mid_col_usize = mid_col as usize;


            // Check if target square is empty and middle square has an opponent's piece
            if board.get_piece(to_row, to_col).is_none() {
                if let Some(mid_piece) = board.get_piece(mid_row_usize, mid_col_usize) {
                    if mid_piece.color != piece.color {
                        // Directionality for non-kings is implicitly handled by the `directions` vector.
                        // For kings, all directions in the vector are valid.
                        // `is_valid_move` logic for captures is essentially replicated here.
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
