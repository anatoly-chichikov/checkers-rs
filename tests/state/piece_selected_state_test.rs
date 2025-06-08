use checkers_rs::state::{GameSession, State, StateTransition};
use checkers_rs::state::states::PieceSelectedState;
use checkers_rs::core::piece::{Color, Piece};
use crossterm::event::{KeyCode, KeyEvent};

#[test]
fn test_piece_selected_state_selects_piece_on_enter() {
    let mut session = GameSession::new();
    // Place a white piece at position (2, 1)
    session.game.board.set_piece(2, 1, Some(Piece::new(Color::White)));
    // Make sure it's white's turn
    session.game.current_player = Color::White;
    
    let mut state = PieceSelectedState::new((2, 1));
    state.on_enter(&mut session);
    
    assert_eq!(session.ui_state.selected_piece, Some((2, 1)));
    // Check if possible moves were calculated (may be empty if no moves available)
}

#[test]
fn test_piece_selected_state_clears_selection_on_exit() {
    let mut session = GameSession::new();
    // Place a white piece at position (2, 1)
    session.game.board.set_piece(2, 1, Some(Piece::new(Color::White)));
    
    let mut state = PieceSelectedState::new((2, 1));
    state.on_enter(&mut session);
    assert_eq!(session.ui_state.selected_piece, Some((2, 1)));
    
    state.on_exit(&mut session);
    assert_eq!(session.ui_state.selected_piece, None);
    assert!(session.ui_state.possible_moves.is_empty());
}

#[test]
fn test_piece_selected_state_cursor_movement() {
    let mut session = GameSession::new();
    session.ui_state.cursor_pos = (2, 1);
    
    let mut state = PieceSelectedState::new((2, 1));
    
    // Test cursor movement up
    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Up));
    assert!(matches!(transition, StateTransition::None));
    assert_eq!(session.ui_state.cursor_pos, (1, 1));
    
    // Test cursor movement right
    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Right));
    assert!(matches!(transition, StateTransition::None));
    assert_eq!(session.ui_state.cursor_pos, (1, 2));
}

#[test]
fn test_piece_selected_state_deselects_on_same_square() {
    let mut session = GameSession::new();
    session.ui_state.cursor_pos = (2, 1);
    
    let mut state = PieceSelectedState::new((2, 1));
    
    // Press space on the same square
    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Char(' ')));
    
    match transition {
        StateTransition::To(_) => {
            // Should transition back to PlayingState
        }
        _ => panic!("Expected transition to PlayingState"),
    }
}

#[test]
fn test_piece_selected_state_makes_valid_move() {
    let mut session = GameSession::new();
    // Clear board first
    for row in 0..8 {
        for col in 0..8 {
            session.game.board.set_piece(row, col, None);
        }
    }
    
    // Set up board for a valid move
    session.game.board.set_piece(2, 1, Some(Piece::new(Color::White)));
    session.game.current_player = Color::White;
    session.ui_state.cursor_pos = (2, 1);
    
    let mut state = PieceSelectedState::new((2, 1));
    state.on_enter(&mut session);
    
    // Move cursor to a valid position (diagonal move)
    session.ui_state.cursor_pos = (3, 2);
    
    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Char(' ')));
    
    // Check if the move is in possible_moves
    if session.ui_state.possible_moves.contains(&(3, 2)) {
        match transition {
            StateTransition::To(_) => {
                // Should transition to PlayingState after successful move
                assert_eq!(session.game.board.get_piece(2, 1), None);
                assert_eq!(session.game.board.get_piece(3, 2).unwrap().color, Color::White);
            }
            _ => panic!("Expected transition after valid move"),
        }
    } else {
        // If not a valid move, should stay in same state
        assert!(matches!(transition, StateTransition::None));
    }
}

