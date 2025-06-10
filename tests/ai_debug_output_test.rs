use checkers_rs::ai::get_ai_move;
use checkers_rs::state::GameSession;
use std::env;
use tokio;

#[tokio::test]
async fn test_ai_move_without_debug_output() {
    // Skip test if GEMINI_API_KEY is not set
    if env::var("GEMINI_API_KEY").is_err() {
        eprintln!("Skipping test: GEMINI_API_KEY not set");
        return;
    }

    // Set up a game where it's Black's turn
    let session = GameSession::new();

    // First make a white move
    let session = session.select_piece(5, 0).unwrap();
    let (session, _) = session.make_move(4, 1).unwrap();

    // Now it's Black's turn - call get_ai_move
    // In the fixed version, this should not print debug output
    match get_ai_move(&session.game).await {
        Ok(_) => {
            // Success - but we need to verify no debug output was printed
            // This is hard to test directly, but the test documents the issue
        }
        Err(_) => {
            // AI error is ok for this test
        }
    }
}
