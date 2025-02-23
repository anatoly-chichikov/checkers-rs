use checkers_rs::board::Board;
use checkers_rs::piece::{Color, Piece};

#[test]
fn test_new_board() {
    let board = Board::new(8);
    assert_eq!(board.size, 8);
    assert_eq!(board.cells.len(), 8);
    assert_eq!(board.cells[0].len(), 8);
}

#[test]
fn test_board_initialization() {
    let mut board = Board::new(8);
    board.initialize();

    // Check black pieces (top of board)
    for row in 0..3 {
        for col in 0..8 {
            if (row + col) % 2 == 1 {
                let piece = board.get_piece(row, col).unwrap();
                assert_eq!(piece.color, Color::Black);
                assert!(!piece.is_king);
            }
        }
    }

    // Check empty middle
    for row in 3..5 {
        for col in 0..8 {
            assert!(board.get_piece(row, col).is_none());
        }
    }

    // Check white pieces (bottom of board)
    for row in 5..8 {
        for col in 0..8 {
            if (row + col) % 2 == 1 {
                let piece = board.get_piece(row, col).unwrap();
                assert_eq!(piece.color, Color::White);
                assert!(!piece.is_king);
            }
        }
    }
}

#[test]
fn test_get_set_piece() {
    let mut board = Board::new(8);
    let piece = Piece::new(Color::White);

    assert!(board.set_piece(3, 3, Some(piece)));
    assert_eq!(board.get_piece(3, 3), Some(piece));

    assert!(board.set_piece(3, 3, None));
    assert_eq!(board.get_piece(3, 3), None);
}

#[test]
fn test_out_of_bounds() {
    let board = Board::new(8);
    assert!(!board.in_bounds(8, 0));
    assert!(!board.in_bounds(0, 8));
    assert!(!board.in_bounds(8, 8));
    assert!(board.in_bounds(7, 7));
    assert!(board.in_bounds(0, 0));
}

#[test]
fn test_move_piece() {
    let mut board = Board::new(8);
    let piece = Piece::new(Color::White);
    board.set_piece(3, 3, Some(piece));

    assert!(board.move_piece((3, 3), (4, 4)));
    assert_eq!(board.get_piece(4, 4), Some(piece));
    assert_eq!(board.get_piece(3, 3), None);
}

#[test]
fn test_should_promote() {
    let board = Board::new(8);
    let white_piece = Piece::new(Color::White);
    let black_piece = Piece::new(Color::Black);

    // White pieces should promote at row 0
    assert!(board.should_promote(&white_piece, 0));
    assert!(!board.should_promote(&white_piece, 1));

    // Black pieces should promote at row 7
    assert!(board.should_promote(&black_piece, 7));
    assert!(!board.should_promote(&black_piece, 6));
}

#[test]
fn test_invalid_moves() {
    let mut board = Board::new(8);
    let piece = Piece::new(Color::White);
    board.set_piece(3, 3, Some(piece));

    // Test moving to out of bounds
    assert!(!board.move_piece((3, 3), (8, 8)));
    assert_eq!(board.get_piece(3, 3), Some(piece));

    // Test moving from empty square
    assert!(!board.move_piece((0, 0), (1, 1)));

    // Test moving from out of bounds
    assert!(!board.move_piece((8, 8), (3, 3)));
} 