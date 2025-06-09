use crate::ai::{hint::HintProvider, Hint};
use crate::core::game::{CheckersGame, GameError};
use crate::core::game_logic::find_capture_path;
use crate::state::ai_state::AIState;
use crate::state::states::WelcomeContent;
use crate::state::ui_state::UIState;

#[derive(Clone)]
pub struct GameSession {
    pub game: CheckersGame,
    pub ui_state: UIState,
    pub ai_state: AIState,
    pub hint: Option<Hint>,
    pub hint_provider: Option<HintProvider>,
    pub welcome_content: Option<WelcomeContent>,
}

#[allow(clippy::derivable_impls)]
impl Default for GameSession {
    fn default() -> Self {
        Self {
            game: CheckersGame::new(),
            ui_state: UIState::new(),
            ai_state: AIState::new(),
            hint: None,
            hint_provider: None,
            welcome_content: None,
        }
    }
}

impl GameSession {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_ui_state(&self, ui_state: UIState) -> Self {
        let mut new_session = self.clone();
        new_session.ui_state = ui_state;
        new_session
    }


    pub fn select_piece(&self, row: usize, col: usize) -> Result<Self, GameError> {
        let mut new_session = self.clone();

        if new_session.ui_state.selected_piece == Some((row, col)) {
            new_session.ui_state = new_session.ui_state.clear_selection();
            return Ok(new_session);
        }

        new_session.game.validate_piece_selection(row, col)?;
        new_session.ui_state = new_session
            .ui_state
            .select_piece((row, col), &new_session.game.board);
        Ok(new_session)
    }

    pub fn deselect_piece(&self) -> Self {
        let mut new_session = self.clone();
        new_session.ui_state = new_session.ui_state.clear_selection();
        new_session
    }

    pub fn make_move(&self, to_row: usize, to_col: usize) -> Result<(Self, bool), GameError> {
        let mut new_session = self.clone();

        let (from_row, from_col) = new_session
            .ui_state
            .selected_piece
            .ok_or(GameError::NoPieceSelected)?;

        let (new_game, continue_capture) = new_session
            .game
            .make_move(from_row, from_col, to_row, to_col)?;
        new_session.game = new_game;

        if continue_capture {
            new_session.ui_state = new_session
                .ui_state
                .select_piece((to_row, to_col), &new_session.game.board);
        } else {
            new_session.ui_state = new_session.ui_state.clear_selection();
        }

        Ok((new_session, continue_capture))
    }


    #[allow(clippy::type_complexity)]
    pub fn try_multicapture_move(
        &self,
        to_row: usize,
        to_col: usize,
    ) -> Result<(Self, bool, Vec<(usize, usize)>), GameError> {
        let mut new_session = self.clone();

        let (from_row, from_col) = new_session
            .ui_state
            .selected_piece
            .ok_or(GameError::NoPieceSelected)?;

        // Check if this is a multicapture move by finding the path
        if let Some(path) =
            find_capture_path(&new_session.game.board, from_row, from_col, to_row, to_col)
        {
            let mut current_pos = (from_row, from_col);
            let mut intermediate_positions = Vec::new();

            // Execute each step of the multicapture
            for &next_pos in &path {
                let (updated_game, continue_capture) = new_session.game.make_move(
                    current_pos.0,
                    current_pos.1,
                    next_pos.0,
                    next_pos.1,
                )?;
                new_session.game = updated_game;
                intermediate_positions.push(next_pos);
                current_pos = next_pos;

                // If this wasn't the last move and capture doesn't continue, something went wrong
                if !continue_capture && next_pos != (to_row, to_col) {
                    return Err(GameError::InvalidMove);
                }
            }

            // Update UI state
            let final_continue = new_session.game.board.get_piece(to_row, to_col).is_some()
                && crate::core::game_logic::has_more_captures_for_piece(
                    &new_session.game.board,
                    to_row,
                    to_col,
                );

            if final_continue {
                new_session.ui_state = new_session
                    .ui_state
                    .select_piece((to_row, to_col), &new_session.game.board);
            } else {
                new_session.ui_state = new_session.ui_state.clear_selection();
            }

            Ok((new_session, final_continue, intermediate_positions))
        } else {
            // Not a multicapture, try regular move
            let (updated_session, continue_capture) = new_session.make_move(to_row, to_col)?;
            Ok((updated_session, continue_capture, vec![(to_row, to_col)]))
        }
    }
}
