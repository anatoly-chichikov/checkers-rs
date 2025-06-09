use std::env;

use crate::ai::{explain_rules, hint::HintProvider, AIError};
use crate::core::piece::Color;
use crate::interface::ui_ratatui::{Input, UI};
use crate::state::states::{WelcomeContent, WelcomeState};
use crate::state::{GameSession, StateMachine, StateType};
use crossterm::event::{KeyCode, KeyEvent};

pub struct Application {
    ui: UI,
    session: GameSession,
    state_machine: StateMachine,
}

impl Application {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut ui = UI::new()?;
        ui.init()?;

        let mut session = GameSession::new();
        Self::initialize_hint_provider(&mut session);
        Self::initialize_welcome_content(&mut session).await;

        let state_machine = StateMachine::new(Box::new(WelcomeState::new()));

        Ok(Self {
            ui,
            session,
            state_machine,
        })
    }

    async fn initialize_welcome_content(session: &mut GameSession) {
        let content = Self::get_welcome_content().await;
        session.welcome_content = Some(content);
    }

    async fn get_welcome_content() -> WelcomeContent {
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
                    WelcomeContent {
                        did_you_know,
                        tip_of_the_day,
                        todays_challenge,
                    }
                } else {
                    Self::default_welcome_content()
                }
            }
            Err(AIError::NoApiKey) | Err(AIError::NoModel) => {
                // Silently fall back to default content
                Self::default_welcome_content()
            }
            Err(e) => {
                eprintln!("Failed to get checkers content from AI: {}", e);
                Self::default_welcome_content()
            }
        }
    }

    fn default_welcome_content() -> WelcomeContent {
        WelcomeContent {
            did_you_know: "The game of checkers has been played for thousands of years!"
                .to_string(),
            tip_of_the_day: "Always try to control the center of the board.".to_string(),
            todays_challenge: "Try to win a game without losing any pieces!".to_string(),
        }
    }

    fn initialize_hint_provider(session: &mut GameSession) {
        if let Ok(api_key) = env::var("GEMINI_API_KEY") {
            if env::var("GEMINI_MODEL").is_ok() {
                if let Ok(provider) = HintProvider::new(api_key) {
                    session.hint_provider = Some(provider);
                }
            }
        }
    }

    pub async fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let view = self.state_machine.get_view_data(&self.session);
            self.ui.draw_view_data(&view)?;

            if self.should_process_ai() {
                self.process_ai_frame()?;
            } else if !self.process_user_input()? {
                break;
            }
        }

        self.ui.restore()?;
        Ok(())
    }

    fn should_process_ai(&self) -> bool {
        matches!(
            self.state_machine.current_state_type(),
            StateType::AITurn
                | StateType::Playing
                    if self.session.game.current_player == Color::Black
        )
    }

    fn process_ai_frame(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Ok(Some(input)) = self.ui.poll_input() {
            if matches!(input, Input::Quit) {
                return Ok(());
            }
        }

        let (new_session, transition) = self
            .state_machine
            .handle_input(&self.session, KeyEvent::from(KeyCode::Char(' ')));
        self.session = new_session;
        self.state_machine.process_transition(transition);

        std::thread::sleep(std::time::Duration::from_millis(50));
        Ok(())
    }

    fn process_user_input(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        if let Ok(input) = self.ui.get_input() {
            let should_quit = matches!(input, Input::Quit);
            let key_event = self.input_to_key_event(input);
            let (new_session, transition) =
                self.state_machine.handle_input(&self.session, key_event);
            self.session = new_session;
            self.state_machine.process_transition(transition);

            if should_quit {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn input_to_key_event(&self, input: Input) -> KeyEvent {
        match input {
            Input::Up => KeyEvent::from(KeyCode::Up),
            Input::Down => KeyEvent::from(KeyCode::Down),
            Input::Left => KeyEvent::from(KeyCode::Left),
            Input::Right => KeyEvent::from(KeyCode::Right),
            Input::Select => KeyEvent::from(KeyCode::Enter),
            Input::Quit => KeyEvent::from(KeyCode::Esc),
        }
    }
}
