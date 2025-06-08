use crate::ai::{hint::HintProvider, Hint};
use crate::core::game::{CheckersGame, GameError};
use crate::core::game_logic::find_capture_path;
use crate::state::ai_state::AIState;
use crate::state::states::WelcomeContent;
use crate::state::ui_state::UIState;

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

    pub fn select_piece(&mut self, row: usize, col: usize) -> Result<(), GameError> {
        if self.ui_state.selected_piece == Some((row, col)) {
            self.ui_state.clear_selection();
            return Ok(());
        }

        self.game.validate_piece_selection(row, col)?;
        self.ui_state.select_piece((row, col), &self.game.board);
        Ok(())
    }

    pub fn deselect_piece(&mut self) {
        self.ui_state.clear_selection();
    }

    pub fn make_move(&mut self, to_row: usize, to_col: usize) -> Result<bool, GameError> {
        let (from_row, from_col) = self
            .ui_state
            .selected_piece
            .ok_or(GameError::NoPieceSelected)?;

        let continue_capture = self.game.make_move(from_row, from_col, to_row, to_col)?;

        if continue_capture {
            self.ui_state
                .select_piece((to_row, to_col), &self.game.board);
        } else {
            self.ui_state.clear_selection();
        }

        Ok(continue_capture)
    }

    pub fn is_piece_selected(&self, pos: (usize, usize)) -> bool {
        self.ui_state.selected_piece == Some(pos)
    }

    pub fn try_multicapture_move(
        &mut self,
        to_row: usize,
        to_col: usize,
    ) -> Result<(bool, Vec<(usize, usize)>), GameError> {
        let (from_row, from_col) = self
            .ui_state
            .selected_piece
            .ok_or(GameError::NoPieceSelected)?;

        // Check if this is a multicapture move by finding the path
        if let Some(path) = find_capture_path(&self.game.board, from_row, from_col, to_row, to_col)
        {
            let mut current_pos = (from_row, from_col);
            let mut intermediate_positions = Vec::new();

            // Execute each step of the multicapture
            for &next_pos in &path {
                let continue_capture =
                    self.game
                        .make_move(current_pos.0, current_pos.1, next_pos.0, next_pos.1)?;
                intermediate_positions.push(next_pos);
                current_pos = next_pos;

                // If this wasn't the last move and capture doesn't continue, something went wrong
                if !continue_capture && next_pos != (to_row, to_col) {
                    return Err(GameError::InvalidMove);
                }
            }

            // Update UI state
            let final_continue = self.game.board.get_piece(to_row, to_col).is_some()
                && crate::core::game_logic::has_more_captures_for_piece(
                    &self.game.board,
                    to_row,
                    to_col,
                );

            if final_continue {
                self.ui_state
                    .select_piece((to_row, to_col), &self.game.board);
            } else {
                self.ui_state.clear_selection();
            }

            Ok((final_continue, intermediate_positions))
        } else {
            // Not a multicapture, try regular move
            let continue_capture = self.make_move(to_row, to_col)?;
            Ok((continue_capture, vec![(to_row, to_col)]))
        }
    }
}
