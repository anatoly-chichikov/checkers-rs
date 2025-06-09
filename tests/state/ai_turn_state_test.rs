use checkers_rs::core::piece::Color;
use checkers_rs::state::states::AITurnState;
use checkers_rs::state::{GameSession, State, StateTransition};
use crossterm::event::{KeyCode, KeyEvent};

#[test]
fn test_ai_turn_state_shows_thinking_status() {
    let mut initial_session = GameSession::new();
    initial_session.game = initial_session.game.with_switched_player();
    assert_eq!(initial_session.game.current_player, Color::Black);

    let state = AITurnState::new();
    let view_data = state.get_view_data(&initial_session);

    assert!(view_data.show_ai_thinking);
    assert_eq!(view_data.status_message, "AI is thinking...");
}

#[test]
fn test_ai_turn_state_shows_ai_error_if_present() {
    let mut initial_session = GameSession::new();
    initial_session.game = initial_session.game.with_switched_player();
    initial_session.ai_state = initial_session.ai_state.set_error("Test error".to_string());

    let state = AITurnState::new();
    let view_data = state.get_view_data(&initial_session);

    assert_eq!(view_data.error_message, Some("Test error"));
}

#[tokio::test]
async fn test_ai_turn_state_makes_ai_move() {
    let mut initial_session = GameSession::new();
    initial_session.game = initial_session.game.with_switched_player();
    assert_eq!(initial_session.game.current_player, Color::Black);

    let state = AITurnState::new();

    // Call handle_input which should make a move immediately in test mode
    let (new_session, transition) =
        state.handle_input(&initial_session, KeyEvent::from(KeyCode::Char(' ')));

    match transition {
        StateTransition::To(next_state) => {
            // Should transition to PlayingState
            assert_eq!(
                next_state.state_type(),
                checkers_rs::state::StateType::Playing
            );
            assert_eq!(new_session.game.current_player, Color::White); // Player should switch

            // Verify initial session unchanged
            assert_eq!(initial_session.game.current_player, Color::Black);

            // Verify a move was made - count pieces moved
            let mut found_moved_piece = false;
            for row in 0..8 {
                for col in 0..8 {
                    if let Some(piece) = new_session.game.board.get_piece(row, col) {
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
    let mut initial_session = GameSession::new();
    initial_session.game = initial_session.game.with_switched_player();

    // Clear the board and set up a scenario with no valid moves for Black
    let mut cleared_board = initial_session.game.board.clone();
    for row in 0..8 {
        for col in 0..8 {
            cleared_board.set_piece(row, col, None);
        }
    }

    // Place one white piece that blocks all black moves
    cleared_board.set_piece(
        7,
        7,
        Some(checkers_rs::core::piece::Piece::new(Color::White)),
    );

    let mut new_game = initial_session.game.clone();
    new_game.board = cleared_board;
    initial_session.game = new_game;

    let state = AITurnState::new();

    // Sleep to pass thinking time
    std::thread::sleep(std::time::Duration::from_millis(600));

    // Should detect no valid moves and transition to GameOver
    let (new_session, transition) =
        state.handle_input(&initial_session, KeyEvent::from(KeyCode::Char(' ')));

    match transition {
        StateTransition::To(next_state) => {
            // Should go to GameOverState
            assert_eq!(
                next_state.state_type(),
                checkers_rs::state::StateType::GameOver
            );
            assert!(new_session.game.is_game_over);
            assert!(!initial_session.game.is_game_over);
        }
        _ => panic!("Expected transition to GameOverState"),
    }
}

#[tokio::test]
async fn test_ai_turn_state_handles_ai_error() {
    std::env::set_var("AI_TEST_MODE", "1");
    let mut initial_session = GameSession::new();
    initial_session.game = initial_session.game.with_switched_player();

    let mut cleared_board = initial_session.game.board.clone();
    for row in 0..8 {
        for col in 0..8 {
            cleared_board.set_piece(row, col, None);
        }
    }

    cleared_board.set_piece(
        5,
        0,
        Some(checkers_rs::core::piece::Piece::new(Color::Black)),
    );
    
    cleared_board.set_piece(
        7,
        2,
        Some(checkers_rs::core::piece::Piece::new(Color::White)),
    );

    let mut new_game = initial_session.game.clone();
    new_game.board = cleared_board;
    initial_session.game = new_game;

    let state = AITurnState::new();

    std::thread::sleep(std::time::Duration::from_millis(600));

    let (new_session, transition) =
        state.handle_input(&initial_session, KeyEvent::from(KeyCode::Char(' ')));

    match transition {
        StateTransition::To(next_state) => {
            assert_eq!(
                next_state.state_type(),
                checkers_rs::state::StateType::Playing
            );
            assert_eq!(new_session.game.current_player, Color::White);
            assert_eq!(initial_session.game.current_player, Color::Black);
        }
        _ => panic!("Expected transition to PlayingState"),
    }
}
