use checkers_rs::core::board::Board;
use checkers_rs::core::piece::Color;

#[test]
fn test_board_has_pieces_after_initialization() {
    let mut board = Board::new(8);
    board.initialize();
    
    let mut piece_count = 0;
    let mut black_count = 0;
    let mut white_count = 0;
    
    for row in 0..8 {
        for col in 0..8 {
            if let Some(piece) = board.get_piece(row, col) {
                piece_count += 1;
                match piece.color {
                    Color::Black => black_count += 1,
                    Color::White => white_count += 1,
                }
                println!("Piece at ({}, {}): {:?}", row, col, piece);
            }
        }
    }
    
    println!("Total pieces: {}, Black: {}, White: {}", piece_count, black_count, white_count);
    
    assert_eq!(piece_count, 24, "Should have 24 pieces total");
    assert_eq!(black_count, 12, "Should have 12 black pieces");
    assert_eq!(white_count, 12, "Should have 12 white pieces");
}