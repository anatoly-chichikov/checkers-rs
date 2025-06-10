//! Tests for [`GameSession::try_multicapture_move`] and related state-management helpers.

use checkers_rs::core::board::Board;
use checkers_rs::core::piece::{Color, Piece};
use checkers_rs::state::GameSession;

// Helper that returns a fresh empty 8×8 board.
fn empty_board() -> Board {
    Board::new(8)
}

// Returns a GameSession whose board we can freely mutate before running tests.
fn blank_session() -> GameSession {
    let mut session = GameSession::new();
    session.game.board = empty_board();
    session
}

#[test]
fn test_try_multicapture_performs_double_jump_and_finishes() {
    // Board layout (indexes):
    // 7 . . . . . . . .
    // 6 . W . . . . . .   (white starts at 5,0 in algebraic -> (5,0))
    // 5 B . . . . . . .
    // 4 . . W . . . . .
    // 3 . . . B . . . .
    // 2 . . . . . . . .
    // etc.

    let mut session = blank_session();

    // Place the attacking white piece at (5,0).
    session
        .game
        .board
        .set_piece(5, 0, Some(Piece::new(Color::White)));

    // Black victims enabling a double capture: (4,1) and (2,3)
    session
        .game
        .board
        .set_piece(4, 1, Some(Piece::new(Color::Black)));
    session
        .game
        .board
        .set_piece(2, 3, Some(Piece::new(Color::Black)));

    session.game.current_player = Color::White;

    // Select the white piece so UI state is primed.
    let session = session.select_piece(5, 0).unwrap();

    // Perform the multi-capture jump to the final square (1,4).
    let (new_session, continue_capture, path) = session
        .try_multicapture_move(1, 4)
        .expect("move should succeed");

    // The move consisted of two steps → (3,2) then (1,4).
    assert_eq!(path, vec![(3, 2), (1, 4)]);

    // No further captures are available from (1,4), so the sequence is finished.
    assert!(!continue_capture);

    // UI selection cleared.
    assert!(new_session.ui_state.selected_piece.is_none());
    assert!(new_session.ui_state.possible_moves.is_empty());

    // Board assertions: the white piece is at its final square, black pieces removed.
    assert!(new_session.game.board.get_piece(1, 4).is_some());
    assert!(new_session.game.board.get_piece(4, 1).is_none());
    assert!(new_session.game.board.get_piece(2, 3).is_none());
}

#[test]
fn test_try_multicapture_continues_when_more_captures_exist() {
    // White begins at (6,1) and can capture (5,2)->(4,3), then (3,4)->(2,5),
    // and finally (1,6)->(0,7). We will execute the first two jumps by
    // landing on (0,7), leaving no additional captures (because row 0 is the
    // promotion row).
    let mut session = blank_session();

    session
        .game
        .board
        .set_piece(6, 1, Some(Piece::new(Color::White)));
    session
        .game
        .board
        .set_piece(5, 2, Some(Piece::new(Color::Black)));
    session
        .game
        .board
        .set_piece(3, 4, Some(Piece::new(Color::Black)));
    session
        .game
        .board
        .set_piece(1, 6, Some(Piece::new(Color::Black)));

    session.game.current_player = Color::White;

    let session = session.select_piece(6, 1).unwrap();

    // Ensure the engine itself believes a multi-capture path leads to (0,7).
    let maybe_path =
        checkers_rs::core::game_logic::find_capture_path(&session.game.board, 6, 1, 0, 7);
    assert!(
        maybe_path.is_some(),
        "capture path to (2,5) should exist, got None"
    );

    let (new_session, continue_capture, path) = session
        .try_multicapture_move(0, 7)
        .expect("triple jump should succeed");

    // Triple-step path executed.
    assert_eq!(path, vec![(4, 3), (2, 5), (0, 7)]);

    // After landing on the promotion row, no further captures are possible.
    assert!(!continue_capture);

    // UI selection cleared (sequence ended).
    assert!(new_session.ui_state.selected_piece.is_none());

    // Piece has arrived at the final destination.
    assert!(new_session.game.board.get_piece(0, 7).is_some());

    // Captured pieces are gone.
    assert!(new_session.game.board.get_piece(5, 2).is_none());
    assert!(new_session.game.board.get_piece(3, 4).is_none());
    assert!(new_session.game.board.get_piece(1, 6).is_none());
}

#[test]
fn test_try_multicapture_invalid_move_is_rejected() {
    let mut session = blank_session();
    session
        .game
        .board
        .set_piece(5, 0, Some(Piece::new(Color::White)));
    session.game.current_player = Color::White;

    let session = session.select_piece(5, 0).unwrap();

    // Attempt a non-diagonal move which is obviously invalid.
    assert!(session.try_multicapture_move(5, 2).is_err());
}
