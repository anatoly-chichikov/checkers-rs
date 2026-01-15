use checkers_rs::core::board::Grid;
use checkers_rs::core::game::Game;
use checkers_rs::core::piece::Color;
use checkers_rs::core::Position;
use checkers_rs::state::states::GameOverState;
use checkers_rs::state::{GameSession, State, StateTransition};
use crossterm::event::{KeyCode, KeyEvent};

#[test]
fn test_game_over_state_displays_winner_message() {
    let mut session = GameSession::new();
    session.game = session.game.finish();
    let state = GameOverState::new(Some(Color::White));
    let view = state.get_view_data(&session);
    assert!(
        view.is_game_over && view.status_message.contains("White wins"),
        "winner message was not displayed"
    );
}

#[test]
fn test_game_over_state_displays_stalemate_message() {
    let mut session = GameSession::new();
    session.game = session.game.finish();
    let state = GameOverState::new(None);
    let view = state.get_view_data(&session);
    assert!(
        view.is_game_over && view.status_message.contains("Stalemate"),
        "stalemate message was not displayed"
    );
}

#[test]
fn test_game_over_state_exits_only_on_esc() {
    let session = GameSession::new();
    let state = GameOverState::new(Some(Color::Black));
    let (result, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Enter));
    let mut flag = step == StateTransition::None;
    let mut same = true;
    for row in 0..8 {
        for col in 0..8 {
            let spot = Position { row, col };
            same = same && result.game.board().piece(spot) == session.game.board().piece(spot);
        }
    }
    let (_, exit) = state.handle_input(&session, KeyEvent::from(KeyCode::Esc));
    let (_, other) = state.handle_input(&session, KeyEvent::from(KeyCode::Char('a')));
    flag = flag && same && exit == StateTransition::Exit && other == StateTransition::None;
    assert!(
        flag,
        "game over input handling did not match expected transitions"
    );
}

#[test]
fn test_game_over_state_shows_correct_winner_for_black() {
    let mut session = GameSession::new();
    session.game = session.game.finish();
    let state = GameOverState::new(Some(Color::Black));
    let view = state.get_view_data(&session);
    assert!(
        view.status_message.contains("Black wins"),
        "black winner message was not displayed"
    );
}

#[test]
fn test_game_over_state_shows_correct_winner_for_white() {
    let mut session = GameSession::new();
    session.game = session.game.finish();
    let state = GameOverState::new(Some(Color::White));
    let view = state.get_view_data(&session);
    assert!(
        view.status_message.contains("White wins"),
        "white winner message was not displayed"
    );
}
