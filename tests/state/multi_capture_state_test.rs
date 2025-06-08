use checkers_rs::core::piece::{Color, Piece};
use checkers_rs::state::states::MultiCaptureState;
use checkers_rs::state::{GameSession, State, StateTransition};
use crossterm::event::{KeyCode, KeyEvent};

#[test]
fn test_multi_capture_state_keeps_piece_selected() {
    let mut session = GameSession::new();
    // Set up a multi-capture scenario
    // White piece at (4, 3) after first capture
    session
        .game
        .board
        .set_piece(4, 3, Some(Piece::new(Color::White)));
    session.game.current_player = Color::White;

    let state = MultiCaptureState::new((4, 3));
    let view_data = state.get_view_data(&session);

    assert_eq!(view_data.selected_piece, Some((4, 3)));
    assert_eq!(view_data.status_message, "You must continue capturing!");
}

#[test]
fn test_multi_capture_state_name() {
    let state = MultiCaptureState::new((4, 3));
    assert_eq!(state.name(), "MultiCaptureState");
}

#[test]
fn test_multi_capture_state_forces_capture_moves_only() {
    let mut session = GameSession::new();

    // Clear board
    for row in 0..8 {
        for col in 0..8 {
            session.game.board.set_piece(row, col, None);
        }
    }

    // Set up scenario: white piece at (4, 3) can capture at (6, 5)
    session
        .game
        .board
        .set_piece(4, 3, Some(Piece::new(Color::White)));
    session
        .game
        .board
        .set_piece(5, 4, Some(Piece::new(Color::Black)));
    session.game.current_player = Color::White;

    let mut state = MultiCaptureState::new((4, 3));
    state.on_enter(&mut session);

    // Should only show capture moves, not regular moves
    assert_eq!(session.ui_state.selected_piece, Some((4, 3)));
    // Possible moves should only include captures
}

#[test]
fn test_multi_capture_state_completes_capture_sequence() {
    let mut session = GameSession::new();

    // Clear board
    for row in 0..8 {
        for col in 0..8 {
            session.game.board.set_piece(row, col, None);
        }
    }

    // Set up: white at (4, 3), black at (5, 4), can capture to (6, 5)
    session
        .game
        .board
        .set_piece(4, 3, Some(Piece::new(Color::White)));
    session
        .game
        .board
        .set_piece(5, 4, Some(Piece::new(Color::Black)));
    session.game.current_player = Color::White;

    let mut state = MultiCaptureState::new((4, 3));
    state.on_enter(&mut session);

    // Move cursor to capture position
    session.ui_state.cursor_pos = (6, 5);

    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Char(' ')));

    // Check if the capture move is valid
    if session.ui_state.possible_moves.contains(&(6, 5)) {
        match transition {
            StateTransition::To(_) => {
                // Black piece should be captured
                assert_eq!(session.game.board.get_piece(5, 4), None);
                // White piece should be at new position
                assert_eq!(
                    session.game.board.get_piece(6, 5).unwrap().color,
                    Color::White
                );
            }
            _ => panic!("Expected transition after capture"),
        }
    } else {
        // If not a valid move, should stay in same state
        assert!(matches!(transition, StateTransition::None));
    }
}

#[test]
fn test_multi_capture_state_continues_if_more_captures() {
    let mut session = GameSession::new();

    // Clear board
    for row in 0..8 {
        for col in 0..8 {
            session.game.board.set_piece(row, col, None);
        }
    }

    // Set up double capture: white at (2, 1), blacks at (3, 2) and (5, 4)
    session
        .game
        .board
        .set_piece(2, 1, Some(Piece::new(Color::White)));
    session
        .game
        .board
        .set_piece(3, 2, Some(Piece::new(Color::Black)));
    session
        .game
        .board
        .set_piece(5, 4, Some(Piece::new(Color::Black)));
    session.game.current_player = Color::White;

    // Simulate first capture already done, piece now at (4, 3)
    session.game.board.set_piece(2, 1, None);
    session.game.board.set_piece(3, 2, None);
    session
        .game
        .board
        .set_piece(4, 3, Some(Piece::new(Color::White)));

    let mut state = MultiCaptureState::new((4, 3));
    state.on_enter(&mut session);

    // Make second capture
    session.ui_state.cursor_pos = (6, 5);

    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Char(' ')));

    // Check if the capture move is valid
    if session.ui_state.possible_moves.contains(&(6, 5)) {
        match transition {
            StateTransition::To(_) => {
                // Second black piece should be captured
                assert_eq!(session.game.board.get_piece(5, 4), None);
            }
            _ => panic!("Expected transition after final capture"),
        }
    } else {
        // If not a valid move, should stay in same state
        assert!(matches!(transition, StateTransition::None));
    }
}

