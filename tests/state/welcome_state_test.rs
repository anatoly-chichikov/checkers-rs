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
    session.welcome_content = Some(content);

    let mut state = WelcomeState::new();

    // Test Enter key transitions to PlayingState
    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Enter));

    match transition {
        StateTransition::To(next_state) => {
            assert_eq!(
                next_state.state_type(),
                checkers_rs::state::StateType::Playing
            );
        }
        _ => panic!("Expected transition to PlayingState"),
    }
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

    let mut state = WelcomeState::new();

    // Test ESC key exits
    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Esc));

    match transition {
        StateTransition::Exit => {
            // Success
        }
        _ => panic!("Expected Exit transition"),
    }
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

    let mut state = WelcomeState::new();

    // Test 'q' key exits
    let transition = state.handle_input(&mut session, KeyEvent::from(KeyCode::Char('q')));

    match transition {
        StateTransition::Exit => {
            // Success
        }
        _ => panic!("Expected Exit transition"),
    }
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

    let mut state = WelcomeState::new();

    // Test other keys do nothing
    let keys = vec![
        KeyCode::Up,
        KeyCode::Down,
        KeyCode::Left,
        KeyCode::Right,
        KeyCode::Char('a'),
        KeyCode::Char(' '),
    ];

    for key in keys {
        let transition = state.handle_input(&mut session, KeyEvent::from(key));
        match transition {
            StateTransition::None => {
                // Success
            }
            _ => panic!("Expected None transition for key: {:?}", key),
        }
    }
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

    // Check that welcome content is set
    assert!(view.welcome_content.is_some());

    if let Some((fact, tip, challenge)) = view.welcome_content {
        assert_eq!(fact, "Test fact");
        assert_eq!(tip, "Test tip");
        assert_eq!(challenge, "Test challenge");
    }

    // Check other view properties
    assert_eq!(view.status_message, "Welcome to Checkers!");
    assert!(!view.show_ai_thinking);
    assert!(view.error_message.is_none());
    assert!(view.hint.is_none());
}
