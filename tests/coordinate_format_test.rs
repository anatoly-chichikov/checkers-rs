use checkers_rs::ai::formatting::format_square;

// Parses a square notation (e.g., "A1", "H8") into internal (row, col) coordinates.
// This is the inverse of ai::formatting::format_square.
// "A1" corresponds to (7,0), "H8" to (0,7).
fn parse_square(square: &str) -> Option<(usize, usize)> {
    let chars: Vec<char> = square.chars().collect();
    if chars.len() != 2 {
        return None;
    }

    let col = (chars[0] as u8).wrapping_sub(b'A') as usize;
    let display_row_num = chars[1].to_digit(10)? as usize;

    if col < 8 && display_row_num >= 1 && display_row_num <= 8 {
        let row = 8 - display_row_num; // Convert display row (1-8) to internal row (7-0)
        Some((row, col))
    } else {
        None
    }
}

#[test]
fn test_format_square_corners() {
    // Test all corners of the board using checkers_rs::ai::formatting::format_square
    // Top-left (row 0, col 0) should be A8 (display row 8)
    assert_eq!(format_square(0, 0), "A8");
    // Top-right (row 0, col 7) should be H8 (display row 8)
    assert_eq!(format_square(0, 7), "H8");
    // Bottom-left (row 7, col 0) should be A1 (display row 1)
    assert_eq!(format_square(7, 0), "A1");
    // Bottom-right (row 7, col 7) should be H1 (display row 1)
    assert_eq!(format_square(7, 7), "H1");
}

#[test]
fn test_format_square_middle() {
    // Middle of board (row 3, col 3) -> D5 (display row 8-3=5)
    assert_eq!(format_square(3, 3), "D5");
    // (row 3, col 7) -> H5 (display row 8-3=5)
    assert_eq!(format_square(3, 7), "H5");
    // (row 5, col 5) -> F3 (display row 8-5=3)
    assert_eq!(format_square(5, 5), "F3");
}

#[test]
fn test_parse_square() {
    assert_eq!(parse_square("A1"), Some((7, 0))); // Display A1 is internal (7,0)
    assert_eq!(parse_square("H8"), Some((0, 7))); // Display H8 is internal (0,7)
    assert_eq!(parse_square("D4"), Some((4, 3))); // Display D4 is internal (4,3)
    assert_eq!(parse_square("H4"), Some((4, 7))); // Display H4 is internal (4,7)
    assert_eq!(parse_square("F6"), Some((2, 5))); // Display F6 is internal (2,5)
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
