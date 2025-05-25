use checkers_rs::core::board::Board;
use checkers_rs::core::game_logic;
use checkers_rs::core::piece::{Color, Piece};

#[test]
fn test_should_promote() {
    let white_piece = Piece::new(Color::White);
    let black_piece = Piece::new(Color::Black);
    let board_size = 8;

    // White pieces should promote at row 0
    assert!(game_logic::should_promote(&white_piece, 0, board_size));
    assert!(!game_logic::should_promote(&white_piece, 1, board_size));

    // Black pieces should promote at row 7
    assert!(game_logic::should_promote(&black_piece, 7, board_size));
    assert!(!game_logic::should_promote(&black_piece, 6, board_size));
}

#[test]
fn test_is_valid_move() {
    let mut board = Board::new(8);
    let white_piece = Piece::new(Color::White);
    let black_piece = Piece::new(Color::Black);

    // Clear the board
    for row in 0..board.size {
        for col in 0..board.size {
            board.set_piece(row, col, None);
        }
    }

    // Place a white piece
    board.set_piece(5, 2, Some(white_piece));

    // Valid diagonal move forward
    assert!(game_logic::is_valid_move(&board, 5, 2, 4, 3, &white_piece));
    assert!(game_logic::is_valid_move(&board, 5, 2, 4, 1, &white_piece));

    // Invalid backward move for regular piece
    assert!(!game_logic::is_valid_move(&board, 5, 2, 6, 3, &white_piece));
    assert!(!game_logic::is_valid_move(&board, 5, 2, 6, 1, &white_piece));

    // Place a black piece
    board.set_piece(2, 3, Some(black_piece));

    // Valid diagonal move forward for black
    assert!(game_logic::is_valid_move(&board, 2, 3, 3, 4, &black_piece));
    assert!(game_logic::is_valid_move(&board, 2, 3, 3, 2, &black_piece));

    // Invalid backward move for regular black piece
    assert!(!game_logic::is_valid_move(&board, 2, 3, 1, 4, &black_piece));
    assert!(!game_logic::is_valid_move(&board, 2, 3, 1, 2, &black_piece));
}

#[test]
fn test_capture_move() {
    let mut board = Board::new(8);
    let white_piece = Piece::new(Color::White);
    let black_piece = Piece::new(Color::Black);

    // Clear the board
    for row in 0..board.size {
        for col in 0..board.size {
            board.set_piece(row, col, None);
        }
    }

    // Setup a capture scenario for white
    board.set_piece(5, 2, Some(white_piece));
    board.set_piece(4, 3, Some(black_piece));
    board.set_piece(3, 4, None); // Make sure the destination is empty

    // Valid capture move
    assert!(game_logic::is_valid_move(&board, 5, 2, 3, 4, &white_piece));

    // Setup a capture scenario for black
    board.set_piece(2, 5, Some(black_piece));
    board.set_piece(3, 4, Some(white_piece));
    board.set_piece(4, 3, None); // Make sure the destination is empty

    // Valid capture move for black
    assert!(game_logic::is_valid_move(&board, 2, 5, 4, 3, &black_piece));
}

#[test]
fn test_king_movement() {
    let mut board = Board::new(8);
    let mut white_king = Piece::new(Color::White);
    white_king.promote_to_king();

    // Clear the board
    for row in 0..board.size {
        for col in 0..board.size {
            board.set_piece(row, col, None);
        }
    }

    // Place a white king
    board.set_piece(4, 4, Some(white_king));

    // Kings can move in any diagonal direction
    assert!(game_logic::is_valid_move(&board, 4, 4, 3, 3, &white_king));
    assert!(game_logic::is_valid_move(&board, 4, 4, 3, 5, &white_king));
    assert!(game_logic::is_valid_move(&board, 4, 4, 5, 3, &white_king));
    assert!(game_logic::is_valid_move(&board, 4, 4, 5, 5, &white_king));
}

#[test]
fn test_has_more_captures() {
    let mut board = Board::new(8);
    let white_piece = Piece::new(Color::White);
    let black_piece = Piece::new(Color::Black);

    // Clear the board
    for row in 0..board.size {
        for col in 0..board.size {
            board.set_piece(row, col, None);
        }
    }

    // Setup a scenario with multiple captures
    board.set_piece(4, 3, Some(white_piece)); // Place the white piece at the position after first capture
    board.set_piece(3, 2, Some(black_piece)); // Place a black piece that can be captured

    // There should be a capture available
    assert!(game_logic::has_more_captures_for_piece(&board, 4, 3));
}

