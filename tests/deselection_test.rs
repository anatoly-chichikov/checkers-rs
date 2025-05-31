use checkers_rs::core::board::Board;
use checkers_rs::core::game::CheckersGame;
use checkers_rs::core::piece::{Color, Piece};

#[test]
fn test_deselection_allowed_when_clicking_outside_possible_moves() {
    let mut game = CheckersGame::new();

    // Select a white piece that has normal moves
    game.select_piece(5, 0).unwrap();
    assert!(game.selected_piece.is_some());
    assert!(game.possible_moves.is_some());

    // Simulate clicking outside possible moves by checking is_in_multi_capture
    // In normal gameplay, deselection should be allowed
    assert!(!game.is_in_multi_capture());

    // Manually clear selection (in main.rs this happens when clicking outside)
    game.selected_piece = None;
    game.possible_moves = None;

    assert!(game.selected_piece.is_none());
    assert!(game.possible_moves.is_none());
}

#[test]
fn test_deselection_blocked_during_multi_capture() {
    let mut game = CheckersGame::new();
    game.board = Board::new(8);

    // Set up a multi-capture scenario
    // White piece at (5, 2) can capture black at (4, 3) and then at (2, 5)
    game.board.set_piece(5, 2, Some(Piece::new(Color::White)));
    game.board.set_piece(4, 3, Some(Piece::new(Color::Black)));
    game.board.set_piece(2, 5, Some(Piece::new(Color::Black)));

    // Select the white piece
    game.select_piece(5, 2).unwrap();

    // Make the first capture
    game.make_move(3, 4).unwrap();

    // Now the piece should be at (3, 4) and must continue capturing
    assert_eq!(game.selected_piece, Some((3, 4)));
    assert!(game.possible_moves.is_some());

    // Check that we're in a multi-capture state
    assert!(game.is_in_multi_capture());

    // In this state, deselection should NOT be allowed
    // The main.rs code checks is_in_multi_capture() before allowing deselection
}

#[test]
fn test_is_in_multi_capture_returns_false_for_normal_moves() {
    let mut game = CheckersGame::new();

    // Select a piece with only normal moves
    game.select_piece(5, 0).unwrap();

    // Should not be in multi-capture state
    assert!(!game.is_in_multi_capture());
}

#[test]
fn test_is_in_multi_capture_returns_true_when_only_captures_available() {
    let mut game = CheckersGame::new();
    game.board = Board::new(8);

    // Set up where piece has ONLY capture moves
    game.board.set_piece(4, 2, Some(Piece::new(Color::White)));
    game.board.set_piece(3, 3, Some(Piece::new(Color::Black)));
    game.board.set_piece(3, 1, Some(Piece::new(Color::Black)));

    game.current_player = Color::White;
    game.select_piece(4, 2).unwrap();

    // Check that all possible moves are captures
    if let Some(moves) = &game.possible_moves {
        assert!(!moves.is_empty());
        // All moves should be captures (distance of 2)
        for (to_row, to_col) in moves {
            let row_diff = (*to_row as i32 - 4).abs();
            let col_diff = (*to_col as i32 - 2).abs();
            assert_eq!(row_diff, 2);
            assert_eq!(col_diff, 2);
        }
    }

    assert!(game.is_in_multi_capture());
}

#[test]
fn test_deselection_with_mixed_moves_not_multi_capture() {
    let mut game = CheckersGame::new();
    game.board = Board::new(8);

    // Regular piece with normal move available
    game.board.set_piece(5, 2, Some(Piece::new(Color::White)));

    // Ensure there are no captures available from this position
    game.current_player = Color::White;
    game.select_piece(5, 2).unwrap();

    // Check that piece has normal moves (not captures)
    if let Some(moves) = &game.possible_moves {
        // Should have normal forward moves
        assert!(moves.contains(&(4, 1)) || moves.contains(&(4, 3)));

        // All moves should be normal (distance of 1)
        for (to_row, to_col) in moves {
            let row_diff = (*to_row as i32 - 5).abs();
            let col_diff = (*to_col as i32 - 2).abs();
            assert_eq!(row_diff, 1);
            assert_eq!(col_diff, 1);
        }
    }

    // Should NOT be in multi-capture because has only normal moves
    assert!(!game.is_in_multi_capture());
}

#[test]
fn test_forced_capture_prevents_selecting_non_capturing_piece() {
    let mut game = CheckersGame::new();
    game.board = Board::new(8);

    // White piece that can capture
    game.board.set_piece(4, 2, Some(Piece::new(Color::White)));
    game.board.set_piece(3, 3, Some(Piece::new(Color::Black)));

    // White piece that cannot capture
    game.board.set_piece(5, 0, Some(Piece::new(Color::White)));

    game.current_player = Color::White;

    // Should not be able to select the non-capturing piece
    assert!(game.select_piece(5, 0).is_err());

    // Should be able to select the capturing piece
    assert!(game.select_piece(4, 2).is_ok());
}

#[test]
fn test_deselection_resets_completely() {
    let mut game = CheckersGame::new();

    // Select a piece
    game.select_piece(5, 0).unwrap();
    let initial_moves = game.possible_moves.clone();
    assert!(initial_moves.is_some());

    // Deselect
    game.selected_piece = None;
    game.possible_moves = None;

    // Select a different piece
    game.select_piece(5, 2).unwrap();

    // Should have different possible moves
    assert!(game.possible_moves.is_some());
    assert_ne!(game.possible_moves, initial_moves);
}

#[test]
fn test_multi_capture_sequence_maintains_selection() {
    let mut game = CheckersGame::new();
    game.board = Board::new(8);

    // Set up a double capture for white
    // White at (5, 2) can capture black at (4, 3) landing at (3, 4)
    // Then from (3, 4) can capture black at (2, 5) landing at (1, 6)
    game.board.set_piece(5, 2, Some(Piece::new(Color::White)));
    game.board.set_piece(4, 3, Some(Piece::new(Color::Black)));
    game.board.set_piece(2, 5, Some(Piece::new(Color::Black)));

    game.current_player = Color::White;

    // Select and make first capture
    game.select_piece(5, 2).unwrap();
    game.make_move(3, 4).unwrap();

    // Should still be selected for continuation
    assert_eq!(game.selected_piece, Some((3, 4)));
    assert!(game.possible_moves.is_some());

    // Verify we're in multi-capture (only capture moves available)
    if let Some(moves) = &game.possible_moves {
        assert!(!moves.is_empty());
        // Should only have the capture to (1, 6)
        assert!(moves.contains(&(1, 6)));
        assert!(game.is_in_multi_capture());
    }

    // Make final capture
    game.make_move(1, 6).unwrap();

    // Now should be deselected (no more captures)
    assert_eq!(game.selected_piece, None);
    assert_eq!(game.possible_moves, None);
}
