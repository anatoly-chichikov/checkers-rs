use checkers_rs::core::board::{BoardSeed, Grid, Seed as BoardSeedTrait};
use checkers_rs::core::game::{Game, GameError, GameSeed, Seed as GameSeedTrait};
use checkers_rs::core::piece::{Color, Piece};
use checkers_rs::core::{Move, Position};

#[test]
fn test_move_returns_new_game() {
    let mut board = BoardSeed { size: 8 }.make();
    board.seed();
    let game = GameSeed {
        board,
        turn: Color::White,
    }
    .make();
    let origin = Position { row: 5, col: 0 };
    let target = Position { row: 4, col: 1 };
    let piece = game.board().piece(origin);
    let player = game.turn();
    let (result, chain) = game.play(Move { origin, target }).unwrap();
    assert!(
        result.board().piece(target).is_some()
            && result.board().piece(origin).is_none()
            && result.turn() == Color::Black
            && !chain
            && game.board().piece(origin) == piece
            && game.turn() == player,
        "move did not return an updated game"
    );
}

#[test]
fn test_move_rejects_forced_capture() {
    let mut board = BoardSeed { size: 8 }.make();
    board.place(
        Position { row: 4, col: 3 },
        Some(Piece {
            color: Color::White,
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
    let game = GameSeed {
        board,
        turn: Color::White,
    }
    .make();
    let origin = Position { row: 4, col: 3 };
    let target = Position { row: 3, col: 2 };
    let result = game.play(Move { origin, target });
    assert!(
        matches!(result, Err(GameError::ForcedCaptureAvailable)),
        "forced capture was not rejected"
    );
}

#[test]
fn test_move_promotes_king() {
    let mut board = BoardSeed { size: 8 }.make();
    board.place(
        Position { row: 1, col: 0 },
        Some(Piece {
            color: Color::White,
            king: false,
        }),
    );
    let game = GameSeed {
        board,
        turn: Color::White,
    }
    .make();
    let origin = Position { row: 1, col: 0 };
    let target = Position { row: 0, col: 1 };
    let (result, _) = game.play(Move { origin, target }).unwrap();
    assert!(
        matches!(result.board().piece(target), Some(Piece { king: true, .. }))
            && matches!(game.board().piece(origin), Some(Piece { king: false, .. })),
        "promotion did not crown the piece"
    );
}

#[test]
fn test_switch_returns_new_game() {
    let mut board = BoardSeed { size: 8 }.make();
    board.seed();
    let game = GameSeed {
        board,
        turn: Color::White,
    }
    .make();
    let result = game.switch();
    assert!(
        result.turn() == Color::Black && game.turn() == Color::White,
        "turn did not switch"
    );
}

#[test]
fn test_move_capture_is_immutable() {
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
    let game = GameSeed {
        board,
        turn: Color::White,
    }
    .make();
    let origin = Position { row: 5, col: 0 };
    let target = Position { row: 3, col: 2 };
    let piece = game.board().piece(Position { row: 4, col: 1 });
    let (result, chain) = game.play(Move { origin, target }).unwrap();
    assert!(
        result.board().piece(target).is_some()
            && result.board().piece(origin).is_none()
            && result.board().piece(Position { row: 4, col: 1 }).is_none()
            && chain
            && game.board().piece(Position { row: 4, col: 1 }) == piece,
        "capture did not preserve immutability"
    );
}

#[test]
fn test_winner_check_is_immutable() {
    let mut board = BoardSeed { size: 8 }.make();
    board.place(
        Position { row: 0, col: 0 },
        Some(Piece {
            color: Color::White,
            king: true,
        }),
    );
    let game = GameSeed {
        board,
        turn: Color::White,
    }
    .make();
    let winner = game.winner();
    assert!(
        winner == Some(Color::White) && game.board().piece(Position { row: 0, col: 0 }).is_some(),
        "winner check did not preserve board"
    );
}
