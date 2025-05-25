use checkers_rs::core::game::{CheckersGame, GameError};
use checkers_rs::core::piece::{Color, Piece};

#[test]
fn test_new_game() {
    let game = CheckersGame::new();
    assert_eq!(game.current_player, Color::White);
    assert!(!game.is_game_over);
    assert_eq!(game.selected_piece, None);
}

#[test]
fn test_select_piece() {
    let mut game = CheckersGame::new();

    // Try to select empty square
    assert!(matches!(
        game.select_piece(3, 3),
        Err(GameError::NoPieceSelected)
    ));

    // Try to select opponent's piece
    assert!(matches!(
        game.select_piece(0, 1),
        Err(GameError::WrongPieceColor)
    ));

    // Select own piece (white piece at row 5, col 0)
    assert!(game.select_piece(5, 0).is_ok());
    assert_eq!(game.selected_piece, Some((5, 0)));
}

#[test]
fn test_valid_moves() {
    let mut game = CheckersGame::new();

    // Select a white piece at (5, 0)
    assert!(game.select_piece(5, 0).is_ok());

    // Try valid diagonal move
    assert!(game.make_move(4, 1).is_ok());
    assert_eq!(game.current_player, Color::Black);
}

#[test]
fn test_invalid_moves() {
    let mut game = CheckersGame::new();

    // Try to move without selecting a piece
    assert!(matches!(
        game.make_move(4, 4),
        Err(GameError::NoPieceSelected)
    ));

    // Select a piece and try invalid moves
    assert!(game.select_piece(5, 0).is_ok());

    // Try to move horizontally
    assert!(matches!(game.make_move(5, 1), Err(GameError::InvalidMove)));

    // Try to move backwards
    assert!(matches!(game.make_move(6, 1), Err(GameError::InvalidMove)));

    // Try to move out of bounds
    assert!(matches!(game.make_move(8, 8), Err(GameError::OutOfBounds)));
}

#[test]
fn test_capture_move() {
    let mut game = CheckersGame::new();

    // Clear the board first
    for row in 0..game.board.size {
        for col in 0..game.board.size {
            game.board.set_piece(row, col, None);
        }
    }

    // Setup a capture scenario
    let white_piece = Piece::new(Color::White);
    let black_piece = Piece::new(Color::Black);

    // Place pieces for the capture
    game.board.set_piece(4, 1, Some(white_piece));
    game.board.set_piece(3, 2, Some(black_piece));

    // Select the piece and perform capture
    assert!(game.select_piece(4, 1).is_ok());
    assert!(game.make_move(2, 3).is_ok());

    // Verify the capture
    assert!(game.board.get_piece(3, 2).is_none());
    assert!(game.board.get_piece(2, 3).is_some());
}

#[test]
fn test_king_promotion() {
    let mut game = CheckersGame::new();

    // Clear the board first
    for row in 0..game.board.size {
        for col in 0..game.board.size {
            game.board.set_piece(row, col, None);
        }
    }

    // Place a white piece near promotion
    let white_piece = Piece::new(Color::White);
    game.board.set_piece(1, 2, Some(white_piece));

    // Select and move to promotion square
    assert!(game.select_piece(1, 2).is_ok());
    assert!(game.make_move(0, 3).is_ok());

    // Verify promotion
    let promoted_piece = game.board.get_piece(0, 3).unwrap();
    assert!(promoted_piece.is_king);
}

#[test]
fn test_game_over() {
    let mut game = CheckersGame::new();

    // Remove all black pieces
    for row in 0..3 {
        for col in 0..8 {
            if (row + col) % 2 == 1 {
                game.board.set_piece(row, col, None);
            }
        }
    }

    assert_eq!(game.check_winner(), Some(Color::White));
}

#[test]
fn test_switch_player() {
    let mut game = CheckersGame::new();
    assert_eq!(game.current_player, Color::White);

    game.switch_player();
    assert_eq!(game.current_player, Color::Black);

    game.switch_player();
    assert_eq!(game.current_player, Color::White);
}

