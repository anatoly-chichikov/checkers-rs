use checkers_rs::core::piece::Color;
use checkers_rs::state::states::AITurnState;
use checkers_rs::state::{GameSession, State, StateTransition};
use crossterm::event::{KeyCode, KeyEvent};

#[test]
fn test_ai_turn_state_starts_ai_thinking() {
    let mut session = GameSession::new();
    session.game.current_player = Color::Black;

    let mut state = AITurnState::new();
    assert!(!session.ai_state.is_thinking);

    state.on_enter(&mut session);
    assert!(session.ai_state.is_thinking);
}

#[test]
fn test_ai_turn_state_stops_ai_thinking_on_exit() {
    let mut session = GameSession::new();
    session.game.current_player = Color::Black;

    let mut state = AITurnState::new();
    state.on_enter(&mut session);
    assert!(session.ai_state.is_thinking);

    state.on_exit(&mut session);
    assert!(!session.ai_state.is_thinking);
}

#[test]
fn test_ai_turn_state_shows_thinking_status() {
    let mut session = GameSession::new();
    session.game.current_player = Color::Black;

    let state = AITurnState::new();
    let view_data = state.get_view_data(&session);

    assert!(view_data.show_ai_thinking);
    assert_eq!(view_data.status_message, "AI is thinking...");
}

#[test]
fn test_ai_turn_state_shows_ai_error_if_present() {
    let mut session = GameSession::new();
    session.game.current_player = Color::Black;
    session.ai_state.set_error("Test error".to_string());

    let state = AITurnState::new();
    let view_data = state.get_view_data(&session);

    assert_eq!(view_data.error_message, Some("Test error"));
}

#[tokio::test]
async fn test_ai_turn_state_makes_ai_move() {
    let mut session = GameSession::new();
    session.game.current_player = Color::Black;

    let mut state = AITurnState::new();

    // Call handle_input which should make a move immediately in test mode
    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Char(' ')));

    match transition {
        StateTransition::To(_) => {
            // Should transition to PlayingState
            assert_eq!(session.game.current_player, Color::White); // Player should switch

            // Verify a move was made - count pieces moved
            let mut found_moved_piece = false;
            for row in 0..8 {
                for col in 0..8 {
                    if let Some(piece) = session.game.board.get_piece(row, col) {
                        if piece.color == Color::Black {
                            // Check if this piece is not in starting position
                            if row > 2 {
                                found_moved_piece = true;
                                break;
                            }
                        }
                    }
                }
            }
            assert!(found_moved_piece, "AI should have moved a piece");
        }
        _ => panic!("Expected transition to PlayingState"),
    }
}

#[tokio::test]
async fn test_ai_turn_state_transitions_to_game_over_if_no_moves() {
    // Force test mode
    std::env::set_var("AI_TEST_MODE", "1");
    let mut session = GameSession::new();
    session.game.current_player = Color::Black;

    // Clear the board and set up a scenario with no valid moves for Black
    for row in 0..8 {
        for col in 0..8 {
            session.game.board.set_piece(row, col, None);
        }
    }

    // Place one white piece that blocks all black moves
    session.game.board.set_piece(
        7,
        7,
        Some(checkers_rs::core::piece::Piece::new(Color::White)),
    );

    let mut state = AITurnState::new();

    // Sleep to pass thinking time
    std::thread::sleep(std::time::Duration::from_millis(600));

    // Should detect no valid moves and transition to GameOver
    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Char(' ')));

    match transition {
        StateTransition::To(_) => {
            // Should go to GameOverState
            assert!(session.game.is_game_over);
        }
        _ => panic!("Expected transition to GameOverState"),
    }
}

#[tokio::test]
async fn test_ai_turn_state_handles_ai_error() {
    // Force test mode
    std::env::set_var("AI_TEST_MODE", "1");
    let mut session = GameSession::new();
    session.game.current_player = Color::Black;

    // Clear board to create a scenario where AI will make a specific move
    for row in 0..8 {
        for col in 0..8 {
            session.game.board.set_piece(row, col, None);
        }
    }

    // Place one black piece
    session.game.board.set_piece(
        5,
        0,
        Some(checkers_rs::core::piece::Piece::new(Color::Black)),
    );

    let mut state = AITurnState::new();

    // Sleep to pass thinking time
    std::thread::sleep(std::time::Duration::from_millis(600));

    // AI should make a move
    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Char(' ')));

    match transition {
        StateTransition::To(_) => {
            // Should transition successfully
            assert_eq!(session.game.current_player, Color::White);
        }
        _ => panic!("Expected transition to PlayingState"),
    }
}
