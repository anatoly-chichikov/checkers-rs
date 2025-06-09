use checkers_rs::core::piece::Color;
use checkers_rs::state::states::PlayingState;
use checkers_rs::state::{GameSession, State, StateTransition};
use crossterm::event::{KeyCode, KeyEvent};

#[test]
fn test_playing_state_handles_cursor_movement() {
    let initial_session = GameSession::new();
    let state = PlayingState::new();

    let initial_pos = initial_session.ui_state.cursor_pos;

    let (session_after_right, transition) =
        state.handle_input(&initial_session, KeyEvent::from(KeyCode::Right));
    assert_eq!(
        session_after_right.ui_state.cursor_pos,
        (initial_pos.0, initial_pos.1 + 1)
    );
    assert_eq!(transition, StateTransition::None);
    assert_eq!(initial_session.ui_state.cursor_pos, initial_pos);

    let (session_after_down, transition) =
        state.handle_input(&session_after_right, KeyEvent::from(KeyCode::Down));
    assert_eq!(
        session_after_down.ui_state.cursor_pos,
        (initial_pos.0 + 1, initial_pos.1 + 1)
    );
    assert_eq!(transition, StateTransition::None);
    assert_eq!(
        session_after_right.ui_state.cursor_pos,
        (initial_pos.0, initial_pos.1 + 1)
    );

    let (session_after_left, transition) =
        state.handle_input(&session_after_down, KeyEvent::from(KeyCode::Left));
    assert_eq!(
        session_after_left.ui_state.cursor_pos,
        (initial_pos.0 + 1, initial_pos.1)
    );
    assert_eq!(transition, StateTransition::None);

    let (session_after_up, transition) =
        state.handle_input(&session_after_left, KeyEvent::from(KeyCode::Up));
    assert_eq!(session_after_up.ui_state.cursor_pos, initial_pos);
    assert_eq!(transition, StateTransition::None);
}

#[test]
fn test_playing_state_transitions_to_piece_selected() {
    let mut initial_session = GameSession::new();
    initial_session.ui_state.cursor_pos = (5, 0);
    let state = PlayingState::new();

    let (new_session, transition) =
        state.handle_input(&initial_session, KeyEvent::from(KeyCode::Enter));

    match transition {
        StateTransition::To(next_state) => {
            assert_eq!(
                next_state.state_type(),
                checkers_rs::state::StateType::PieceSelected
            );
        }
        _ => panic!("Expected transition to PieceSelectedState"),
    }

    assert_eq!(initial_session.ui_state.cursor_pos, (5, 0));
    assert_eq!(new_session.ui_state.cursor_pos, (5, 0));
}

#[test]
fn test_playing_state_transitions_to_ai_turn() {
    let mut initial_session = GameSession::new();
    initial_session.game = initial_session.game.with_switched_player();
    assert_eq!(initial_session.game.current_player, Color::Black);

    let state = PlayingState::new();

    let (_new_session, transition) =
        state.handle_input(&initial_session, KeyEvent::from(KeyCode::Up));

    match transition {
        StateTransition::To(next_state) => {
            assert_eq!(
                next_state.state_type(),
                checkers_rs::state::StateType::AITurn
            );
        }
        _ => panic!("Expected transition to AITurnState"),
    }

    assert_eq!(initial_session.game.current_player, Color::Black);
}

#[test]
fn test_playing_state_ignores_invalid_selection() {
    let mut initial_session = GameSession::new();
    initial_session.ui_state.cursor_pos = (3, 3);
    let state = PlayingState::new();

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
fn test_playing_state_cannot_select_opponent_piece() {
    let mut initial_session = GameSession::new();
    initial_session.ui_state.cursor_pos = (2, 1);
    assert_eq!(initial_session.game.current_player, Color::White);

    let state = PlayingState::new();

    let (new_session, transition) =
        state.handle_input(&initial_session, KeyEvent::from(KeyCode::Enter));

    assert_eq!(transition, StateTransition::None);
    assert_eq!(
        new_session.ui_state.cursor_pos,
        initial_session.ui_state.cursor_pos
    );
}

#[test]
fn test_playing_state_exit_on_quit() {
    let initial_session = GameSession::new();
    let state = PlayingState::new();

    let (_, transition) = state.handle_input(&initial_session, KeyEvent::from(KeyCode::Esc));
    assert_eq!(transition, StateTransition::Exit);

    let (_, transition) = state.handle_input(&initial_session, KeyEvent::from(KeyCode::Char('q')));
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
    let mut initial_session = GameSession::new();
    initial_session.ui_state.cursor_pos = (0, 0);
    let state = PlayingState::new();

    let (session_after_up, _) = state.handle_input(&initial_session, KeyEvent::from(KeyCode::Up));
    assert_eq!(session_after_up.ui_state.cursor_pos, (0, 0));

    let (session_after_left, _) =
        state.handle_input(&initial_session, KeyEvent::from(KeyCode::Left));
    assert_eq!(session_after_left.ui_state.cursor_pos, (0, 0));

    let mut corner_session = initial_session.clone();
    corner_session.ui_state.cursor_pos = (7, 7);

    let (session_after_down, _) =
        state.handle_input(&corner_session, KeyEvent::from(KeyCode::Down));
    assert_eq!(session_after_down.ui_state.cursor_pos, (7, 7));

    let (session_after_right, _) =
        state.handle_input(&corner_session, KeyEvent::from(KeyCode::Right));
    assert_eq!(session_after_right.ui_state.cursor_pos, (7, 7));
}
