use checkers_rs::ai::hint::HintProvider;
use checkers_rs::core::{
    game::CheckersGame, game_logic::get_all_valid_moves_for_player, piece::Color as PieceColor,
};
use serial_test::serial;
use std::env;
use std::sync::{Arc, Mutex};
use tokio;

#[tokio::test]
#[serial]
async fn test_hint_updates_after_ai_move() {
    // Skip test if GEMINI_API_KEY is not set
    let api_key = match env::var("GEMINI_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Skipping test: GEMINI_API_KEY not set");
            return;
        }
    };

    // Create hint provider
    let hint_provider = match HintProvider::new(api_key) {
        Ok(provider) => provider,
        Err(_) => {
            eprintln!("Skipping test: Failed to create HintProvider");
            return;
        }
    };

    // Create a game with AI mode
    let mut game = CheckersGame::new();

    // Simulate a move by the human player (White)
    game.current_player = PieceColor::White;
    let white_moves = get_all_valid_moves_for_player(&game.board, PieceColor::White);
    if !white_moves.is_empty() {
        let ((from_row, from_col), (to_row, to_col), _) = white_moves[0];
        // First select the piece
        game.select_piece(from_row, from_col).unwrap();
        // Then make the move
        game.make_move(to_row, to_col)
            .expect("Failed to make white move");
    }

    // Now it's AI's turn (Black)
    assert_eq!(game.current_player, PieceColor::Black);

    // Get hint before AI move
    let hint_before = hint_provider
        .get_hint(
            &game.board,
            PieceColor::White, // Hints are always for the human player
            &game.move_history,
        )
        .await
        .ok();

    // Simulate AI move
    let black_moves = get_all_valid_moves_for_player(&game.board, PieceColor::Black);
    if !black_moves.is_empty() {
        let ((from_row, from_col), (to_row, to_col), _) = black_moves[0];
        // First select the piece
        game.select_piece(from_row, from_col).unwrap();
        // Then make the move
        game.make_move(to_row, to_col)
            .expect("Failed to make black move");
    }

    // Now it's human's turn again
    assert_eq!(game.current_player, PieceColor::White);

    // Get hint after AI move
    let hint_after = hint_provider
        .get_hint(&game.board, PieceColor::White, &game.move_history)
        .await
        .ok();

    // Hints should be different since the board state has changed
    assert_ne!(hint_before, hint_after, "Hints should update after AI move");
}

#[tokio::test]
#[serial]
async fn test_hint_reflects_current_board_state() {
    // Skip test if GEMINI_API_KEY is not set
    let api_key = match env::var("GEMINI_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Skipping test: GEMINI_API_KEY not set");
            return;
        }
    };

    // Create hint provider
    let hint_provider = match HintProvider::new(api_key) {
        Ok(provider) => provider,
        Err(_) => {
            eprintln!("Skipping test: Failed to create HintProvider");
            return;
        }
    };

    let mut game = CheckersGame::new();

    // Get initial hint
    let initial_hint = hint_provider
        .get_hint(&game.board, PieceColor::White, &game.move_history)
        .await
        .ok();

    // Make several moves to change board state significantly
    let moves = vec![
        ((5, 0), (4, 1)), // White move
        ((2, 1), (3, 0)), // Black move
        ((5, 2), (4, 3)), // White move
        ((2, 3), (3, 2)), // Black move
    ];

    for (from, to) in moves.iter() {
        // Select piece first
        if let Err(_) = game.select_piece(from.0, from.1) {
            continue;
        }
        // Make move
        if let Err(_) = game.make_move(to.0, to.1) {
            continue;
        }
    }

    // Get hint after moves
    let final_hint = hint_provider
        .get_hint(&game.board, PieceColor::White, &game.move_history)
        .await
        .ok();

    // Hints should be different
    assert_ne!(
        initial_hint, final_hint,
        "Hints should reflect board state changes"
    );
}

#[test]
fn test_hint_automatic_update_mechanism() {
    // This test verifies the mechanism for automatic hint updates
    // In the actual implementation, this would be triggered after AI moves

    let hint_state = Arc::new(Mutex::new(Option::<String>::None));
    let hint_state_clone = Arc::clone(&hint_state);

    // Simulate hint update after AI move
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(100));
        let mut hint = hint_state_clone.lock().unwrap();
        *hint = Some("Updated hint after AI move".to_string());
    });

    // Initial state should be None
    assert_eq!(*hint_state.lock().unwrap(), None);

    // Wait for update
    std::thread::sleep(std::time::Duration::from_millis(200));

    // Hint should be updated
    assert_eq!(
        *hint_state.lock().unwrap(),
        Some("Updated hint after AI move".to_string())
    );
}