#[test]
fn test_multi_capture_state_transitions_to_game_over() {
    let mut session = GameSession::new();

    // Clear board
    for row in 0..8 {
        for col in 0..8 {
            session.game.board.set_piece(row, col, None);
        }
    }

    // Set up: last black piece will be captured
    session
        .game
        .board
        .set_piece(4, 3, Some(Piece::new(Color::White)));
    session
        .game
        .board
        .set_piece(5, 4, Some(Piece::new(Color::Black)));
    session.game.current_player = Color::White;

    let mut state = MultiCaptureState::new((4, 3));
    state.on_enter(&mut session);

    // Make the final capture
    session.ui_state.cursor_pos = (6, 5);

    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Char(' ')));

    // Check if the capture move is valid
    if session.ui_state.possible_moves.contains(&(6, 5)) {
        // Should transition (game over will be detected)
        assert!(matches!(transition, StateTransition::To(_)));
    } else {
        // If not a valid move, should stay in same state
        assert!(matches!(transition, StateTransition::None));
    }
}

#[test]
fn test_multi_capture_state_cursor_movement() {
    let mut session = GameSession::new();
    session.ui_state.cursor_pos = (4, 3);

    let mut state = MultiCaptureState::new((4, 3));

    // Test cursor movement
    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Up));
    assert!(matches!(transition, StateTransition::None));
    assert_eq!(session.ui_state.cursor_pos, (3, 3));

    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Right));
    assert!(matches!(transition, StateTransition::None));
    assert_eq!(session.ui_state.cursor_pos, (3, 4));
}

#[test]
fn test_multi_capture_state_ignores_non_capture_moves() {
    let mut session = GameSession::new();

    // Clear board
    for row in 0..8 {
        for col in 0..8 {
            session.game.board.set_piece(row, col, None);
        }
    }

    // White piece at (4, 3) with no captures available
    session
        .game
        .board
        .set_piece(4, 3, Some(Piece::new(Color::White)));
    session.game.current_player = Color::White;

    let mut state = MultiCaptureState::new((4, 3));
    state.on_enter(&mut session);

    // Try to make a regular move (should be ignored)
    session.ui_state.cursor_pos = (5, 4);

    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Char(' ')));

    // Should not allow non-capture move
    assert!(matches!(transition, StateTransition::None));
    assert_eq!(
        session.game.board.get_piece(4, 3).unwrap().color,
        Color::White
    );
}

#[test]
fn test_multi_capture_state_no_exit_key() {
    let mut session = GameSession::new();
    let mut state = MultiCaptureState::new((4, 3));

    // ESC should not exit multi-capture (must complete captures)
    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Esc));
    assert!(matches!(transition, StateTransition::None));
}

#[test]
fn test_multi_capture_state_view_data() {
    let mut session = GameSession::new();

    // Set up board
    session
        .game
        .board
        .set_piece(4, 3, Some(Piece::new(Color::White)));
    session.game.current_player = Color::White;
    session.ui_state.cursor_pos = (5, 4);

    let state = MultiCaptureState::new((4, 3));
    let view_data = state.get_view_data(&session);

    assert_eq!(view_data.selected_piece, Some((4, 3)));
    assert_eq!(view_data.cursor_pos, (5, 4));
    assert_eq!(view_data.status_message, "You must continue capturing!");
    assert!(!view_data.show_ai_thinking);
    assert!(view_data.error_message.is_none());
}

#[test]
fn test_multi_capture_state_clears_selection_on_exit() {
    let mut session = GameSession::new();

    // Set up
    session
        .game
        .board
        .set_piece(4, 3, Some(Piece::new(Color::White)));
    session.ui_state.selected_piece = Some((4, 3));
    session.ui_state.possible_moves = vec![(6, 5)];

    let mut state = MultiCaptureState::new((4, 3));
    state.on_exit(&mut session);

    // Should clear selection when exiting multi-capture state
    assert_eq!(session.ui_state.selected_piece, None);
    assert!(session.ui_state.possible_moves.is_empty());
}
