pub mod ai;
pub mod core;
pub mod interface;
pub mod state;
pub mod utils;

// Make this available for integration tests too
pub mod test_helpers {
    use crate::state::GameSession;

    pub trait MultiCaptureCheck {
        fn is_in_multi_capture(&self) -> bool;
    }

    impl MultiCaptureCheck for GameSession {
        fn is_in_multi_capture(&self) -> bool {
            // Check if we have a selected piece and all possible moves are captures
            if let Some((row, col)) = self.ui_state.selected_piece {
                let possible_moves = &self.ui_state.possible_moves;
                // If there are possible moves and they are all captures (distance of 2)
                !possible_moves.is_empty()
                    && possible_moves.iter().all(|(to_row, to_col)| {
                        ((*to_row as i32 - row as i32).abs() == 2)
                            && ((*to_col as i32 - col as i32).abs() == 2)
                    })
            } else {
                false
            }
        }
    }
}
