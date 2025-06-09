use checkers_rs::core::piece::{Color, Piece};
use checkers_rs::state::states::PieceSelectedState;
use checkers_rs::state::{GameSession, State, StateTransition};
use crossterm::event::{KeyCode, KeyEvent};

#[test]
fn test_piece_selected_state_makes_a_valid_move() {
    let mut initial_session = GameSession::new();

    initial_session.game.board.cells = vec![vec![None; 8]; 8];
    initial_session.game.board.cells[5][2] = Some(Piece::new(Color::White));
    initial_session.game.board.cells[0][3] = Some(Piece::new(Color::Black));
    initial_session.game.board.cells[0][5] = Some(Piece::new(Color::Black));
    initial_session.ui_state.cursor_pos = (4, 1);
    initial_session.ui_state.selected_piece = Some((5, 2));
    initial_session.ui_state.possible_moves = vec![(4, 1), (4, 3)];

    let state = PieceSelectedState::new((5, 2));

    let (new_session, transition) =
        state.handle_input(&initial_session, KeyEvent::from(KeyCode::Enter));

    match transition {
        StateTransition::To(next_state) => {
            assert_eq!(
                next_state.state_type(),
                checkers_rs::state::StateType::Playing
            );
        }
        _ => panic!("Expected transition to PlayingState"),
    }

    assert!(new_session.game.board.get_piece(5, 2).is_none());
    assert!(new_session.game.board.get_piece(4, 1).is_some());
    assert_eq!(new_session.game.current_player, Color::Black);

    assert!(initial_session.game.board.get_piece(5, 2).is_some());
    assert!(initial_session.game.board.get_piece(4, 1).is_none());
}

#[test]
fn test_piece_selected_state_deselects_piece() {
    let mut initial_session = GameSession::new();
    initial_session.ui_state.cursor_pos = (2, 1);
    initial_session.ui_state.selected_piece = Some((2, 1));

    let state = PieceSelectedState::new((2, 1));

    let (new_session, transition) =
        state.handle_input(&initial_session, KeyEvent::from(KeyCode::Enter));

    match transition {
        StateTransition::To(next_state) => {
            assert_eq!(
                next_state.state_type(),
                checkers_rs::state::StateType::Playing
            );
        }
        _ => panic!("Expected transition to PlayingState"),
    }

    assert_eq!(new_session.ui_state.selected_piece, None);
    assert_eq!(initial_session.ui_state.selected_piece, Some((2, 1)));
}

#[test]
fn test_piece_selected_state_transitions_to_multi_capture() {
    let mut initial_session = GameSession::new();

    initial_session.game.board.cells = vec![vec![None; 8]; 8];
    initial_session.game.board.cells[5][0] = Some(Piece::new(Color::White));
    initial_session.game.board.cells[4][1] = Some(Piece::new(Color::Black));
    initial_session.game.board.cells[2][3] = Some(Piece::new(Color::Black));

    initial_session.ui_state.cursor_pos = (3, 2);
    initial_session.ui_state.selected_piece = Some((5, 0));
    initial_session.ui_state.possible_moves = vec![(3, 2)];

    let state = PieceSelectedState::new((5, 0));

    let (new_session, transition) =
        state.handle_input(&initial_session, KeyEvent::from(KeyCode::Enter));

    match transition {
        StateTransition::To(next_state) => {
            assert_eq!(
                next_state.state_type(),
                checkers_rs::state::StateType::MultiCapture
            );
        }
        _ => panic!("Expected transition to MultiCaptureState"),
    }

    assert!(new_session.game.board.get_piece(3, 2).is_some());
    assert!(new_session.game.board.get_piece(4, 1).is_none());
    assert!(new_session.ui_state.selected_piece.is_some());

    assert!(initial_session.game.board.get_piece(5, 0).is_some());
    assert!(initial_session.game.board.get_piece(4, 1).is_some());
}

