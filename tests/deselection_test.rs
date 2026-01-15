use checkers_rs::core::board::{BoardSeed, Grid, Seed as BoardSeedTrait};
use checkers_rs::core::game::{GameSeed, Seed as GameSeedTrait};
use checkers_rs::core::piece::{Color, Piece};
use checkers_rs::core::Position;
use checkers_rs::state::GameSession;
use checkers_rs::test_helpers::MultiCaptureCheck;

#[test]
fn test_deselection_allowed_when_clicking_outside_possible_moves() {
    let session = GameSession::new();
    let session = session.select_piece(5, 0).unwrap();
    let flag = session.ui_state.selected_piece.is_some()
        && !session.ui_state.possible_moves.is_empty()
        && !session.is_in_multi_capture();
    let session = session.select_piece(5, 0).unwrap();
    assert!(
        flag && session.ui_state.selected_piece.is_none()
            && session.ui_state.possible_moves.is_empty(),
        "deselection did not clear selection"
    );
}

#[test]
fn test_deselection_blocked_during_multi_capture() {
    let mut session = GameSession::new();
    let mut board = BoardSeed { size: 8 }.make();
    board.place(
        Position { row: 5, col: 2 },
        Some(Piece {
            color: Color::White,
            king: false,
        }),
    );
    board.place(
        Position { row: 4, col: 3 },
        Some(Piece {
            color: Color::Black,
            king: false,
        }),
    );
    board.place(
        Position { row: 2, col: 5 },
        Some(Piece {
            color: Color::Black,
            king: false,
        }),
    );
    session.game = GameSeed {
        board,
        turn: Color::White,
    }
    .make();
    let session = session.select_piece(5, 2).unwrap();
    let (session, _) = session.make_move(3, 4).unwrap();
    assert!(
        session.ui_state.selected_piece == Some((3, 4))
            && !session.ui_state.possible_moves.is_empty()
            && session.is_in_multi_capture(),
        "multi capture did not keep selection"
    );
}

#[test]
fn test_is_in_multi_capture_returns_false_for_normal_moves() {
    let session = GameSession::new();
    let session = session.select_piece(5, 0).unwrap();
    assert!(
        !session.is_in_multi_capture(),
        "multi capture was reported for normal moves"
    );
}

#[test]
fn test_is_in_multi_capture_returns_true_when_only_captures_available() {
    let mut session = GameSession::new();
    let mut board = BoardSeed { size: 8 }.make();
    board.place(
        Position { row: 4, col: 2 },
        Some(Piece {
            color: Color::White,
            king: false,
        }),
    );
    board.place(
        Position { row: 3, col: 3 },
        Some(Piece {
            color: Color::Black,
            king: false,
        }),
    );
    board.place(
        Position { row: 3, col: 1 },
        Some(Piece {
            color: Color::Black,
            king: false,
        }),
    );
    session.game = GameSeed {
        board,
        turn: Color::White,
    }
    .make();
    let session = session.select_piece(4, 2).unwrap();
    let moves = &session.ui_state.possible_moves;
    let mut flag = !moves.is_empty();
    for (row, col) in moves {
        let rise = (*row as i32 - 4).abs();
        let run = (*col as i32 - 2).abs();
        flag = flag && rise == 2 && run == 2;
    }
    assert!(
        flag && session.is_in_multi_capture(),
        "multi capture was not detected"
    );
}

#[test]
fn test_deselection_with_mixed_moves_not_multi_capture() {
    let mut session = GameSession::new();
    let mut board = BoardSeed { size: 8 }.make();
    board.place(
        Position { row: 5, col: 2 },
        Some(Piece {
            color: Color::White,
            king: false,
        }),
    );
    session.game = GameSeed {
        board,
        turn: Color::White,
    }
    .make();
    let session = session.select_piece(5, 2).unwrap();
    let moves = &session.ui_state.possible_moves;
    let mut flag = moves.contains(&(4, 1)) || moves.contains(&(4, 3));
    for (row, col) in moves {
        let rise = (*row as i32 - 5).abs();
        let run = (*col as i32 - 2).abs();
        flag = flag && rise == 1 && run == 1;
    }
    assert!(
        flag && !session.is_in_multi_capture(),
        "normal moves were treated as multi capture"
    );
}

#[test]
fn test_forced_capture_prevents_selecting_non_capturing_piece() {
    let mut session = GameSession::new();
    let mut board = BoardSeed { size: 8 }.make();
    board.place(
        Position { row: 4, col: 2 },
        Some(Piece {
            color: Color::White,
            king: false,
        }),
    );
    board.place(
        Position { row: 3, col: 3 },
        Some(Piece {
            color: Color::Black,
            king: false,
        }),
    );
    board.place(
        Position { row: 5, col: 0 },
        Some(Piece {
            color: Color::White,
            king: false,
        }),
    );
    session.game = GameSeed {
        board,
        turn: Color::White,
    }
    .make();
    let miss = session.select_piece(5, 0).is_err();
    let hit = session.select_piece(4, 2).is_ok();
    assert!(miss && hit, "forced capture did not block selection");
}

#[test]
fn test_deselection_resets_completely() {
    let session = GameSession::new();
    let session = session.select_piece(5, 0).unwrap();
    let origin = session.ui_state.possible_moves.clone();
    let flag = !origin.is_empty();
    let session = session.select_piece(5, 0).unwrap();
    let state =
        session.ui_state.selected_piece.is_none() && session.ui_state.possible_moves.is_empty();
    let session = session.select_piece(5, 2).unwrap();
    assert!(
        flag && state
            && !session.ui_state.possible_moves.is_empty()
            && session.ui_state.possible_moves != origin,
        "deselection did not reset moves"
    );
}

#[test]
fn test_multi_capture_sequence_maintains_selection() {
    let mut session = GameSession::new();
    let mut board = BoardSeed { size: 8 }.make();
    board.place(
        Position { row: 5, col: 2 },
        Some(Piece {
            color: Color::White,
            king: false,
        }),
    );
    board.place(
        Position { row: 4, col: 3 },
        Some(Piece {
            color: Color::Black,
            king: false,
        }),
    );
    board.place(
        Position { row: 2, col: 5 },
        Some(Piece {
            color: Color::Black,
            king: false,
        }),
    );
    session.game = GameSeed {
        board,
        turn: Color::White,
    }
    .make();
    let session = session.select_piece(5, 2).unwrap();
    let (session, _) = session.make_move(3, 4).unwrap();
    let moves = session.ui_state.possible_moves.clone();
    let mut flag = session.ui_state.selected_piece == Some((3, 4)) && !moves.is_empty();
    flag = flag && moves.contains(&(1, 6)) && session.is_in_multi_capture();
    let (session, _) = session.make_move(1, 6).unwrap();
    assert!(
        flag && session.ui_state.selected_piece.is_none()
            && session.ui_state.possible_moves.is_empty(),
        "multi capture sequence did not clear selection"
    );
}
