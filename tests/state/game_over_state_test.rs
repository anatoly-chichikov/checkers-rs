use checkers_rs::core::piece::Color;
use checkers_rs::state::states::GameOverState;
use checkers_rs::state::{GameSession, State, StateTransition};
use crossterm::event::{KeyCode, KeyEvent};

#[test]
fn test_game_over_state_displays_winner_message() {
    let mut session = GameSession::new();
    session.game.is_game_over = true;

    let state = GameOverState::new(Some(Color::White));
    let view_data = state.get_view_data(&session);

    assert!(view_data.is_game_over);
    assert!(view_data.status_message.contains("White wins"));
}

#[test]
fn test_game_over_state_displays_stalemate_message() {
    let mut session = GameSession::new();
    session.game.is_game_over = true;

    let state = GameOverState::new(None);
    let view_data = state.get_view_data(&session);

    assert!(view_data.is_game_over);
    assert!(view_data.status_message.contains("Stalemate"));
}

#[test]
fn test_game_over_state_exits_on_any_key() {
    let mut session = GameSession::new();
    let mut state = GameOverState::new(Some(Color::Black));

    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Enter));
    assert_eq!(transition, StateTransition::Exit);

    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Esc));
    assert_eq!(transition, StateTransition::Exit);

    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Char('a')));
    assert_eq!(transition, StateTransition::Exit);
}

#[test]
fn test_game_over_state_shows_correct_winner_for_black() {
    let mut session = GameSession::new();
    session.game.is_game_over = true;

    let state = GameOverState::new(Some(Color::Black));
    let view_data = state.get_view_data(&session);

    assert!(view_data.status_message.contains("Black wins"));
}

#[test]
fn test_game_over_state_shows_correct_winner_for_white() {
    let mut session = GameSession::new();
    session.game.is_game_over = true;

    let state = GameOverState::new(Some(Color::White));
    let view_data = state.get_view_data(&session);

    assert!(view_data.status_message.contains("White wins"));
}
