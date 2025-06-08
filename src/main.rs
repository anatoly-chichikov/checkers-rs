mod ai;
mod core;
mod interface;
mod state;
mod utils;

use std::env;

use crate::ai::{explain_rules, hint::HintProvider, AIError};
use crate::core::piece::Color;
use crate::interface::ui_ratatui::{Input, UI};
use crate::state::{
    states::{WelcomeContent, WelcomeState},
    GameSession, StateMachine,
};
use crossterm::event::{KeyCode, KeyEvent};

async fn get_welcome_content() -> (String, String, String) {
    match explain_rules().await {
        Ok(rules) => {
            let parts: Vec<&str> = rules.split("\n\n").collect();
            if parts.len() >= 3 {
                let did_you_know = parts[0]
                    .strip_prefix("Did You Know?\n")
                    .or_else(|| parts[0].strip_prefix("**Did You Know?**\n"))
                    .unwrap_or(parts[0])
                    .replace("**", "")
                    .to_string();
                let tip_of_the_day = parts[1]
                    .strip_prefix("Tip of the Day\n")
                    .or_else(|| parts[1].strip_prefix("**Tip of the Day**\n"))
                    .unwrap_or(parts[1])
                    .replace("**", "")
                    .to_string();
                let todays_challenge = parts[2]
                    .strip_prefix("Challenge\n")
                    .or_else(|| parts[2].strip_prefix("**Challenge**\n"))
                    .or_else(|| parts[2].strip_prefix("**Today's Challenge**\n"))
                    .unwrap_or(parts[2])
                    .replace("**", "")
                    .to_string();
                (did_you_know, tip_of_the_day, todays_challenge)
            } else {
                default_welcome_content()
            }
        }
        Err(AIError::NoApiKey) | Err(AIError::NoModel) => {
            eprintln!("Note: Add GEMINI_API_KEY and GEMINI_MODEL to your .env file to enable AI-powered content.");
            default_welcome_content()
        }
        Err(e) => {
            eprintln!("Failed to get checkers content from AI: {}", e);
            default_welcome_content()
        }
    }
}

fn default_welcome_content() -> (String, String, String) {
    (
        "The game of checkers has been played for thousands of years!".to_string(),
        "Always try to control the center of the board.".to_string(),
        "Try to win a game without losing any pieces!".to_string(),
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize UI
    let mut ui = UI::new()?;
    ui.init()?;

    // Get welcome content
    let (did_you_know, tip_of_the_day, todays_challenge) = get_welcome_content().await;

    // Initialize state machine with welcome state
    let mut session = GameSession::new();
    session.welcome_content = Some(WelcomeContent {
        did_you_know: did_you_know.clone(),
        tip_of_the_day: tip_of_the_day.clone(),
        todays_challenge: todays_challenge.clone(),
    });

    // Initialize hint provider if API key is available
    if let Ok(api_key) = env::var("GEMINI_API_KEY") {
        if env::var("GEMINI_MODEL").is_ok() {
            if let Ok(provider) = HintProvider::new(api_key) {
                session.hint_provider = Some(provider);
            }
        }
    }

    let mut state_machine = StateMachine::new(Box::new(WelcomeState::new()));

    // Main state machine loop
    loop {
        let view = state_machine.get_view_data(&session);
        ui.draw_view_data(&view)?;

        // Add small delay to see welcome screen
        if state_machine.current_state_name() == "WelcomeState" {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        // Check if current state is AITurnState or if we need to transition to AI turn
        let current_state = state_machine.current_state_name();
        let is_ai_turn = current_state == "AITurnState";
        let should_check_ai_transition =
            current_state == "PlayingState" && session.game.current_player == Color::Black;

        if is_ai_turn || should_check_ai_transition {
            // For AI turn or potential AI turn, use non-blocking input and always trigger handle_input
            if let Ok(Some(input)) = ui.poll_input() {
                if matches!(input, Input::Quit) {
                    break;
                }
            }
            // Always call handle_input to let state machine progress (for AI turn or transition)
            if is_ai_turn || should_check_ai_transition {
                state_machine.handle_input(&mut session, KeyEvent::from(KeyCode::Char(' ')));
            }

            // Small delay to prevent busy loop
            std::thread::sleep(std::time::Duration::from_millis(50));
        } else {
            // For other states, use blocking input
            if let Ok(input) = ui.get_input() {
                let key_event = match input {
                    Input::Up => KeyEvent::from(KeyCode::Up),
                    Input::Down => KeyEvent::from(KeyCode::Down),
                    Input::Left => KeyEvent::from(KeyCode::Left),
                    Input::Right => KeyEvent::from(KeyCode::Right),
                    Input::Select => KeyEvent::from(KeyCode::Enter),
                    Input::Quit => KeyEvent::from(KeyCode::Esc),
                };
                state_machine.handle_input(&mut session, key_event);

                // Check if we should exit
                if matches!(input, Input::Quit) {
                    break;
                }
            }
        }
    }

    ui.restore()?;
    Ok(())
}
