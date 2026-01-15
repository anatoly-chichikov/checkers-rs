use checkers_rs::core::board::{BoardSeed, Grid, Seed as BoardSeedTrait};
use checkers_rs::core::piece::{Color, Piece};
use checkers_rs::core::Position;

#[test]
fn test_new_board() {
    let board = BoardSeed { size: 8 }.make();
    assert!(
        board.edge() == 8
            && board.bounds(Position { row: 7, col: 7 })
            && !board.bounds(Position { row: 8, col: 0 }),
        "board bounds did not match the size"
    );
}

#[test]
fn test_board_initialization() {
    let mut board = BoardSeed { size: 8 }.make();
    board.seed();
    let mut flag = true;
    for row in 0..3 {
        for col in 0..8 {
            if (row + col) % 2 == 1 {
                let piece = board.piece(Position { row, col });
                flag = flag
                    && matches!(
                        piece,
                        Some(Piece {
                            color: Color::Black,
                            king: false
                        })
                    );
            }
        }
    }
    for row in 3..5 {
        for col in 0..8 {
            flag = flag && board.piece(Position { row, col }).is_none();
        }
    }
    for row in 5..8 {
        for col in 0..8 {
            if (row + col) % 2 == 1 {
                let piece = board.piece(Position { row, col });
                flag = flag
                    && matches!(
                        piece,
                        Some(Piece {
                            color: Color::White,
                            king: false
                        })
                    );
            }
        }
    }
    assert!(flag, "board seed did not place expected pieces");
}

#[test]
fn test_get_set_piece() {
    let mut board = BoardSeed { size: 8 }.make();
    let piece = Piece {
        color: Color::White,
        king: false,
    };
    let place = board.place(Position { row: 3, col: 3 }, Some(piece));
    let state = board.piece(Position { row: 3, col: 3 });
    let clearance = board.place(Position { row: 3, col: 3 }, None);
    let end = board.piece(Position { row: 3, col: 3 });
    assert!(
        place && state == Some(piece) && clearance && end.is_none(),
        "piece placement did not round trip"
    );
}

#[test]
fn test_out_of_bounds() {
    let board = BoardSeed { size: 8 }.make();
    assert!(
        !board.bounds(Position { row: 8, col: 0 })
            && !board.bounds(Position { row: 0, col: 8 })
            && !board.bounds(Position { row: 8, col: 8 })
            && board.bounds(Position { row: 7, col: 7 })
            && board.bounds(Position { row: 0, col: 0 }),
        "bounds check did not match expected limits"
    );
}

#[test]
fn test_move_piece() {
    let mut board = BoardSeed { size: 8 }.make();
    let piece = Piece {
        color: Color::White,
        king: false,
    };
    board.place(Position { row: 3, col: 3 }, Some(piece));
    let shift = board.shift(Position { row: 3, col: 3 }, Position { row: 4, col: 4 });
    assert!(
        shift
            && board.piece(Position { row: 4, col: 4 }) == Some(piece)
            && board.piece(Position { row: 3, col: 3 }).is_none(),
        "move did not relocate the piece"
    );
}

#[test]
fn test_invalid_moves() {
    let mut board = BoardSeed { size: 8 }.make();
    let piece = Piece {
        color: Color::White,
        king: false,
    };
    board.place(Position { row: 3, col: 3 }, Some(piece));
    let start = board.shift(Position { row: 3, col: 3 }, Position { row: 8, col: 8 });
    let state = board.piece(Position { row: 3, col: 3 });
    let middle = board.shift(Position { row: 0, col: 0 }, Position { row: 1, col: 1 });
    let end = board.shift(Position { row: 8, col: 8 }, Position { row: 3, col: 3 });
    assert!(
        !start && state == Some(piece) && !middle && !end,
        "invalid moves were not rejected"
    );
}
