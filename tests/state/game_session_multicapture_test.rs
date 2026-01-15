use checkers_rs::core::board::{BoardSeed, Grid, Seed as BoardSeedTrait};
use checkers_rs::core::game::{Game, GameSeed, Seed as GameSeedTrait};
use checkers_rs::core::piece::{Color, Piece};
use checkers_rs::core::Position;
use checkers_rs::state::GameSession;

#[test]
fn test_try_multicapture_performs_double_jump_and_finishes() {
    let mut session = GameSession::new();
    let mut board = BoardSeed { size: 8 }.make();
    board.place(
        Position { row: 5, col: 0 },
        Some(Piece {
            color: Color::White,
            king: false,
        }),
    );
    board.place(
        Position { row: 4, col: 1 },
        Some(Piece {
            color: Color::Black,
            king: false,
        }),
    );
    board.place(
        Position { row: 2, col: 3 },
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
    let session = session.select_piece(5, 0).unwrap();
    let (result, chain, path) = session
        .try_multicapture_move(1, 4)
        .expect("move should succeed");
    let flag = path == vec![(3, 2), (1, 4)]
        && !chain
        && result.ui_state.selected_piece.is_none()
        && result.ui_state.possible_moves.is_empty()
        && result
            .game
            .board()
            .piece(Position { row: 1, col: 4 })
            .is_some()
        && result
            .game
            .board()
            .piece(Position { row: 4, col: 1 })
            .is_none()
        && result
            .game
            .board()
            .piece(Position { row: 2, col: 3 })
            .is_none();
    assert!(flag, "multi capture did not resolve correctly");
}

#[test]
fn test_try_multicapture_continues_when_more_captures_exist() {
    let mut session = GameSession::new();
    let mut board = BoardSeed { size: 8 }.make();
    board.place(
        Position { row: 6, col: 1 },
        Some(Piece {
            color: Color::White,
            king: false,
        }),
    );
    board.place(
        Position { row: 5, col: 2 },
        Some(Piece {
            color: Color::Black,
            king: false,
        }),
    );
    board.place(
        Position { row: 3, col: 4 },
        Some(Piece {
            color: Color::Black,
            king: false,
        }),
    );
    board.place(
        Position { row: 1, col: 6 },
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
    let session = session.select_piece(6, 1).unwrap();
    let path = session
        .game
        .board()
        .path(Position { row: 6, col: 1 }, Position { row: 0, col: 7 });
    let (result, chain, steps) = session
        .try_multicapture_move(0, 7)
        .expect("triple jump should succeed");
    let flag = path.is_some()
        && steps == vec![(4, 3), (2, 5), (0, 7)]
        && !chain
        && result.ui_state.selected_piece.is_none()
        && result
            .game
            .board()
            .piece(Position { row: 0, col: 7 })
            .is_some()
        && result
            .game
            .board()
            .piece(Position { row: 5, col: 2 })
            .is_none()
        && result
            .game
            .board()
            .piece(Position { row: 3, col: 4 })
            .is_none()
        && result
            .game
            .board()
            .piece(Position { row: 1, col: 6 })
            .is_none();
    assert!(flag, "multi capture did not follow expected path");
}

#[test]
fn test_try_multicapture_invalid_move_is_rejected() {
    let mut session = GameSession::new();
    let mut board = BoardSeed { size: 8 }.make();
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
    let session = session.select_piece(5, 0).unwrap();
    assert!(
        session.try_multicapture_move(5, 2).is_err(),
        "invalid multicapture move was accepted"
    );
}