#[test]
fn test_piece_selected_state_cursor_movement() {
    let initial_session = GameSession::new();
    let state = PieceSelectedState::new((2, 1));

    let initial_pos = initial_session.ui_state.cursor_pos;

    let (session_after_up, transition) =
        state.handle_input(&initial_session, KeyEvent::from(KeyCode::Up));
    assert_eq!(
        session_after_up.ui_state.cursor_pos,
        (initial_pos.0.saturating_sub(1), initial_pos.1)
    );
    assert_eq!(transition, StateTransition::None);

    let (session_after_right, transition) =
        state.handle_input(&session_after_up, KeyEvent::from(KeyCode::Right));
    assert_eq!(
        session_after_right.ui_state.cursor_pos,
        (initial_pos.0.saturating_sub(1), (initial_pos.1 + 1).min(7))
    );
    assert_eq!(transition, StateTransition::None);
}

#[test]
fn test_piece_selected_state_exits_on_esc() {
    let initial_session = GameSession::new();
    let state = PieceSelectedState::new((2, 1));

    let (new_session, transition) =
        state.handle_input(&initial_session, KeyEvent::from(KeyCode::Esc));

    match transition {
        StateTransition::To(next_state) => {
            assert_eq!(
                next_state.state_type(),
                checkers_rs::state::StateType::Playing
            );
        }
        _ => panic!("Expected transition to PlayingState on ESC"),
    }

    assert_eq!(new_session.ui_state.selected_piece, None);
}

#[test]
fn test_piece_selected_state_rejects_invalid_move() {
    let mut initial_session = GameSession::new();
    initial_session.ui_state.cursor_pos = (5, 5);

    let state = PieceSelectedState::new((2, 1));

    let (new_session, transition) =
        state.handle_input(&initial_session, KeyEvent::from(KeyCode::Enter));

    assert_eq!(transition, StateTransition::None);
    assert_eq!(
        new_session.ui_state.cursor_pos,
        initial_session.ui_state.cursor_pos
    );
    assert_eq!(
        new_session.game.board.cells,
        initial_session.game.board.cells
    );
}

#[test]
fn test_piece_selected_state_view_data() {
    let mut session = GameSession::new();
    session.game.board.cells[2][1] = Some(Piece::new(Color::White));
    session.ui_state.cursor_pos = (3, 0);
    session.ui_state.selected_piece = Some((2, 1));

    let state = PieceSelectedState::new((2, 1));
    let view_data = state.get_view_data(&session);

    assert_eq!(view_data.selected_piece, Some((2, 1)));
    assert_eq!(view_data.cursor_pos, (3, 0));
    assert_eq!(view_data.status_message, "Select a square to move to");
    assert!(!view_data.show_ai_thinking);
}

#[test]
fn test_piece_selected_state_game_over_transition() {
    let mut initial_session = GameSession::new();

    initial_session.game.board.cells = vec![vec![None; 8]; 8];
    let mut king_piece = Piece::new(Color::White);
    king_piece.promote_to_king();
    initial_session.game.board.cells[2][1] = Some(king_piece);
    initial_session.game.board.cells[3][2] = Some(Piece::new(Color::Black));

    initial_session.ui_state.cursor_pos = (4, 3);
    initial_session.ui_state.selected_piece = Some((2, 1));
    initial_session.ui_state.possible_moves = vec![(1, 0), (1, 2), (3, 0), (3, 2), (4, 3)];

    let state = PieceSelectedState::new((2, 1));

    let (new_session, transition) =
        state.handle_input(&initial_session, KeyEvent::from(KeyCode::Enter));

    match &transition {
        StateTransition::To(next_state) => {
            if new_session.game.check_winner().is_some() {
                assert_eq!(
                    next_state.state_type(),
                    checkers_rs::state::StateType::GameOver
                );
            } else {
                assert_eq!(
                    next_state.state_type(),
                    checkers_rs::state::StateType::Playing
                );
            }
        }
        _ => {}
    }

    assert!(new_session.game.board.get_piece(3, 2).is_none());
}