#[test]
fn test_has_captures_available() {
    let mut board = Board::new(8);
    let white_piece = Piece::new(Color::White);
    let black_piece = Piece::new(Color::Black);

    // Clear the board
    for row in 0..board.size {
        for col in 0..board.size {
            board.set_piece(row, col, None);
        }
    }

    // Setup a capture scenario
    board.set_piece(5, 2, Some(white_piece));
    board.set_piece(4, 3, Some(black_piece));
    board.set_piece(3, 4, None); // Make sure the destination is empty

    // White should have a capture available
    assert!(game_logic::has_captures_available(&board, Color::White));
    
    // Black should not have a capture available in this scenario
    // Let's make sure there are no black pieces that can capture
    for row in 0..board.size {
        for col in 0..board.size {
            if let Some(piece) = board.get_piece(row, col) {
                if piece.color == Color::Black {
                    // Make sure this black piece can't capture
                    let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
                    for (dr, dc) in directions {
                        let r = (row as i32 + dr) as usize;
                        let c = (col as i32 + dc) as usize;
                        if board.in_bounds(r, c) {
                            if let Some(p) = board.get_piece(r, c) {
                                if p.color == Color::White {
                                    // If there's a white piece, make sure the landing spot is occupied
                                    let r2 = (r as i32 + dr) as usize;
                                    let c2 = (c as i32 + dc) as usize;
                                    if board.in_bounds(r2, c2) {
                                        board.set_piece(r2, c2, Some(white_piece));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    assert!(!game_logic::has_captures_available(&board, Color::Black));
}

#[test]
fn test_is_stalemate() {
    let mut board = Board::new(8);
    
    // Clear the board
    for row in 0..board.size {
        for col in 0..board.size {
            board.set_piece(row, col, None);
        }
    }

    // Setup a stalemate scenario for white
    let black_piece = Piece::new(Color::Black);
    board.set_piece(0, 1, Some(black_piece));
    
    let white_piece = Piece::new(Color::White);
    board.set_piece(1, 0, Some(white_piece));
    
    // Block all possible moves for white
    board.set_piece(0, 1, Some(black_piece));
    
    // White should be in stalemate
    assert!(game_logic::is_stalemate(&board, Color::White));
    assert!(!game_logic::is_stalemate(&board, Color::Black));
}

#[test]
fn test_check_winner() {
    let mut board = Board::new(8);
    
    // Clear the board
    for row in 0..board.size {
        for col in 0..board.size {
            board.set_piece(row, col, None);
        }
    }

    // No pieces, should be a draw
    assert_eq!(game_logic::check_winner(&board), None);
    
    // Only white pieces
    let white_piece = Piece::new(Color::White);
    board.set_piece(5, 2, Some(white_piece));
    assert_eq!(game_logic::check_winner(&board), Some(Color::White));
    
    // Both colors present
    let black_piece = Piece::new(Color::Black);
    board.set_piece(2, 3, Some(black_piece));
    assert_eq!(game_logic::check_winner(&board), None);
    
    // Only black pieces
    board.set_piece(5, 2, None);
    assert_eq!(game_logic::check_winner(&board), Some(Color::Black));
} 

// Tests for can_piece_capture

#[test]
fn test_can_piece_capture_positive_white_regular() {
    let mut board = Board::new(8);
    let white_piece = Piece::new(Color::White);
    let black_piece = Piece::new(Color::Black);
    board.set_piece(5, 2, Some(white_piece));
    board.set_piece(4, 3, Some(black_piece)); // Opponent to capture
    // Landing spot (3,4) is empty
    assert!(game_logic::can_piece_capture(&board, 5, 2));
}

#[test]
fn test_can_piece_capture_positive_black_regular() {
    let mut board = Board::new(8);
    let white_piece = Piece::new(Color::White);
    let black_piece = Piece::new(Color::Black);
    board.set_piece(2, 2, Some(black_piece));
    board.set_piece(3, 3, Some(white_piece)); // Opponent to capture
    // Landing spot (4,4) is empty
    assert!(game_logic::can_piece_capture(&board, 2, 2));
}

#[test]
fn test_can_piece_capture_positive_white_king() {
    let mut board = Board::new(8);
    let mut white_king = Piece::new(Color::White);
    white_king.promote_to_king();
    let black_piece = Piece::new(Color::Black);
    board.set_piece(5, 2, Some(white_king));
    board.set_piece(4, 3, Some(black_piece)); // Capture forward
    assert!(game_logic::can_piece_capture(&board, 5, 2));

    board.set_piece(4, 3, None); // Clear previous opponent
    board.set_piece(6, 3, Some(black_piece)); // Capture backward
    assert!(game_logic::can_piece_capture(&board, 5, 2));
}

#[test]
fn test_can_piece_capture_positive_black_king() {
    let mut board = Board::new(8);
    let white_piece = Piece::new(Color::White);
    let mut black_king = Piece::new(Color::Black);
    black_king.promote_to_king();
    board.set_piece(2, 2, Some(black_king));
    board.set_piece(3, 3, Some(white_piece)); // Capture forward
    assert!(game_logic::can_piece_capture(&board, 2, 2));

    board.set_piece(3, 3, None); // Clear previous opponent
    board.set_piece(1, 3, Some(white_piece)); // Capture backward
    assert!(game_logic::can_piece_capture(&board, 2, 2));
}

#[test]
fn test_can_piece_capture_negative_no_opponent_piece() {
    let mut board = Board::new(8);
    let white_piece = Piece::new(Color::White);
    board.set_piece(5, 2, Some(white_piece));
    // Middle spot (4,3) is empty, landing (3,4) is empty
    assert!(!game_logic::can_piece_capture(&board, 5, 2));
}

#[test]
fn test_can_piece_capture_negative_landing_blocked() {
    let mut board = Board::new(8);
    let white_piece = Piece::new(Color::White);
    let black_piece = Piece::new(Color::Black);
    board.set_piece(5, 2, Some(white_piece));
    board.set_piece(4, 3, Some(black_piece));
    board.set_piece(3, 4, Some(white_piece)); // Landing spot blocked by own piece
    assert!(!game_logic::can_piece_capture(&board, 5, 2));
}

#[test]
fn test_can_piece_capture_negative_landing_out_of_bounds() {
    let mut board = Board::new(8);
    let white_piece = Piece::new(Color::White);
    let black_piece = Piece::new(Color::Black);
    board.set_piece(1, 0, Some(white_piece));
    board.set_piece(0, 1, Some(black_piece)); // Opponent to capture
    // Landing spot (-1, 2) is out of bounds
    assert!(!game_logic::can_piece_capture(&board, 1, 0));
}

#[test]
fn test_can_piece_capture_negative_opponent_but_no_empty_landing() {
    let mut board = Board::new(8);
    let white_piece = Piece::new(Color::White);
    let black_piece = Piece::new(Color::Black);
    board.set_piece(5, 2, Some(white_piece));
    board.set_piece(4, 3, Some(black_piece));
    board.set_piece(3, 4, Some(black_piece)); // Landing spot blocked by other opponent piece
    assert!(!game_logic::can_piece_capture(&board, 5, 2));
}

#[test]
fn test_can_piece_capture_negative_wrong_direction_regular_white() {
    let mut board = Board::new(8);
    let white_piece = Piece::new(Color::White);
    let black_piece = Piece::new(Color::Black);
    board.set_piece(3, 2, Some(white_piece));
    board.set_piece(4, 3, Some(black_piece)); // Opponent is "behind" white piece
    // Landing spot (5,4) is empty
    assert!(!game_logic::can_piece_capture(&board, 3, 2));
}

#[test]
fn test_can_piece_capture_negative_wrong_direction_regular_black() {
    let mut board = Board::new(8);
    let white_piece = Piece::new(Color::White);
    let black_piece = Piece::new(Color::Black);
    board.set_piece(5, 2, Some(black_piece));
    board.set_piece(4, 3, Some(white_piece)); // Opponent is "behind" black piece
    // Landing spot (3,4) is empty
    assert!(!game_logic::can_piece_capture(&board, 5, 2));
}

#[test]
fn test_can_piece_capture_negative_no_piece_at_coords() {
    let board = Board::new(8); // Empty board
    assert!(!game_logic::can_piece_capture(&board, 5, 2));
}

#[test]
fn test_can_piece_capture_negative_piece_no_moves() {
    let mut board = Board::new(8);
    let white_piece = Piece::new(Color::White);
    board.set_piece(0, 0, Some(white_piece)); // Cornered piece
    // Fill surrounding potential jump spots to ensure no capture
    board.set_piece(1,1, Some(Piece::new(Color::White))); // Block simple move/jump
    // No opponent to jump anyway
    assert!(!game_logic::can_piece_capture(&board, 0, 0));

    let mut board2 = Board::new(8);
    let white_piece2 = Piece::new(Color::White);
    board2.set_piece(7,0, Some(white_piece2));
    board2.set_piece(6,1, Some(Piece::new(Color::White)));
    assert!(!game_logic::can_piece_capture(&board2,7,0));
}

// New module for get_all_possible_moves tests
#[cfg(test)]
mod get_all_possible_moves_tests {
    use checkers_rs::core::board::Board;
    use checkers_rs::core::game_logic::get_all_possible_moves;
    use checkers_rs::core::piece::{Color, Piece};
    use std::collections::HashSet;

    // Helper function to compare two vectors of moves ignoring order
    fn assert_moves_equal(actual: &[(usize, usize)], expected: &[(usize, usize)]) {
        let actual_set: HashSet<_> = actual.iter().cloned().collect();
        let expected_set: HashSet<_> = expected.iter().cloned().collect();
        assert_eq!(actual_set, expected_set, "Actual moves: {:?}, Expected moves: {:?}", actual, expected);
    }

    // 1. Regular Piece - No Captures
    #[test]
    fn test_regular_white_no_captures_forward() {
        let mut board = Board::new(8);
        let white_piece = Piece::new(Color::White);
        board.set_piece(5, 2, Some(white_piece)); // White piece at (5,2)

        let moves = get_all_possible_moves(&board, 5, 2);
        let expected_moves = vec![(4, 1), (4, 3)];
        assert_moves_equal(&moves, &expected_moves);
    }

    #[test]
    fn test_regular_black_no_captures_forward() {
        let mut board = Board::new(8);
        let black_piece = Piece::new(Color::Black);
        board.set_piece(2, 2, Some(black_piece)); // Black piece at (2,2)

        let moves = get_all_possible_moves(&board, 2, 2);
        let expected_moves = vec![(3, 1), (3, 3)];
        assert_moves_equal(&moves, &expected_moves);
    }

    #[test]
    fn test_regular_white_edge_no_captures() {
        let mut board = Board::new(8);
        let white_piece = Piece::new(Color::White);
        board.set_piece(5, 0, Some(white_piece)); // White piece at (5,0) edge

        let moves = get_all_possible_moves(&board, 5, 0);
        let expected_moves = vec![(4, 1)];
        assert_moves_equal(&moves, &expected_moves);

        board.set_piece(5,0, None);
        board.set_piece(5, 7, Some(white_piece)); // White piece at (5,7) other edge
        let moves_edge7 = get_all_possible_moves(&board, 5, 7);
        let expected_moves_edge7 = vec![(4, 6)];
        assert_moves_equal(&moves_edge7, &expected_moves_edge7);
    }

    #[test]
    fn test_regular_black_edge_no_captures() {
        let mut board = Board::new(8);
        let black_piece = Piece::new(Color::Black);
        board.set_piece(2, 0, Some(black_piece)); // Black piece at (2,0) edge

        let moves = get_all_possible_moves(&board, 2, 0);
        let expected_moves = vec![(3, 1)];
        assert_moves_equal(&moves, &expected_moves);
        
        board.set_piece(2,0, None);
        board.set_piece(2, 7, Some(black_piece)); // Black piece at (2,7) other edge
        let moves_edge7 = get_all_possible_moves(&board, 2, 7);
        let expected_moves_edge7 = vec![(3, 6)];
        assert_moves_equal(&moves_edge7, &expected_moves_edge7);
    }
    
    #[test]
    fn test_regular_white_blocked_no_captures() {
        let mut board = Board::new(8);
        let white_piece = Piece::new(Color::White);
        let friendly_blocking_piece = Piece::new(Color::White); // Friendly piece
        board.set_piece(5, 2, Some(white_piece));
        board.set_piece(4, 1, Some(friendly_blocking_piece)); // Blocked by friendly
        board.set_piece(4, 3, Some(friendly_blocking_piece)); // Blocked by friendly

        let moves = get_all_possible_moves(&board, 5, 2);
        let expected_moves: Vec<(usize, usize)> = vec![]; // Expect no moves
        assert_moves_equal(&moves, &expected_moves);
    }

    #[test]
    fn test_regular_black_blocked_no_captures() {
        let mut board = Board::new(8);
        let black_piece = Piece::new(Color::Black);
        let friendly_blocking_piece = Piece::new(Color::Black); // Friendly piece
        board.set_piece(2, 2, Some(black_piece));
        board.set_piece(3, 1, Some(friendly_blocking_piece)); // Blocked by friendly
        board.set_piece(3, 3, Some(friendly_blocking_piece)); // Blocked by friendly

        let moves = get_all_possible_moves(&board, 2, 2);
        let expected_moves: Vec<(usize, usize)> = vec![]; // Expect no moves
        assert_moves_equal(&moves, &expected_moves);
    }

    // 2. King - No Captures
    #[test]
    fn test_king_no_captures_all_directions() {
        let mut board = Board::new(8);
        let mut white_king = Piece::new(Color::White);
        white_king.promote_to_king();
        board.set_piece(3, 3, Some(white_king));

        let moves = get_all_possible_moves(&board, 3, 3);
        let expected_moves = vec![(2, 2), (2, 4), (4, 2), (4, 4)];
        assert_moves_equal(&moves, &expected_moves);
    }

    #[test]
    fn test_king_edge_no_captures() {
        let mut board = Board::new(8);
        let mut white_king = Piece::new(Color::White);
        white_king.promote_to_king();
        board.set_piece(0, 3, Some(white_king)); // King at top edge

        let moves = get_all_possible_moves(&board, 0, 3);
        let expected_moves = vec![(1, 2), (1, 4)];
        assert_moves_equal(&moves, &expected_moves);

        board.set_piece(0,3,None);
        board.set_piece(3, 0, Some(white_king)); // King at left edge
        let moves_left_edge = get_all_possible_moves(&board, 3, 0);
        let expected_moves_left_edge = vec![(2,1), (4,1)];
        assert_moves_equal(&moves_left_edge, &expected_moves_left_edge);
    }

    #[test]
    fn test_king_blocked_no_captures() {
        let mut board = Board::new(8);
        let mut white_king = Piece::new(Color::White);
        white_king.promote_to_king();
        let friendly_blocking_piece = Piece::new(Color::White); // Friendly piece
        board.set_piece(3, 3, Some(white_king));
        board.set_piece(2, 2, Some(friendly_blocking_piece.clone())); // Blocked by friendly
        board.set_piece(2, 4, Some(friendly_blocking_piece.clone())); // Blocked by friendly
        board.set_piece(4, 2, Some(friendly_blocking_piece.clone())); // Blocked by friendly
        board.set_piece(4, 4, Some(friendly_blocking_piece.clone())); // Blocked by friendly

        let moves = get_all_possible_moves(&board, 3, 3);
        let expected_moves: Vec<(usize, usize)> = vec![]; // Expect no moves
        assert_moves_equal(&moves, &expected_moves);
    }

    // 3. Regular Piece - Single Captures
    #[test]
    fn test_regular_white_single_capture() {
        let mut board = Board::new(8);
        let white_piece = Piece::new(Color::White);
        let black_piece = Piece::new(Color::Black);
        board.set_piece(5, 2, Some(white_piece));
        board.set_piece(4, 3, Some(black_piece)); // Opponent to capture

        let moves = get_all_possible_moves(&board, 5, 2);
        let expected_moves = vec![(3, 4)]; // Only capture move
        assert_moves_equal(&moves, &expected_moves);
    }
    
    #[test]
    fn test_regular_white_multiple_single_capture_options() {
        let mut board = Board::new(8);
        let white_piece = Piece::new(Color::White);
        let black_piece1 = Piece::new(Color::Black);
        let black_piece2 = Piece::new(Color::Black);
        board.set_piece(5, 2, Some(white_piece));
        board.set_piece(4, 1, Some(black_piece1)); // Opponent 1
        board.set_piece(4, 3, Some(black_piece2)); // Opponent 2

        let moves = get_all_possible_moves(&board, 5, 2);
        let expected_moves = vec![(3, 0), (3, 4)]; // Both capture moves
        assert_moves_equal(&moves, &expected_moves);
    }

    #[test]
    fn test_regular_black_single_capture() {
        let mut board = Board::new(8);
        let black_piece = Piece::new(Color::Black);
        let white_piece = Piece::new(Color::White);
        board.set_piece(2, 2, Some(black_piece));
        board.set_piece(3, 3, Some(white_piece)); // Opponent to capture

        let moves = get_all_possible_moves(&board, 2, 2);
        let expected_moves = vec![(4, 4)]; // Only capture move
        assert_moves_equal(&moves, &expected_moves);
    }
    
    #[test]
    fn test_regular_black_multiple_single_capture_options() {
        let mut board = Board::new(8);
        let black_piece = Piece::new(Color::Black);
        let white_piece1 = Piece::new(Color::White);
        let white_piece2 = Piece::new(Color::White);
        board.set_piece(2, 2, Some(black_piece));
        board.set_piece(3, 1, Some(white_piece1)); // Opponent 1
        board.set_piece(3, 3, Some(white_piece2)); // Opponent 2

        let moves = get_all_possible_moves(&board, 2, 2);
        let expected_moves = vec![(4, 0), (4, 4)]; // Both capture moves
        assert_moves_equal(&moves, &expected_moves);
    }

    // 4. King - Single Captures
    #[test]
    fn test_king_single_capture_all_directions() {
        let mut board = Board::new(8);
        let mut white_king = Piece::new(Color::White);
        white_king.promote_to_king();
        let black_piece = Piece::new(Color::Black);
        board.set_piece(3, 3, Some(white_king));
        board.set_piece(2, 2, Some(black_piece.clone())); // Top-left
        board.set_piece(2, 4, Some(black_piece.clone())); // Top-right
        board.set_piece(4, 2, Some(black_piece.clone())); // Bottom-left
        board.set_piece(4, 4, Some(black_piece.clone())); // Bottom-right

        let moves = get_all_possible_moves(&board, 3, 3);
        let expected_moves = vec![(1, 1), (1, 5), (5, 1), (5, 5)];
        assert_moves_equal(&moves, &expected_moves);
    }
    
    #[test]
    fn test_king_single_capture_prefers_capture_over_regular() {
        let mut board = Board::new(8);
        let mut white_king = Piece::new(Color::White);
        white_king.promote_to_king();
        let black_piece = Piece::new(Color::Black);
        board.set_piece(3, 3, Some(white_king));
        board.set_piece(2, 2, Some(black_piece.clone())); // Opponent for capture

        // Add other pieces to make sure regular moves would be possible if not for capture
        // board.set_piece(2, 4, None); // Empty for potential regular move
        // board.set_piece(4, 2, None); // Empty for potential regular move
        // board.set_piece(4, 4, None); // Empty for potential regular move
        
        let moves = get_all_possible_moves(&board, 3, 3);
        let expected_moves = vec![(1, 1)]; // Only the capture move
        assert_moves_equal(&moves, &expected_moves);
    }

    // 5. Regular Piece - Multi-Captures
    #[test]
    fn test_regular_white_double_capture() {
        let mut board = Board::new(8);
        let white_piece = Piece::new(Color::White);
        let black_piece1 = Piece::new(Color::Black);
        let black_piece2 = Piece::new(Color::Black);
        board.set_piece(7, 0, Some(white_piece));
        board.set_piece(6, 1, Some(black_piece1));
        // (5,2) is empty
        board.set_piece(4, 3, Some(black_piece2));
        // (3,4) is empty

        let moves = get_all_possible_moves(&board, 7, 0);
        // The final landing position after the sequence of jumps
        let expected_moves = vec![(3, 4)];
        assert_moves_equal(&moves, &expected_moves);
    }

    #[test]
    fn test_regular_black_double_capture() {
        let mut board = Board::new(8);
        let black_piece = Piece::new(Color::Black);
        let white_piece1 = Piece::new(Color::White);
        let white_piece2 = Piece::new(Color::White);
        board.set_piece(0, 1, Some(black_piece));
        board.set_piece(1, 2, Some(white_piece1));
        // (2,3) is empty
        board.set_piece(3, 4, Some(white_piece2));
        // (4,5) is empty

        let moves = get_all_possible_moves(&board, 0, 1);
        let expected_moves = vec![(4, 5)];
        assert_moves_equal(&moves, &expected_moves);
    }
    
    #[test]
    fn test_regular_white_multiple_multi_capture_paths() {
        let mut board = Board::new(8);
        // W . . .
        // . B . .
        // . . W . (start)
        // . B . B .
        // . . . . .
        // . B . .
        // W . . .
        // Path 1: (4,2) -> (2,0)
        // Path 2: (4,2) -> (2,4) -> (0,6) (if board was larger, or piece was king)
        // For regular piece, it will be (4,2) -> (2,4) only if no further jump in that dir.
        // Let's set up simpler:
        //    . . . . . . .
        //    . B . B . . .  (B at 1,1 and 1,3)
        //    . . W . . . .  (W at 2,2)
        //    . B . B . . .  (B at 3,1 and 3,3)
        //    . . . . . . .
        // White piece at (4,3)
        // Opponent at (3,2) -> jump to (2,1)
        // Opponent at (1,0) -> jump to (0,0) X (cannot jump backwards)
        // Opponent at (3,4) -> jump to (2,5)
        // Opponent at (1,6) -> jump to (0,7) X (cannot jump backwards)
        //
        // Let's make it simpler for regular piece, must move forward.
        // W at (6,1)
        // B at (5,2) -> land (4,3)
        // B at (3,4) -> land (2,5)  (Path 1: (2,5))
        //
        // W at (6,1)
        // B at (5,0) -> land (4,0)
        // B at (3,0) -> land (2,0) X (blocked by previous jump piece, this needs careful thought for test)
        // The test should reflect that board state changes *during* the sequence for one path.
        //
        // Scenario: W at (6,1). B at (5,2). B at (3,2).
        // (6,1) -> captures (5,2) -> lands at (4,3)
        // From (4,3) -> captures (3,2) -> lands at (2,1)
        // Expected: (2,1)
        board.set_piece(6,1, Some(Piece::new(Color::White)));
        board.set_piece(5,2, Some(Piece::new(Color::Black))); // first capture
        // (4,3) is landing
        board.set_piece(3,2, Some(Piece::new(Color::Black))); // second capture
        // (2,1) is final landing

        // Scenario: W at (6,5)
        // B at (5,4) -> lands (4,3)
        // B at (3,2) -> lands (2,1) (Path A)
        //
        // B at (5,6) -> lands (4,7)
        // B at (3,6) -> lands (2,5) (Path B)
        // Expected: (2,1), (2,5)

        board.set_piece(6, 5, Some(Piece::new(Color::White)));
        // Path A
        board.set_piece(5, 4, Some(Piece::new(Color::Black))); // 1st jump for Path A
        board.set_piece(3, 2, Some(Piece::new(Color::Black))); // 2nd jump for Path A
        // Path B
        board.set_piece(5, 6, Some(Piece::new(Color::Black))); // 1st jump for Path B
        board.set_piece(3, 6, Some(Piece::new(Color::Black))); // 2nd jump for Path B
        
        let moves = get_all_possible_moves(&board, 6, 5);
        let expected_moves = vec![(2,1), (2,5)];
        assert_moves_equal(&moves, &expected_moves);
    }


    // 6. King - Multi-Captures
    #[test]
    fn test_king_multi_capture_changing_directions() {
        let mut board = Board::new(8);
        let mut white_king = Piece::new(Color::White);
        white_king.promote_to_king();
        let black_piece1 = Piece::new(Color::Black);
        let black_piece2 = Piece::new(Color::Black);

        // K at (3,3)
        // B at (2,2) -> K lands at (1,1)
        // B at (2,0) -> K lands at (3,-1) X out of bounds
        // B at (0,2) -> K lands at (-1,3) X out of bounds
        // Let's try: K at (3,3). B at (2,2). B at (2,4)
        // K captures (2,2) -> lands (1,1)
        // From (1,1), K captures (2,4) (which is now opponent at (2,4) relative to original board, but (1,3) relative to (1,1))
        // This means K needs to capture (2,2) -> (1,1), then from (1,1) capture an opponent at (2,0) or (0,2) or (0,0) etc.
        // Setup: King at (3,3). Opponent at (2,2). Opponent at (0,2).
        // (3,3) captures (2,2) -> lands at (1,1).
        // From (1,1), captures (0,2) -> lands at (-1,3) X - no (0,2) is absolute
        // From (1,1), captures piece at (2,0) relative to (1,1) i.e. absolute (3,1) No.
        // From (1,1), captures piece at (abs 2,2) which is the one just jumped? No.
        
        // King at (4,3)
        // B at (3,2) -> K lands at (2,1) // Forward-left capture
        // B at (3,0) -> K lands at (4,-1) X // Backward-left capture from (2,1)
        // Let's place second B at (1,2) relative to original board.
        // King at (4,3). B1 at (3,2). B2 at (1,2).
        // (4,3) captures B1 at (3,2) -> lands at (2,1).
        // From (2,1), captures B2 at (1,2) -> lands at (0,3).
        board.set_piece(4, 3, Some(white_king));
        board.set_piece(3, 2, Some(black_piece1));
        board.set_piece(1, 2, Some(black_piece2));

        let moves = get_all_possible_moves(&board, 4, 3);
        let expected_moves = vec![(0, 3)];
        assert_moves_equal(&moves, &expected_moves);
    }
    
    #[test]
    fn test_king_multiple_multi_capture_paths() {
        let mut board = Board::new(8);
        let mut king = Piece::new(Color::White);
        king.promote_to_king();
        board.set_piece(3,3, Some(king));

        // Path 1: (3,3) -> (1,1) -> (-1,-1) (lands (1,1))
        board.set_piece(2,2, Some(Piece::new(Color::Black))); // cap1_1

        // Path 2: (3,3) -> (1,5) -> (-1,7) (lands (1,5))
        board.set_piece(2,4, Some(Piece::new(Color::Black))); // cap2_1
        
        // Path 3: (3,3) -> (5,1) -> (7,-1) (lands (5,1))
        board.set_piece(4,2, Some(Piece::new(Color::Black))); // cap3_1

        // Path 4: (3,3) -> (5,5) -> (7,7) (lands (5,5))
        board.set_piece(4,4, Some(Piece::new(Color::Black))); // cap4_1

        // Add second jumps to make them multi
        // Path 1 extension: (1,1) -> captures (0,2) -> lands (-1,3) X
        // Let's make path 1: (3,3) cap (2,2) land (1,1). Then from (1,1) cap (0,0) land (-1,-1) X
        // (3,3) cap (2,2) land (1,1). Then from (1,1) cap (2,0) land (3,-1) X

        // Simpler multi-jump paths for king:
        // K at (3,3)
        // Path A: B at (2,2) -> K lands (1,1). Then B at (0,2) -> K lands (-1,3) X
        // Path A: B at (2,2) -> K lands (1,1). Then B at (2,0) -> K lands (3,-1) X
        // Let's make pieces available for two distinct multi-jump paths
        // K at (3,3)
        // Path A: Capture (2,2) to land (1,1). Then capture (0,0) to land (-1,-1) X
        // Path A: Capture (2,2) to land (1,1). Then capture (2,0) to land (3,-1) X.
        // This is hard to setup without pieces overlapping if not careful.

        // K at (3,3)
        // Path A: (2,2) and (0,2) -> Expected: (-1,3) X.  (2,2) and (2,0)
        // (3,3) -> cap (2,2) -> land (1,1). From (1,1) -> cap (2,0) -> land (3,-1) X. (abs coords for 2nd piece: (2,0))
        // (3,3) -> cap (2,2) -> land (1,1). From (1,1) -> cap (0,2) -> land (-1,3) X (abs coords for 2nd piece: (0,2))
        
        // Reset board for clarity
        board = Board::new(8);
        let mut k = Piece::new(Color::White);
        k.promote_to_king();
        board.set_piece(3,3, Some(k));

        // Path 1: (3,3) capture (2,2) to (1,1), then capture (0,2) to (-1,3) X
        // Path 1: (3,3) capture (2,2) to (1,1), then capture (2,0) to (3,-1) X
        // Path 1: (3,3) capture (2,2) to (1,1) [first jump]. Opponent at (0,0) for second jump. Lands (-1,-1) X
        // Let's use the example from prompt description for get_all_possible_moves if possible.
        // For now, two separate double jumps.
        // Path A: (3,3) -> (1,1) -> (-1,-1)  Requires B at (2,2) and B at (0,0)
        board.set_piece(2,2, Some(Piece::new(Color::Black))); // Path A, jump 1
        board.set_piece(0,0, Some(Piece::new(Color::Black))); // Path A, jump 2 -> lands (-1,-1) X. Should be (2,2) then (0,0)
                                                                // (3,3) via (2,2) lands (1,1). From (1,1) via (0,0) lands (-1,-1). This is not possible.
                                                                // The piece at (0,0) is jumped from (1,1).
        // Path B: (3,3) -> (1,5) -> (-1,7) Requires B at (2,4) and B at (0,6)
        board.set_piece(2,4, Some(Piece::new(Color::Black))); // Path B, jump 1
        board.set_piece(0,6, Some(Piece::new(Color::Black))); // Path B, jump 2 -> lands (-1,7) X.

        // Let's use a concrete example that works on 8x8
        // King at (7,0)
        // B at (6,1) -> lands (5,2)
        // B at (4,1) -> lands (3,0) (Path 1: (3,0))
        //
        // King at (7,0)
        // B at (5,2) (this is the landing spot of first jump, so cannot be another piece)
        //
        // Need distinct pieces for distinct paths.
        // K at (4,3)
        // Path 1: B at (3,2) -> (2,1). B at (1,0) -> (0,0-invalid) X. (1,2) -> (0,3).
        // (4,3) cap (3,2) land (2,1). Then cap (1,2) land (0,3). Path 1: (0,3)
        // Path 2: (4,3) cap (3,4) land (2,5). Then cap (1,4) land (0,3). Path 2: (0,3)
        // Path 2: (4,3) cap (3,4) land (2,5). Then cap (1,6) land (0,7). Path 2: (0,7)
        //
        let mut king_piece = Piece::new(Color::White);
        king_piece.promote_to_king();
        board.set_piece(4,3, Some(king_piece));
        //Path 1
        board.set_piece(3,2, Some(Piece::new(Color::Black)));
        board.set_piece(1,2, Some(Piece::new(Color::Black)));
        //Path 2
        board.set_piece(3,4, Some(Piece::new(Color::Black)));
        board.set_piece(1,6, Some(Piece::new(Color::Black)));
        
        let moves = get_all_possible_moves(&board, 4, 3);
        let expected_moves = vec![(0,3), (0,7)];
        assert_moves_equal(&moves, &expected_moves);
    }

    // 7. No Moves Possible
    #[test]
    fn test_regular_piece_no_moves_possible() {
        let mut board = Board::new(8);
        
        // Scenario 1: White piece at starting row, blocked by friendly pieces or edge
        let white_piece1 = Piece::new(Color::White);
        board.set_piece(7, 0, Some(white_piece1)); // Piece at an edge
        board.set_piece(6, 1, Some(Piece::new(Color::White))); // Blocked by friendly piece
        let moves1 = get_all_possible_moves(&board, 7, 0);
        assert_moves_equal(&moves1, &[]);

        // Scenario 2: White piece at promotion row, cannot move backward (not a king)
        // and forward moves are blocked by friendly pieces or edge.
        let white_piece2 = Piece::new(Color::White);
        board.set_piece(0, 1, Some(white_piece2)); // At promotion row
        // Assuming it cannot move backward. If it could, these would block forward.
        // No need to add blockers if it cannot move from promotion row unless it's a king.
        // If it's at (0,1), it can only move to (-1,0) or (-1,2) which is out of bounds.
        let moves2 = get_all_possible_moves(&board, 0, 1);
        assert_moves_equal(&moves2, &[]);

        // Scenario 3: White piece surrounded by friendly pieces
        let white_piece3 = Piece::new(Color::White);
        board.set_piece(5, 2, Some(white_piece3));
        board.set_piece(4, 1, Some(Piece::new(Color::White)));
        board.set_piece(4, 3, Some(Piece::new(Color::White)));
        let moves3 = get_all_possible_moves(&board, 5, 2);
        assert_moves_equal(&moves3, &[]);
    }

    #[test]
    fn test_king_no_moves_possible() {
        let mut board = Board::new(8);
        let mut white_king = Piece::new(Color::White);
        white_king.promote_to_king();

        // Scenario 1: King at corner, blocked by friendly piece
        board.set_piece(0, 0, Some(white_king)); 
        board.set_piece(1, 1, Some(Piece::new(Color::White))); // Blocked by friendly
        let moves1 = get_all_possible_moves(&board, 0, 0);
        assert_moves_equal(&moves1, &[]);

        // Scenario 2: King surrounded by friendly pieces
        board.set_piece(0,0,None); // Clear previous
        board.set_piece(1,1,None);
        board.set_piece(3,3, Some(white_king));
        board.set_piece(2,2, Some(Piece::new(Color::White)));
        board.set_piece(2,4, Some(Piece::new(Color::White)));
        board.set_piece(4,2, Some(Piece::new(Color::White)));
        board.set_piece(4,4, Some(Piece::new(Color::White)));
        let moves_surrounded = get_all_possible_moves(&board,3,3);
        assert_moves_equal(&moves_surrounded, &[]);
    }
}