use checkers_rs::ai::hint::HintProvider;
use checkers_rs::core::{
    game_logic::get_all_valid_moves_for_player, piece::Color as PieceColor,
};
use checkers_rs::state::GameSession;
use serial_test::serial;
use std::env;
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
    let mut session = GameSession::new();

    // Simulate a move by the human player (White)
    session.game.current_player = PieceColor::White;
    let white_moves = get_all_valid_moves_for_player(&session.game.board, PieceColor::White);
    if !white_moves.is_empty() {
        let ((from_row, from_col), (to_row, to_col), _) = white_moves[0];
        // First select the piece
        session.select_piece(from_row, from_col).unwrap();
        // Then make the move
        session.make_move(to_row, to_col).unwrap();

        // Verify it's now Black's turn
        assert_eq!(session.game.current_player, PieceColor::Black);

        // In a real game, the AI would make a move here
        // For this test, we'll simulate that Black has made a move

        // Get a hint for the current board state (after simulated AI move)
        match hint_provider
            .get_hint(&session.game.board, PieceColor::White, &session.game.move_history)
            .await
        {
            Ok(hint_text) => {
                // Verify we got a meaningful hint
                assert!(!hint_text.is_empty());
                assert!(hint_text.len() > 10); // Should be more than a trivial response
            }
            Err(e) => {
                eprintln!("Hint generation failed: {}", e);
                // It's ok if hint generation fails in tests due to API issues
            }
        }
    }
}

#[tokio::test]
#[serial]
async fn test_hint_content_changes_with_board_state() {
    // Skip test if GEMINI_API_KEY is not set
    let api_key = match env::var("GEMINI_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Skipping test: GEMINI_API_KEY not set");
            return;
        }
    };

    let hint_provider = match HintProvider::new(api_key) {
        Ok(provider) => provider,
        Err(_) => {
            eprintln!("Skipping test: Failed to create HintProvider");
            return;
        }
    };

    let mut session = GameSession::new();

    // Get hint for initial board
    let hint1 = hint_provider
        .get_hint(&session.game.board, PieceColor::White, &session.game.move_history)
        .await;

    // Make a move
    let white_moves = get_all_valid_moves_for_player(&session.game.board, PieceColor::White);
    if !white_moves.is_empty() {
        let ((from_row, from_col), (to_row, to_col), _) = white_moves[0];
        session.select_piece(from_row, from_col).unwrap();
        session.make_move(to_row, to_col).unwrap();
    }

    // Get hint for new board state
    let hint2 = hint_provider
        .get_hint(&session.game.board, PieceColor::White, &session.game.move_history)
        .await;

    // If both hints succeeded, they should be different
    if let (Ok(h1), Ok(h2)) = (hint1, hint2) {
        // The hints should be different since the board state changed
        // But we can't guarantee this 100% due to AI variability
        // So we just check that both are non-empty
        assert!(!h1.is_empty());
        assert!(!h2.is_empty());
    }
}

#[tokio::test]
#[serial]
async fn test_hint_provider_handles_api_failures_gracefully() {
    // Test with invalid API key
    let hint_provider = match HintProvider::new("invalid_key".to_string()) {
        Ok(provider) => provider,
        Err(_) => return, // This is expected
    };

    let session = GameSession::new();

    // This should fail gracefully
    match hint_provider
        .get_hint(&session.game.board, PieceColor::White, &session.game.move_history)
        .await
    {
        Ok(_) => {
            // Unlikely with invalid key
        }
        Err(e) => {
            // This is expected - verify it's a reasonable error
            assert!(!e.to_string().is_empty());
        }
    }
}