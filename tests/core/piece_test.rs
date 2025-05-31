use checkers_rs::core::piece::{Color, Piece};

#[test]
fn test_new_piece() {
    let white_piece = Piece::new(Color::White);
    assert_eq!(white_piece.color, Color::White);
    assert!(!white_piece.is_king);

    let black_piece = Piece::new(Color::Black);
    assert_eq!(black_piece.color, Color::Black);
    assert!(!black_piece.is_king);
}

#[test]
fn test_promote_to_king() {
    let mut piece = Piece::new(Color::White);
    assert!(!piece.is_king);
    piece.promote_to_king();
    assert!(piece.is_king);
}

#[test]
fn test_piece_display() {
    let mut white_piece = Piece::new(Color::White);
    assert_eq!(white_piece.display(), "(w)".to_string());
    white_piece.promote_to_king();
    assert_eq!(white_piece.display(), "[W]".to_string());

    let mut black_piece = Piece::new(Color::Black);
    assert_eq!(black_piece.display(), "(b)".to_string());
    black_piece.promote_to_king();
    assert_eq!(black_piece.display(), "[B]".to_string());
}

#[test]
fn test_color_opposite() {
    assert_eq!(Color::White.opposite(), Color::Black);
    assert_eq!(Color::Black.opposite(), Color::White);
}
