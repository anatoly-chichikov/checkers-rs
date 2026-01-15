use checkers_rs::core::piece::{Color, Face, Piece, Side};

#[test]
fn test_piece_attributes() {
    let white = Piece {
        color: Color::White,
        king: false,
    };
    let black = Piece {
        color: Color::Black,
        king: false,
    };
    assert!(
        white.color == Color::White && !white.king && black.color == Color::Black && !black.king,
        "piece attributes were incorrect"
    );
}

#[test]
fn test_piece_symbol() {
    let white = Piece {
        color: Color::White,
        king: false,
    };
    let crown = Piece {
        color: Color::White,
        king: true,
    };
    let black = Piece {
        color: Color::Black,
        king: false,
    };
    let king = Piece {
        color: Color::Black,
        king: true,
    };
    assert!(
        white.symbol() == "(w)".to_string()
            && crown.symbol() == "[W]".to_string()
            && black.symbol() == "(b)".to_string()
            && king.symbol() == "[B]".to_string(),
        "piece symbols were incorrect"
    );
}

#[test]
fn test_color_opponent() {
    assert!(
        Color::White.opponent() == Color::Black && Color::Black.opponent() == Color::White,
        "color opponent did not match"
    );
}
