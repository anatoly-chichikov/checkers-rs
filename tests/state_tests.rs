mod state;

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
            assert_eq!(next_state.state_type(), checkers_rs::state::StateType::Playing);
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
