use checkers_rs::core::game::CheckersGame;
use checkers_rs::core::piece::Color as PieceColor;

#[test]
fn test_two_player_mode_alternates_turns() {
    let mut game = CheckersGame::new();

    assert_eq!(game.current_player, PieceColor::White);

    // White player makes a move (White is at bottom, rows 5-7)
    game.select_piece(5, 0).unwrap();
    game.make_move(4, 1).unwrap();

    assert_eq!(game.current_player, PieceColor::Black);

    // Black player makes a move (Black is at top, rows 0-2)
    game.select_piece(2, 1).unwrap();
    game.make_move(3, 0).unwrap();

    assert_eq!(game.current_player, PieceColor::White);
}

#[test]
fn test_both_players_can_move_pieces() {
    let mut game = CheckersGame::new();

    // White moves (bottom) - avoiding captures
    game.select_piece(5, 0).unwrap();
    game.make_move(4, 1).unwrap();

    // Black moves (top) - avoiding captures
    game.select_piece(2, 7).unwrap();
    game.make_move(3, 6).unwrap();

    // White moves again
    game.select_piece(5, 2).unwrap();
    game.make_move(4, 3).unwrap();

    // Black moves again - avoiding capture
    game.select_piece(2, 5).unwrap();
    game.make_move(3, 4).unwrap();

    // Verify pieces moved correctly
    assert!(game.board.get_piece(4, 1).is_some());
    assert!(game.board.get_piece(3, 6).is_some());
    assert!(game.board.get_piece(4, 3).is_some());
    assert!(game.board.get_piece(3, 4).is_some());
}

#[test]
fn test_both_players_can_capture() {
    let mut game = CheckersGame::new();

    // Setup: carefully position pieces for capture without triggering forced capture
    // First few moves to position pieces
    game.select_piece(5, 0).unwrap();
    game.make_move(4, 1).unwrap();

    game.select_piece(2, 3).unwrap();
    game.make_move(3, 2).unwrap();

    // Now White must capture
    game.select_piece(4, 1).unwrap();
    game.make_move(2, 3).unwrap();

    // Verify capture
    assert!(game.board.get_piece(3, 2).is_none()); // Black piece removed
    assert!(game.board.get_piece(2, 3).is_some()); // White piece at destination

    // Black's turn - if there's a forced capture, take it
    // Otherwise setup for another capture scenario
    let black_has_capture =
        game.board.get_piece(1, 2).is_some() || game.board.get_piece(1, 4).is_some();

    if black_has_capture {
        // If Black has a forced capture from the previous White capture, handle it
        if game.board.get_piece(1, 2).is_some() {
            game.select_piece(1, 2).unwrap();
            game.make_move(3, 4).unwrap();
        } else if game.board.get_piece(1, 4).is_some() {
            game.select_piece(1, 4).unwrap();
            game.make_move(3, 2).unwrap();
        }
    } else {
        // Otherwise just make a normal move
        game.select_piece(2, 5).unwrap();
        game.make_move(3, 4).unwrap();
    }
}

#[test]
fn test_game_continues_without_ai() {
    let mut game = CheckersGame::new();

    // Make a few predetermined moves that avoid forced captures
    // White move 1
    game.select_piece(5, 0).unwrap();
    game.make_move(4, 1).unwrap();
    assert_eq!(game.current_player, PieceColor::Black);

    // Black move 1
    game.select_piece(2, 7).unwrap();
    game.make_move(3, 6).unwrap();
    assert_eq!(game.current_player, PieceColor::White);

    // White move 2
    game.select_piece(5, 2).unwrap();
    game.make_move(4, 3).unwrap();
    assert_eq!(game.current_player, PieceColor::Black);

    // Black move 2
    game.select_piece(2, 1).unwrap();
    game.make_move(3, 0).unwrap();
    assert_eq!(game.current_player, PieceColor::White);

    // White move 3
    game.select_piece(5, 4).unwrap();
    game.make_move(4, 5).unwrap();
    assert_eq!(game.current_player, PieceColor::Black);
}
