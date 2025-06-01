use checkers_rs::core::{board::Board, piece::Color};

fn format_square(row: usize, col: usize) -> String {
    format!("{}{}", (col as u8 + b'A') as char, row + 1)
}

fn parse_square(square: &str) -> Option<(usize, usize)> {
    let chars: Vec<char> = square.chars().collect();
    if chars.len() != 2 {
        return None;
    }

    let col = (chars[0] as u8).wrapping_sub(b'A') as usize;
    let row = chars[1].to_digit(10)? as usize - 1;

    if col < 8 && row < 8 {
        Some((row, col))
    } else {
        None
    }
}

#[test]
fn test_format_square_corners() {
    // Test all corners of the board
    assert_eq!(format_square(0, 0), "A1"); // Top-left
    assert_eq!(format_square(0, 7), "H1"); // Top-right
    assert_eq!(format_square(7, 0), "A8"); // Bottom-left
    assert_eq!(format_square(7, 7), "H8"); // Bottom-right
}

#[test]
fn test_format_square_middle() {
    assert_eq!(format_square(3, 3), "D4"); // Middle of board
    assert_eq!(format_square(3, 7), "H4"); // The position mentioned in hint
    assert_eq!(format_square(5, 5), "F6"); // The target mentioned in hint
}

#[test]
fn test_parse_square() {
    assert_eq!(parse_square("A1"), Some((0, 0)));
    assert_eq!(parse_square("H8"), Some((7, 7)));
    assert_eq!(parse_square("D4"), Some((3, 3)));
    assert_eq!(parse_square("H4"), Some((3, 7)));
    assert_eq!(parse_square("F6"), Some((5, 5)));
}

#[test]
fn test_format_parse_roundtrip() {
    for row in 0..8 {
        for col in 0..8 {
            let formatted = format_square(row, col);
            let parsed = parse_square(&formatted).unwrap();
            assert_eq!(
                parsed,
                (row, col),
                "Failed roundtrip for ({}, {})",
                row,
                col
            );
        }
    }
}

#[test]
fn test_board_position_h4() {
    // According to the screenshot, h4 should have a white piece
    // Let's verify what h4 means in our coordinate system
    let (row, col) = parse_square("H4").unwrap();
    assert_eq!((row, col), (3, 7)); // Row 4 (index 3), Column H (index 7)

    // Create a board and check if there's a piece at that position
    let mut board = Board::new(8);
    board.initialize();

    // In initial position, row 3 should be empty
    assert_eq!(board.get_piece(row, col), None);

    // But in the game state shown, there should be a white piece there
    // This test documents the expected behavior
}

#[test]
fn test_board_position_f6() {
    // F6 is the suggested move target
    let (row, col) = parse_square("F6").unwrap();
    assert_eq!((row, col), (5, 5)); // Row 6 (index 5), Column F (index 5)

    // In initial position, this should have a white piece
    let mut board = Board::new(8);
    board.initialize();

    // Check initial position
    if let Some(piece) = board.get_piece(row, col) {
        assert_eq!(piece.color, Color::White);
    }
}

#[test]
fn test_hint_coordinates_validity() {
    // This test documents that the hint "Move piece from h4 to f6"
    // should correspond to valid board positions

    let h4 = parse_square("H4").unwrap();
    let f6 = parse_square("F6").unwrap();

    // h4 = (3, 7) = Row 4, Column H
    // f6 = (5, 5) = Row 6, Column F

    // For a white piece moving forward (up the board),
    // this would be moving from row 3 to row 5, which is backwards!
    // White pieces move from higher rows to lower rows (7->0)

    // This suggests the AI might be confused about board orientation
    assert!(h4.0 < f6.0, "H4 to F6 moves down the board, not up!");
}
