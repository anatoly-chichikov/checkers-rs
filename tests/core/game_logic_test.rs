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