use crate::ai::AIError;
use crate::core::{board::Board, game::CheckersGame, move_history::MoveHistory, piece::Color};
use std::future::Future;
use std::pin::Pin;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub trait RulesExplainer {
    fn explain<'a>(&'a self) -> BoxFuture<'a, Result<String, AIError>>;
}

pub trait MoveChooser {
    fn choose<'a>(
        &'a self,
        game: &'a CheckersGame,
    ) -> BoxFuture<'a, Result<((usize, usize), (usize, usize)), AIError>>;
}

pub trait HintGiver {
    fn hint<'a>(
        &'a self,
        board: &'a Board,
        player: Color,
        history: &'a MoveHistory,
    ) -> BoxFuture<'a, Result<String, AIError>>;
}

pub trait Agent: RulesExplainer + MoveChooser + HintGiver {}

impl<T> Agent for T where T: RulesExplainer + MoveChooser + HintGiver {}