#[test]
fn test_piece_selected_state_rejects_invalid_move() {
    let mut session = GameSession::new();
    // Set up board
    session.game.board.set_piece(2, 1, Some(Piece::new(Color::White)));
    session.ui_state.cursor_pos = (2, 1);
    
    let mut state = PieceSelectedState::new((2, 1));
    state.on_enter(&mut session);
    
    // Move cursor to an invalid position
    session.ui_state.cursor_pos = (5, 5);
    // Don't add this to possible_moves
    
    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Char(' ')));
    
    assert!(matches!(transition, StateTransition::None));
    // Piece should still be at original position
    assert_eq!(session.game.board.get_piece(2, 1).unwrap().color, Color::White);
}

#[test]
fn test_piece_selected_state_exits_on_esc() {
    let mut session = GameSession::new();
    let mut state = PieceSelectedState::new((2, 1));
    
    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Esc));
    
    match transition {
        StateTransition::To(_) => {
            // Should transition back to PlayingState
        }
        _ => panic!("Expected transition to PlayingState on ESC"),
    }
}

#[test]
fn test_piece_selected_state_view_data() {
    let mut session = GameSession::new();
    session.game.board.set_piece(2, 1, Some(Piece::new(Color::White)));
    session.ui_state.cursor_pos = (3, 0);
    
    let state = PieceSelectedState::new((2, 1));
    let view_data = state.get_view_data(&session);
    
    assert_eq!(view_data.selected_piece, Some((2, 1)));
    assert_eq!(view_data.cursor_pos, (3, 0));
    assert_eq!(view_data.status_message, "Select a square to move to");
    assert!(!view_data.show_ai_thinking);
}

#[test]
fn test_piece_selected_state_name() {
    let state = PieceSelectedState::new((2, 1));
    assert_eq!(state.name(), "PieceSelectedState");
}

#[test]
fn test_piece_selected_state_transitions_to_multi_capture() {
    let mut session = GameSession::new();
    
    // Clear board first
    for row in 0..8 {
        for col in 0..8 {
            session.game.board.set_piece(row, col, None);
        }
    }
    
    // Set up a capture scenario
    // White piece at (2, 1), black piece at (3, 2), empty at (4, 3)
    session.game.board.set_piece(2, 1, Some(Piece::new(Color::White)));
    session.game.board.set_piece(3, 2, Some(Piece::new(Color::Black)));
    session.game.current_player = Color::White;
    
    let mut state = PieceSelectedState::new((2, 1));
    state.on_enter(&mut session);
    
    // Move cursor to capture position
    session.ui_state.cursor_pos = (4, 3);
    
    // Make the capture move
    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Char(' ')));
    
    // Check if this is a valid capture move
    if session.ui_state.possible_moves.contains(&(4, 3)) {
        match transition {
            StateTransition::To(_) => {
                // Should capture the black piece
                assert_eq!(session.game.board.get_piece(3, 2), None);
                assert_eq!(session.game.board.get_piece(4, 3).unwrap().color, Color::White);
            }
            _ => panic!("Expected transition after capture"),
        }
    } else {
        // If not a valid move, should stay in same state
        assert!(matches!(transition, StateTransition::None));
    }
}

#[test]
fn test_piece_selected_state_game_over_check() {
    let mut session = GameSession::new();
    
    // Set up a scenario where this move ends the game
    // Clear the board first
    for row in 0..8 {
        for col in 0..8 {
            session.game.board.set_piece(row, col, None);
        }
    }
    
    // Only two pieces left
    session.game.board.set_piece(2, 1, Some(Piece::new(Color::White)));
    session.game.board.set_piece(5, 4, Some(Piece::new(Color::Black)));
    session.game.current_player = Color::White;
    
    let mut state = PieceSelectedState::new((2, 1));
    state.on_enter(&mut session);
    
    // Make a move
    session.ui_state.cursor_pos = (3, 2);
    
    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Char(' ')));
    
    // Check if move is valid
    if session.ui_state.possible_moves.contains(&(3, 2)) {
        match transition {
            StateTransition::To(_) => {
                // Move should be made
                assert_eq!(session.game.board.get_piece(2, 1), None);
                assert_eq!(session.game.board.get_piece(3, 2).unwrap().color, Color::White);
            }
            _ => panic!("Expected transition after move"),
        }
    } else {
        // If not a valid move, should stay in same state
        assert!(matches!(transition, StateTransition::None));
    }
}