#[test]
fn test_king_movement() {
    let mut game = CheckersGame::new();

    // Clear the board first
    for row in 0..game.board.size {
        for col in 0..game.board.size {
            game.board.set_piece(row, col, None);
        }
    }

    // Create a king and place it on the board
    let mut king = Piece::new(Color::White);
    king.promote_to_king();
    game.board.set_piece(4, 1, Some(king));

    // Test king can move in any diagonal direction
    assert!(game.select_piece(4, 1).is_ok());

    // Move backwards (which regular pieces can't do)
    assert!(game.make_move(5, 2).is_ok());
}

#[test]
fn test_multiple_jump_paths() {
    let mut game = CheckersGame::new();

    // Clear the board first
    for row in 0..game.board.size {
        for col in 0..game.board.size {
            game.board.set_piece(row, col, None);
        }
    }

    // Setup a scenario with multiple capture paths
    let white_piece = Piece::new(Color::White);
    let black_piece = Piece::new(Color::Black);

    // Place pieces for multiple capture paths
    game.board.set_piece(6, 1, Some(white_piece));
    game.board.set_piece(5, 2, Some(black_piece));
    game.board.set_piece(3, 2, Some(black_piece));
    game.board.set_piece(5, 4, Some(black_piece));

    // Select the white piece
    assert!(game.select_piece(6, 1).is_ok());

    // First capture
    assert!(game.make_move(4, 3).is_ok());
    assert!(game.board.get_piece(5, 2).is_none()); // Captured piece should be removed

    // Second capture (multiple options available)
    assert!(game.make_move(2, 1).is_ok());
    assert!(game.board.get_piece(3, 2).is_none());

    // Verify final position
    assert!(game.board.get_piece(2, 1).is_some());
}

#[test]
fn test_forced_capture() {
    let mut game = CheckersGame::new();

    // Clear the board first
    for row in 0..game.board.size {
        for col in 0..game.board.size {
            game.board.set_piece(row, col, None);
        }
    }

    // Setup a scenario where capture is available
    let white_piece = Piece::new(Color::White);
    let black_piece = Piece::new(Color::Black);

    // Place pieces: white piece can either move or capture
    game.board.set_piece(5, 2, Some(white_piece));
    game.board.set_piece(4, 3, Some(black_piece));

    // Try to make a regular move when capture is available
    assert!(game.select_piece(5, 2).is_ok());
    assert!(matches!(
        game.make_move(4, 1),
        Err(GameError::ForcedCaptureAvailable)
    ));

    // Make the capture move
    assert!(game.make_move(3, 4).is_ok());
    assert!(game.board.get_piece(4, 3).is_none()); // Captured piece should be removed
}

#[test]
fn test_king_promotion_with_continued_capture() {
    let mut game = CheckersGame::new();

    // Clear the board first
    for row in 0..game.board.size {
        for col in 0..game.board.size {
            game.board.set_piece(row, col, None);
        }
    }

    // Setup a scenario where piece gets promoted and can continue capturing
    let white_piece = Piece::new(Color::White);
    let black_piece = Piece::new(Color::Black);

    // Place pieces for promotion and subsequent capture
    game.board.set_piece(1, 2, Some(white_piece));
    game.board.set_piece(2, 3, Some(black_piece));

    // Select the white piece
    assert!(game.select_piece(1, 2).is_ok());

    // Move to promotion square and capture
    assert!(game.make_move(0, 3).is_ok());

    // Verify promotion
    let promoted_piece = game.board.get_piece(0, 3).unwrap();
    assert!(promoted_piece.is_king);

    // Verify the piece can continue capturing as a king
    if game.has_captures_available() {
        assert!(game.select_piece(0, 3).is_ok());
        assert!(game.make_move(2, 5).is_ok());
        assert!(game.board.get_piece(1, 4).is_none()); // Captured piece should be removed
    }
}

#[test]
fn test_move_without_selection() {
    let mut game = CheckersGame::new();

    // Try to make a move without selecting a piece
    assert!(matches!(
        game.make_move(4, 4),
        Err(GameError::NoPieceSelected)
    ));
}

#[test]
fn test_select_empty_square() {
    let mut game = CheckersGame::new();

    // Try to select an empty square in the middle of the board
    assert!(matches!(
        game.select_piece(3, 3),
        Err(GameError::NoPieceSelected)
    ));
}

