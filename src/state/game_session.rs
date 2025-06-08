use crate::ai::{hint::HintProvider, Hint};
use crate::core::game::{CheckersGame, GameError};
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
}
