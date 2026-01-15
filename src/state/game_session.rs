use crate::ai::{Agent, Hint};
use crate::core::board::{BoardSeed, Grid, Seed as BoardSeedTrait};
use crate::core::game::{CheckersGame, Game, GameError, GameSeed, Seed as GameSeedTrait};
use crate::core::piece::Color;
use crate::core::{Move, Position};
use crate::state::ai_state::AIState;
use crate::state::states::WelcomeContent;
use crate::state::ui_state::UIState;
use std::sync::Arc;

#[derive(Clone)]
pub struct GameSession {
    pub game: CheckersGame,
    pub ui_state: UIState,
    pub ai_state: AIState,
    pub hint: Option<Hint>,
    pub ai_agent: Option<Arc<dyn Agent + Send + Sync>>,
    pub welcome_content: Option<WelcomeContent>,
}

#[allow(clippy::derivable_impls)]
impl Default for GameSession {
    fn default() -> Self {
        let mut board = BoardSeed { size: 8 }.make();
        board.seed();
        let game = GameSeed {
            board,
            turn: Color::White,
        }
        .make();
        Self {
            game,
            ui_state: UIState::new(),
            ai_state: AIState::new(),
            hint: None,
            ai_agent: None,
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

        new_session.game.selection(Position { row, col })?;
        new_session.ui_state = new_session
            .ui_state
            .select_piece((row, col), new_session.game.board());
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

        let play = Move {
            origin: Position {
                row: from_row,
                col: from_col,
            },
            target: Position {
                row: to_row,
                col: to_col,
            },
        };
        let (new_game, continue_capture) = new_session.game.play(play)?;
        new_session.game = new_game;

        if continue_capture {
            new_session.ui_state = new_session
                .ui_state
                .select_piece((to_row, to_col), new_session.game.board());
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

        if let Some(path) = new_session.game.board().path(
            Position {
                row: from_row,
                col: from_col,
            },
            Position {
                row: to_row,
                col: to_col,
            },
        ) {
            let mut current_pos = (from_row, from_col);
            let mut intermediate_positions = Vec::new();

            for &next_pos in &path {
                let play = Move {
                    origin: Position {
                        row: current_pos.0,
                        col: current_pos.1,
                    },
                    target: Position {
                        row: next_pos.row,
                        col: next_pos.col,
                    },
                };
                let (updated_game, continue_capture) = new_session.game.play(play)?;
                new_session.game = updated_game;
                intermediate_positions.push((next_pos.row, next_pos.col));
                current_pos = (next_pos.row, next_pos.col);

                if !continue_capture && (next_pos.row, next_pos.col) != (to_row, to_col) {
                    return Err(GameError::InvalidMove);
                }
            }

            let final_continue = new_session
                .game
                .board()
                .piece(Position {
                    row: to_row,
                    col: to_col,
                })
                .is_some()
                && new_session.game.board().chain(Position {
                    row: to_row,
                    col: to_col,
                });

            if final_continue {
                new_session.ui_state = new_session
                    .ui_state
                    .select_piece((to_row, to_col), new_session.game.board());
            } else {
                new_session.ui_state = new_session.ui_state.clear_selection();
            }

            Ok((new_session, final_continue, intermediate_positions))
        } else {
            let (updated_session, continue_capture) = new_session.make_move(to_row, to_col)?;
            Ok((updated_session, continue_capture, vec![(to_row, to_col)]))
        }
    }
}
