use checkers_rs::core::board::Grid;
use checkers_rs::core::game::Game;
use checkers_rs::core::Position;
use checkers_rs::state::{
    states::{WelcomeContent, WelcomeState},
    GameSession, State, StateTransition,
};
use crossterm::event::{KeyCode, KeyEvent};

#[test]
fn test_welcome_state_transitions_to_playing_on_enter() {
    let content = WelcomeContent {
        did_you_know: "Test fact".to_string(),
        tip_of_the_day: "Test tip".to_string(),
        todays_challenge: "Test challenge".to_string(),
    };
    let mut session = GameSession::new();
    session.welcome_content = Some(content.clone());
    let state = WelcomeState::new();
    let (result, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Enter));
    let mut flag = false;
    if let StateTransition::To(state) = &step {
        flag = state.state_type() == checkers_rs::state::StateType::Playing;
    }
    let mut same = true;
    for row in 0..8 {
        for col in 0..8 {
            let spot = Position { row, col };
            same = same && result.game.board().piece(spot) == session.game.board().piece(spot);
        }
    }
    flag = flag && session.welcome_content.as_ref().unwrap().did_you_know == "Test fact" && same;
    assert!(flag, "welcome enter did not transition or preserve session");
}

#[test]
fn test_welcome_state_exits_on_esc() {
    let content = WelcomeContent {
        did_you_know: "Test fact".to_string(),
        tip_of_the_day: "Test tip".to_string(),
        todays_challenge: "Test challenge".to_string(),
    };
    let mut session = GameSession::new();
    session.welcome_content = Some(content);
    let state = WelcomeState::new();
    let (_, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Esc));
    assert!(
        step == StateTransition::Exit,
        "esc did not exit welcome state"
    );
}

#[test]
fn test_welcome_state_exits_on_q() {
    let content = WelcomeContent {
        did_you_know: "Test fact".to_string(),
        tip_of_the_day: "Test tip".to_string(),
        todays_challenge: "Test challenge".to_string(),
    };
    let mut session = GameSession::new();
    session.welcome_content = Some(content);
    let state = WelcomeState::new();
    let (_, step) = state.handle_input(&session, KeyEvent::from(KeyCode::Char('q')));
    assert!(
        step == StateTransition::Exit,
        "q did not exit welcome state"
    );
}

#[test]
fn test_welcome_state_ignores_other_keys() {
    let content = WelcomeContent {
        did_you_know: "Test fact".to_string(),
        tip_of_the_day: "Test tip".to_string(),
        todays_challenge: "Test challenge".to_string(),
    };
    let mut session = GameSession::new();
    session.welcome_content = Some(content);
    let state = WelcomeState::new();
    let keys = vec![
        KeyCode::Up,
        KeyCode::Down,
        KeyCode::Left,
        KeyCode::Right,
        KeyCode::Char('a'),
        KeyCode::Char(' '),
    ];
    let mut flag = true;
    for key in keys {
        let (result, step) = state.handle_input(&session, KeyEvent::from(key));
        let mut same = true;
        for row in 0..8 {
            for col in 0..8 {
                let spot = Position { row, col };
                same = same && result.game.board().piece(spot) == session.game.board().piece(spot);
            }
        }
        flag = flag && step == StateTransition::None && same;
    }
    assert!(flag, "unexpected key input altered welcome state");
}

#[test]
fn test_welcome_state_view_data() {
    let content = WelcomeContent {
        did_you_know: "Test fact".to_string(),
        tip_of_the_day: "Test tip".to_string(),
        todays_challenge: "Test challenge".to_string(),
    };
    let mut session = GameSession::new();
    session.welcome_content = Some(content);
    let state = WelcomeState::new();
    let view = state.get_view_data(&session);
    let content = view.welcome_content.clone();
    let mut flag = false;
    if let Some((fact, tip, challenge)) = content {
        flag = fact == "Test fact" && tip == "Test tip" && challenge == "Test challenge";
    }
    flag = flag
        && view.status_message == "Welcome to Checkers!"
        && !view.show_ai_thinking
        && view.error_message.is_none()
        && view.hint.is_none();
    assert!(flag, "welcome view did not contain expected content");
}