#[test]
fn test_out_of_bounds_moves() {
    let mut game = CheckersGame::new();

    // Select a valid piece first
    assert!(game.select_piece(5, 0).is_ok());

    // Try various out of bounds moves
    assert!(matches!(game.make_move(8, 0), Err(GameError::OutOfBounds)));
    assert!(matches!(game.make_move(0, 8), Err(GameError::OutOfBounds)));

    // Try to select out of bounds
    assert!(matches!(
        game.select_piece(8, 0),
        Err(GameError::OutOfBounds)
    ));
    assert!(matches!(
        game.select_piece(0, 8),
        Err(GameError::OutOfBounds)
    ));
}

#[test]
fn test_select_opponent_piece() {
    let mut game = CheckersGame::new();

    // Try to select black piece when it's white's turn
    assert!(matches!(
        game.select_piece(2, 1),
        Err(GameError::WrongPieceColor)
    ));

    // Make a valid move for white
    assert!(game.select_piece(5, 0).is_ok());
    assert!(game.make_move(4, 1).is_ok());

    // Try to select white piece when it's black's turn
    assert!(matches!(
        game.select_piece(5, 2),
        Err(GameError::WrongPieceColor)
    ));
}

#[test]
fn test_stalemate_condition() {
    let mut game = CheckersGame::new();

    // Clear the board first
    for row in 0..game.board.size {
        for col in 0..game.board.size {
            game.board.set_piece(row, col, None);
        }
    }

    // Setup a stalemate scenario where white has no legal moves
    let white_piece = Piece::new(Color::White);
    let black_piece = Piece::new(Color::Black);

    // Place white piece in corner
    game.board.set_piece(7, 0, Some(white_piece));

    // Place black pieces to block all possible moves
    // Block forward-right diagonal
    game.board.set_piece(6, 1, Some(black_piece));
    // Block the landing square for any potential capture
    game.board.set_piece(5, 2, Some(black_piece));

    // Verify stalemate condition
    assert!(game.is_stalemate());

    // Make sure it's not a stalemate when there are valid moves
    game.board.set_piece(6, 1, None);
    game.board.set_piece(5, 2, None);
    assert!(!game.is_stalemate());
}

#[test]
fn test_toggle_piece_selection() {
    let mut game = CheckersGame::new();

    // Select a white piece at (5, 0)
    assert!(game.select_piece(5, 0).is_ok());
    assert_eq!(game.selected_piece, Some((5, 0)));

    // Toggle selection off by selecting the same piece
    assert!(game.select_piece(5, 0).is_ok());
    assert_eq!(game.selected_piece, None);

    // Select the piece again
    assert!(game.select_piece(5, 0).is_ok());
    assert_eq!(game.selected_piece, Some((5, 0)));

    // Select a different valid piece - should switch selection
    assert!(game.select_piece(5, 2).is_ok());
    assert_eq!(game.selected_piece, Some((5, 2)));
}

#[test]
fn test_toggle_piece_selection_with_no_moves() {
    let mut game = CheckersGame::new();

    // Clear the board first
    for row in 0..game.board.size {
        for col in 0..game.board.size {
            game.board.set_piece(row, col, None);
        }
    }

    // Place a white piece in a position with no moves
    let white_piece = Piece::new(Color::White);
    game.board.set_piece(7, 0, Some(white_piece));

    // Place black pieces to block all possible moves
    let black_piece = Piece::new(Color::Black);
    game.board.set_piece(6, 1, Some(black_piece));

    // Select the white piece
    assert!(game.select_piece(7, 0).is_ok());
    assert_eq!(game.selected_piece, Some((7, 0)));

    // Toggle selection off even though there are no moves
    assert!(game.select_piece(7, 0).is_ok());
    assert_eq!(game.selected_piece, None);
}

// --- Tests for Forced Jump Logic ---

#[test]
fn test_select_non_capturing_fails_when_capture_exists() {
    let mut game = CheckersGame::new();
    // Clear board
    for r in 0..game.board.size {
        for c in 0..game.board.size {
            game.board.set_piece(r, c, None);
        }
    }

    // Piece A can capture, Piece B cannot
    let white_a = Piece::new(Color::White);
    let white_b = Piece::new(Color::White);
    let black_x = Piece::new(Color::Black);

    game.board.set_piece(5, 0, Some(white_a)); // Piece A
    game.board.set_piece(4, 1, Some(black_x)); // Opponent for A
                                               // Landing for A is (3,2)

    game.board.set_piece(5, 4, Some(white_b)); // Piece B, can only move to (4,3) or (4,5)

    // Attempt to select Piece B (non-capturing)
    assert!(matches!(
        game.select_piece(5, 4),
        Err(GameError::ForcedCaptureAvailable)
    ));
}

