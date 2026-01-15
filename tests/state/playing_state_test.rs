use checkers_rs::core::board::Grid;
use checkers_rs::core::game::Game;
use checkers_rs::core::piece::Color;
use checkers_rs::core::Position;
use checkers_rs::state::states::PlayingState;
use checkers_rs::state::{GameSession, State, StateTransition};
use crossterm::event::{KeyCode, KeyEvent};

#[test]
fn test_playing_state_handles_cursor_movement() {
    let base = GameSession::new();
    let state = PlayingState::new();
    let start = base.ui_state.cursor_pos;
    let east = (start.0, start.1 + 1);
    let south = (start.0 + 1, start.1 + 1);
    let west = (start.0 + 1, start.1);
    let (session, step) = state.handle_input(&base, KeyEvent::from(KeyCode::Right));
    let mut flag = step == StateTransition::None
        && session.ui_state.cursor_pos == east
        && base.ui_state.cursor_pos == start;
    let (session, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Down));
    flag = flag && step == StateTransition::None && session.ui_state.cursor_pos == south;
    let (session, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Left));
    flag = flag && step == StateTransition::None && session.ui_state.cursor_pos == west;
    let (session, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Up));
    flag = flag && step == StateTransition::None && session.ui_state.cursor_pos == start;
    assert!(flag, "cursor movement did not follow expected steps");
}

#[test]
fn test_playing_state_transitions_to_piece_selected() {
    let mut session = GameSession::new();
    session.ui_state.cursor_pos = (5, 0);
    let state = PlayingState::new();
    let (result, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Enter));
    let mut flag = false;
    if let StateTransition::To(state) = &step {
        flag = state.state_type() == checkers_rs::state::StateType::PieceSelected;
    }
    flag = flag && session.ui_state.cursor_pos == (5, 0) && result.ui_state.cursor_pos == (5, 0);
    assert!(flag, "piece selection did not transition correctly");
}

#[test]
fn test_playing_state_transitions_to_ai_turn() {
    let mut session = GameSession::new();
    session.game = session.game.switch();
    let state = PlayingState::new();
    let (_, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Up));
    let mut flag = false;
    if let StateTransition::To(state) = &step {
        flag = state.state_type() == checkers_rs::state::StateType::AITurn;
    }
    flag = flag && session.game.turn() == Color::Black;
    assert!(flag, "ai turn transition did not occur");
}

#[test]
fn test_playing_state_ignores_invalid_selection() {
    let mut session = GameSession::new();
    session.ui_state.cursor_pos = (3, 3);
    let state = PlayingState::new();
    let (result, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Enter));
    let mut same = true;
    for row in 0..8 {
        for col in 0..8 {
            let spot = Position { row, col };
            same = same && result.game.board().piece(spot) == session.game.board().piece(spot);
        }
    }
    let flag = step == StateTransition::None
        && result.ui_state.cursor_pos == session.ui_state.cursor_pos
        && same;
    assert!(flag, "invalid selection was not ignored");
}

#[test]
fn test_playing_state_cannot_select_opponent_piece() {
    let mut session = GameSession::new();
    session.ui_state.cursor_pos = (2, 1);
    let state = PlayingState::new();
    let (result, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Enter));
    let flag = step == StateTransition::None
        && result.ui_state.cursor_pos == session.ui_state.cursor_pos
        && session.game.turn() == Color::White;
    assert!(flag, "opponent piece selection was not blocked");
}

#[test]
fn test_playing_state_exit_on_quit() {
    let session = GameSession::new();
    let state = PlayingState::new();
    let (_, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Esc));
    let (_, exit) = state.handle_input(&session, KeyEvent::from(KeyCode::Char('q')));
    assert!(
        step == StateTransition::Exit && exit == StateTransition::Exit,
        "quit keys did not exit"
    );
}

#[test]
fn test_playing_state_view_data() {
    let session = GameSession::new();
    let state = PlayingState::new();
    let view = state.get_view_data(&session);
    let flag = !view.is_game_over
        && !view.show_ai_thinking
        && view.status_message.contains("White's turn")
        && view.current_player == Color::White;
    assert!(flag, "playing view data did not match expected values");
}

#[test]
fn test_playing_state_cursor_bounds() {
    let mut session = GameSession::new();
    session.ui_state.cursor_pos = (0, 0);
    let state = PlayingState::new();
    let (result, _) = state.handle_input(&session, KeyEvent::from(KeyCode::Up));
    let (result, _) = state.handle_input(&result, KeyEvent::from(KeyCode::Left));
    let mut flag = result.ui_state.cursor_pos == (0, 0);
    let mut edge = session.clone();
    edge.ui_state.cursor_pos = (7, 7);
    let (edge, _) = state.handle_input(&edge, KeyEvent::from(KeyCode::Down));
    let (edge, _) = state.handle_input(&edge, KeyEvent::from(KeyCode::Right));
    flag = flag && edge.ui_state.cursor_pos == (7, 7);
    assert!(flag, "cursor bounds were not enforced");
}
