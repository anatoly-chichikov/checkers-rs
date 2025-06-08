use checkers_rs::core::piece::Color;
use checkers_rs::state::states::PlayingState;
use checkers_rs::state::{GameSession, State, StateTransition};
use crossterm::event::{KeyCode, KeyEvent};

#[test]
fn test_playing_state_cursor_movement() {
    let mut session = GameSession::new();
    let mut state = PlayingState::new();

    // Test cursor movement
    let initial_pos = session.ui_state.cursor_pos;

    state.handle_input(&mut session, KeyEvent::from(KeyCode::Right));
    assert_eq!(
        session.ui_state.cursor_pos,
        (initial_pos.0, initial_pos.1 + 1)
    );

    state.handle_input(&mut session, KeyEvent::from(KeyCode::Down));
    assert_eq!(
        session.ui_state.cursor_pos,
        (initial_pos.0 + 1, initial_pos.1 + 1)
    );

    state.handle_input(&mut session, KeyEvent::from(KeyCode::Left));
    assert_eq!(
        session.ui_state.cursor_pos,
        (initial_pos.0 + 1, initial_pos.1)
    );

    state.handle_input(&mut session, KeyEvent::from(KeyCode::Up));
    assert_eq!(session.ui_state.cursor_pos, initial_pos);
}

#[test]
fn test_playing_state_piece_selection() {
    let mut session = GameSession::new();
    let mut state = PlayingState::new();

    // Move cursor to white piece at (5, 0)
    session.ui_state.cursor_pos = (5, 0);

    // Select the piece
    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Enter));

    match transition {
        StateTransition::To(_) => {
            // Should transition to PieceSelectedState
        }
        _ => panic!("Expected transition to PieceSelectedState"),
    }
}

#[test]
fn test_playing_state_cannot_select_empty_square() {
    let mut session = GameSession::new();
    let mut state = PlayingState::new();

    // Move cursor to empty square
    session.ui_state.cursor_pos = (3, 3);

    // Try to select
    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Enter));

    match transition {
        StateTransition::None => {
            // Correct - no transition
        }
        _ => panic!("Should not transition when selecting empty square"),
    }
}

#[test]
fn test_playing_state_cannot_select_opponent_piece() {
    let mut session = GameSession::new();
    let mut state = PlayingState::new();

    // Current player is White, move cursor to Black piece
    session.ui_state.cursor_pos = (2, 1);

    // Try to select
    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Enter));

    match transition {
        StateTransition::None => {
            // Correct - no transition
        }
        _ => panic!("Should not transition when selecting opponent's piece"),
    }
}

#[test]
fn test_playing_state_transitions_to_ai_turn() {
    let mut session = GameSession::new();
    let mut state = PlayingState::new();

    // Switch to Black's turn
    session.game.current_player = Color::Black;

    // Any input should trigger AI turn
    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Up));

    match transition {
        StateTransition::To(_) => {
            // Should transition to AITurnState
        }
        _ => panic!("Expected transition to AITurnState"),
    }
}

#[test]
fn test_playing_state_exit_on_quit() {
    let mut session = GameSession::new();
    let mut state = PlayingState::new();

    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Esc));
    assert_eq!(transition, StateTransition::Exit);

    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Char('q')));
    assert_eq!(transition, StateTransition::Exit);
}

#[test]
fn test_playing_state_view_data() {
    let session = GameSession::new();
    let state = PlayingState::new();

    let view_data = state.get_view_data(&session);

    assert!(!view_data.is_game_over);
    assert!(!view_data.show_ai_thinking);
    assert!(view_data.status_message.contains("White's turn"));
    assert_eq!(view_data.current_player, Color::White);
}

#[test]
fn test_playing_state_cursor_bounds() {
    let mut session = GameSession::new();
    let mut state = PlayingState::new();

    // Move cursor to top-left corner
    session.ui_state.cursor_pos = (0, 0);

    // Try to move up and left - should stay at (0, 0)
    state.handle_input(&mut session, KeyEvent::from(KeyCode::Up));
    assert_eq!(session.ui_state.cursor_pos, (0, 0));

    state.handle_input(&mut session, KeyEvent::from(KeyCode::Left));
    assert_eq!(session.ui_state.cursor_pos, (0, 0));

    // Move cursor to bottom-right corner
    session.ui_state.cursor_pos = (7, 7);

    // Try to move down and right - should stay at (7, 7)
    state.handle_input(&mut session, KeyEvent::from(KeyCode::Down));
    assert_eq!(session.ui_state.cursor_pos, (7, 7));

    state.handle_input(&mut session, KeyEvent::from(KeyCode::Right));
    assert_eq!(session.ui_state.cursor_pos, (7, 7));
}