#[test]
fn test_select_capturing_succeeds_when_capture_exists() {
    let mut game = CheckersGame::new();
    for r in 0..game.board.size {
        for c in 0..game.board.size {
            game.board.set_piece(r, c, None);
        }
    }

    let white_a = Piece::new(Color::White);
    let white_b = Piece::new(Color::White);
    let black_x = Piece::new(Color::Black);

    game.board.set_piece(5, 0, Some(white_a)); 
    game.board.set_piece(4, 1, Some(black_x)); 
    game.board.set_piece(5, 4, Some(white_b));

    // Attempt to select Piece A (capturing)
    assert!(game.select_piece(5, 0).is_ok());
    assert_eq!(game.selected_piece, Some((5,0)));
}

#[test]
fn test_select_any_piece_succeeds_when_no_capture_exists() {
    let mut game = CheckersGame::new();
    for r in 0..game.board.size {
        for c in 0..game.board.size {
            game.board.set_piece(r, c, None);
        }
    }
    let white_a = Piece::new(Color::White);
    let white_b = Piece::new(Color::White);
    game.board.set_piece(5, 0, Some(white_a));
    game.board.set_piece(5, 2, Some(white_b));
    // No black pieces, so no captures available

    assert!(game.select_piece(5, 0).is_ok());
    assert_eq!(game.selected_piece, Some((5,0)));
    // Deselect to select another
    assert!(game.select_piece(5,0).is_ok()); 
    assert_eq!(game.selected_piece, None);

    assert!(game.select_piece(5, 2).is_ok());
    assert_eq!(game.selected_piece, Some((5,2)));
}

#[test]
fn test_make_non_capture_move_fails_when_capture_exists_detailed() {
    // This test re-confirms/details the behavior already in `test_forced_capture`
    let mut game = CheckersGame::new();
    for r in 0..game.board.size {
        for c in 0..game.board.size {
            game.board.set_piece(r, c, None);
        }
    }
    let black_piece = Piece::new(Color::Black);

    // Place black pieces for the king to capture
    game.board.set_piece(4, 1, Some(black_piece)); // Opponent 1
    game.board.set_piece(4, 3, Some(black_piece)); // Opponent 2

    // Place other white piece (not involved in this specific test but part of original setup)
    game.board.set_piece(6,5, Some(Piece::new(Color::White)));

    // Define and place the white KING piece
    let mut white_king = Piece::new(Color::White);
    white_king.promote_to_king();
    game.board.set_piece(5,2, Some(white_king));

    // Ensure the non-capture target square (6,1) is empty
    game.board.set_piece(6,1, None);

    // Select the KING at (5,2)
    assert!(game.select_piece(5, 2).is_ok());

    // Assert that attempting the non-capture move to (6,1) results in ForcedCaptureAvailable
    assert!(matches!(
        game.make_move(6, 1), // Attempt non-capturing move
        Err(GameError::ForcedCaptureAvailable)
    ));

    // Make one of the captures (optional, but good to keep to ensure captures are still possible)
    // Note: After the failed make_move above, the king is still selected.
    // If the previous make_move was Err, current_player would not change, selected_piece would not change.
    assert!(game.make_move(3,0).is_ok()); // Capture to (3,0) via (4,1)
    assert!(game.board.get_piece(4,1).is_none());
}


#[test]
fn test_must_make_single_available_jump_detailed() {
    let mut game = CheckersGame::new();
    for r in 0..game.board.size { for c in 0..game.board.size { game.board.set_piece(r, c, None); } }

    let white_a = Piece::new(Color::White);
    let black_x = Piece::new(Color::Black);
    game.board.set_piece(5, 0, Some(white_a)); // Piece A
    game.board.set_piece(4, 1, Some(black_x)); // Opponent X
                                               // Landing for A is (3,2)
    // Ensure no other white pieces or moves complicate the scenario
    // e.g. block other moves for white_a if it were a king.

    assert!(game.select_piece(5, 0).is_ok());
    assert!(game.make_move(3, 2).is_ok());
    assert!(game.board.get_piece(4, 1).is_none(), "Opponent X should be removed");
    assert!(game.board.get_piece(3, 2).is_some(), "Piece A should be at (3,2)");
    assert_eq!(game.board.get_piece(3,2).unwrap().color, Color::White);
    // Check for promotion if applicable (e.g. if landing row was 0 for white)
    // In this case, (3,2) is not a promotion row.
}

