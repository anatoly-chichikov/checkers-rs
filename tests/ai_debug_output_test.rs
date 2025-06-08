use checkers_rs::ai::get_ai_move;
use checkers_rs::state::GameSession;
use std::env;
use tokio;

#[test]
fn test_ai_debug_messages_should_not_be_in_stderr() {
    // This test verifies that debug messages like "AI raw response"
    // should not be printed to stderr in production code

    // The presence of eprintln! statements in get_ai_move function
    // causes debug output to appear in the game interface

    // We can't easily test stderr output in unit tests,
    // but we can document the expected behavior

    // Expected: No debug output should be printed to stderr
    // Actual: eprintln! statements in genai_client.rs print debug info

    // This test serves as documentation that these debug statements
    // should be removed or conditionally compiled
}

#[tokio::test]
async fn test_ai_move_without_debug_output() {
    // Skip test if GEMINI_API_KEY is not set
    if env::var("GEMINI_API_KEY").is_err() {
        eprintln!("Skipping test: GEMINI_API_KEY not set");
        return;
    }

    // Set up a game where it's Black's turn
    let mut session = GameSession::new();

    // First make a white move
    session.select_piece(5, 0).unwrap();
    session.make_move(4, 1).unwrap();

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

#[test]
fn test_debug_output_should_be_conditionally_compiled() {
    // This test documents that debug output should use conditional compilation
    // For example:
    // #[cfg(debug_assertions)]
    // eprintln!("AI raw response: '{}'", text_response);

    // Or use a debug flag:
    // if std::env::var("DEBUG_AI").is_ok() {
    //     eprintln!("AI raw response: '{}'", text_response);
    // }
}
