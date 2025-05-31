#[test]
fn test_parse_ai_move_string() {
    let possible_moves: Vec<((usize, usize), (usize, usize), bool)> = vec![
        ((2, 2), (3, 3), false),
        ((4, 4), (5, 5), false),
        ((6, 6), (7, 7), true),
    ];

    let test_cases = vec![
        ("1", Some((possible_moves[0].0, possible_moves[0].1)), "Valid: Basic number"),
        (" 2 ", Some((possible_moves[1].0, possible_moves[1].1)), "Valid: Number with whitespace"),
        ("3.", Some((possible_moves[2].0, possible_moves[2].1)), "Valid: Number with trailing period (AI might add this)"),
        ("  1  ", Some((possible_moves[0].0, possible_moves[0].1)), "Valid: Number with extensive whitespace"),
        ("0", None, "Invalid: Zero index"),
        ("4", None, "Invalid: Out of bounds (too high)"),
        ("99", None, "Invalid: Way out of bounds"),
        ("hello", None, "Invalid: Non-numeric response"),
        ("1.5", None, "Invalid: Non-integer number"),
        ("", None, "Invalid: Empty string"),
        ("  ", None, "Invalid: Whitespace only string"),
    ];

    for (ai_response, expected_move_tuple, description) in test_cases {
        let text_response = ai_response.trim();
        let parsed_move_index: Option<usize> = match text_response.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<usize>() {
            Ok(move_number) if move_number > 0 && move_number <= possible_moves.len() => {
                Some(move_number - 1)
            }
            _ => None,
        };

        if let Some(expected_coords) = expected_move_tuple {
            assert!(parsed_move_index.is_some(), "Expected a valid index for '{}', but got None. Case: {}", ai_response, description);
            let index = parsed_move_index.unwrap();
            let chosen_move_data = &possible_moves[index];
            let actual_coords = (chosen_move_data.0, chosen_move_data.1);
            assert_eq!(actual_coords, expected_coords, "Mismatch for '{}'. Expected {:?}, got {:?}. Case: {}", ai_response, expected_coords, actual_coords, description);
        } else {
            assert!(parsed_move_index.is_none(), "Expected None (invalid index) for '{}', but got Some({:?}). Case: {}", ai_response, parsed_move_index, description);
        }
    }
}