#[test]
fn test_can_choose_between_multiple_capturing_pieces() {
    let mut game = CheckersGame::new();

    let setup_board = |game: &mut CheckersGame| {
        for r in 0..game.board.size { for c in 0..game.board.size { game.board.set_piece(r, c, None); } }
        let white_a = Piece::new(Color::White);
        let white_b = Piece::new(Color::White);
        let black_x = Piece::new(Color::Black);
        let black_y = Piece::new(Color::Black);

        game.board.set_piece(5, 0, Some(white_a)); // Piece A
        game.board.set_piece(4, 1, Some(black_x)); // Opponent X for A (landing (3,2))

        game.board.set_piece(5, 4, Some(white_b)); // Piece B
        game.board.set_piece(4, 5, Some(black_y)); // Opponent Y for B (landing (3,6))
    };

    // Scenario 1: Choose Piece A
    setup_board(&mut game);
    assert_eq!(game.current_player, Color::White);
    assert!(game.select_piece(5, 0).is_ok(), "Should be able to select Piece A");
    assert!(game.make_move(3, 2).is_ok(), "Piece A should make its capture");
    assert!(game.board.get_piece(4, 1).is_none(), "Opponent X should be removed");
    assert!(game.board.get_piece(3, 2).is_some(), "Piece A should be at (3,2)");
    assert_eq!(game.current_player, Color::Black, "Player should switch after capture sequence ends");


    // Scenario 2: Choose Piece B
    setup_board(&mut game);
    game.current_player = Color::White; // Reset player
    assert!(game.select_piece(5, 4).is_ok(), "Should be able to select Piece B");
    assert!(game.make_move(3, 6).is_ok(), "Piece B should make its capture");
    assert!(game.board.get_piece(4, 5).is_none(), "Opponent Y should be removed");
    assert!(game.board.get_piece(3, 6).is_some(), "Piece B should be at (3,6)");
    assert_eq!(game.current_player, Color::Black, "Player should switch after capture sequence ends");
}


#[test]
fn test_must_continue_multi_jump_detailed() {
    let mut game = CheckersGame::new();
    for r in 0..game.board.size { for c in 0..game.board.size { game.board.set_piece(r, c, None); } }

    let white_a = Piece::new(Color::White);
    let black_x = Piece::new(Color::Black);
    let black_z = Piece::new(Color::Black);

    game.board.set_piece(6, 1, Some(white_a)); // Piece A
    game.board.set_piece(5, 2, Some(black_x)); // Opponent X, landing for A is (4,3)
    game.board.set_piece(3, 4, Some(black_z)); // Opponent Z, for A to jump from (4,3) to (2,5)

    // Select A and make the first jump
    assert!(game.select_piece(6, 1).is_ok());
    assert!(game.make_move(4, 3).is_ok(), "First jump should succeed");
    assert!(game.board.get_piece(5, 2).is_none(), "Opponent X removed");
    assert!(game.board.get_piece(4, 3).is_some(), "Piece A at (4,3)");
    
    // Verify game state for multi-jump
    assert_eq!(game.selected_piece, Some((4, 3)), "Piece A should remain selected at new position");
    assert_eq!(game.current_player, Color::White, "Player should not change during multi-jump");

    // Make the second jump
    assert!(game.make_move(2, 5).is_ok(), "Second jump should succeed");
    assert!(game.board.get_piece(3, 4).is_none(), "Opponent Z removed");
    assert!(game.board.get_piece(2, 5).is_some(), "Piece A at (2,5)");
    
    // Verify game state after multi-jump sequence ends
    assert_eq!(game.selected_piece, None, "Piece should be deselected after multi-jump sequence");
    assert_eq!(game.current_player, Color::Black, "Player should switch after multi-jump sequence");
}
