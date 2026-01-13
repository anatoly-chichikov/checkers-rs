use crate::core::piece::Color;
use crate::state::{GameSession, State, StateTransition, StateType, ViewData};
use crossterm::event::KeyEvent;

#[derive(Default)]
pub struct AITurnState {
    move_requested: bool,
}

impl AITurnState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Marks that move request is issued.
    fn requested(&self) -> Self {
        Self {
            move_requested: true,
        }
    }

    /// Uses configured AI agent when available.
    fn actual(&self, session: &GameSession) -> (GameSession, StateTransition) {
        let mut new_session = session.clone();
        new_session.ai_state = new_session.ai_state.start_thinking();
        let agent = new_session.ai_agent.clone();
        match agent {
            Some(agent_ref) => {
                let ai_result = tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current().block_on(agent_ref.choose(&session.game))
                });
                match ai_result {
                    Ok((from, to)) => Self::apply(new_session, from, to),
                    Err(e) => Self::fail(new_session, format!("AI error {e}")),
                }
            }
            None => Self::basic(&new_session),
        }
    }

    /// Applies AI move and updates game state.
    fn apply(
        mut new_session: GameSession,
        from: (usize, usize),
        to: (usize, usize),
    ) -> (GameSession, StateTransition) {
        let game_move = crate::core::GameMove::from_tuples(from, to);
        match new_session.game.make_move(game_move) {
            Ok((updated_game, _)) => {
                new_session.game = updated_game;
                new_session.ai_state = new_session.ai_state.clear_error();
                if let Some(ref agent_ref) = new_session.ai_agent {
                    if new_session.game.current_player == Color::White
                        && !new_session.game.is_game_over
                    {
                        let hint_result = tokio::task::block_in_place(|| {
                            tokio::runtime::Handle::current().block_on(agent_ref.hint(
                                &new_session.game.board,
                                Color::White,
                                &new_session.game.move_history,
                            ))
                        });
                        new_session.hint = hint_result
                            .ok()
                            .map(|hint_text| crate::ai::Hint { hint: hint_text });
                    }
                }
                let transition = Self::after(&mut new_session);
                match transition {
                    Some(t) => (new_session, t),
                    None => (
                        new_session,
                        StateTransition::To(Box::new(super::PlayingState::new())),
                    ),
                }
            }
            Err(e) => Self::fail(new_session, format!("AI failed to move: {e}")),
        }
    }

    /// Handles post-move transitions.
    fn after(new_session: &mut GameSession) -> Option<StateTransition> {
        let winner = new_session.game.check_winner();
        if winner.is_some() {
            new_session.game.is_game_over = true;
            Some(StateTransition::To(Box::new(super::GameOverState::new(
                winner,
            ))))
        } else if new_session.game.is_stalemate() {
            let winner = Some(new_session.game.current_player.opposite());
            new_session.game.is_game_over = true;
            Some(StateTransition::To(Box::new(super::GameOverState::new(
                winner,
            ))))
        } else {
            None
        }
    }

    /// Records AI failure and switches player.
    fn fail(mut new_session: GameSession, message: String) -> (GameSession, StateTransition) {
        new_session.ai_state = new_session.ai_state.set_error(message);
        new_session.game = new_session.game.with_switched_player();
        (
            new_session,
            StateTransition::To(Box::new(super::PlayingState::new())),
        )
    }

    /// Simple deterministic AI fallback.
    fn basic(session: &GameSession) -> (GameSession, StateTransition) {
        let mut new_session = session.clone();
        let all_moves = crate::core::game_logic::get_all_valid_moves_for_player(
            &new_session.game.board,
            Color::Black,
        );
        if all_moves.is_empty() {
            new_session.game.is_game_over = true;
            return (
                new_session,
                StateTransition::To(Box::new(super::GameOverState::new(Some(Color::White)))),
            );
        }
        let captures: Vec<_> = all_moves
            .iter()
            .filter(|(_, _, is_capture)| *is_capture)
            .cloned()
            .collect();
        let moves_to_consider = if !captures.is_empty() {
            captures
        } else {
            all_moves
        };
        let ((from_row, from_col), (to_row, to_col), _) = moves_to_consider[0];
        match new_session
            .game
            .make_move_coords(from_row, from_col, to_row, to_col)
        {
            Ok((updated_game, _)) => {
                new_session.game = updated_game;
                new_session.ai_state = new_session.ai_state.clear_error();
                let transition = Self::after(&mut new_session);
                match transition {
                    Some(t) => (new_session, t),
                    None => (
                        new_session,
                        StateTransition::To(Box::new(super::PlayingState::new())),
                    ),
                }
            }
            Err(e) => Self::fail(new_session, format!("AI error: {e}")),
        }
    }
}

impl State for AITurnState {
    fn handle_input(
        &self,
        session: &GameSession,
        _key: KeyEvent,
    ) -> (GameSession, StateTransition) {
        if !self.move_requested {
            let next = self.actual(session);
            let transition = match next.1 {
                StateTransition::None => StateTransition::To(Box::new(self.requested())),
                other => other,
            };
            return (next.0, transition);
        }
        (session.clone(), StateTransition::None)
    }

    fn get_view_data<'a>(&self, session: &'a GameSession) -> ViewData<'a> {
        ViewData {
            board: &session.game.board,
            current_player: session.game.current_player,
            cursor_pos: session.ui_state.cursor_pos,
            selected_piece: None,
            possible_moves: &[],
            pieces_with_captures: Vec::new(),
            status_message: "AI is thinking...".to_string(),
            show_ai_thinking: true,
            error_message: session.ai_state.last_error.as_deref(),
            is_simple_ai: std::env::var("GEMINI_API_KEY").is_err()
                || std::env::var("GEMINI_MODEL").is_err(),
            hint: session.hint.as_ref(),
            is_game_over: false,
            welcome_content: None,
        }
    }

    fn state_type(&self) -> StateType {
        StateType::AITurn
    }
}
