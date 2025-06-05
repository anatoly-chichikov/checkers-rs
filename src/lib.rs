pub mod ai;
pub mod core;
pub mod interface;
pub mod utils;

// Make this available for integration tests too
pub mod test_helpers {
    use crate::core::game::CheckersGame;

    pub trait MultiCaptureCheck {
        fn is_in_multi_capture(&self) -> bool;
    }

    impl MultiCaptureCheck for CheckersGame {
        fn is_in_multi_capture(&self) -> bool {
            // Check if we have a selected piece and all possible moves are captures
            if let Some((row, col)) = self.selected_piece {
                if let Some(possible_moves) = &self.possible_moves {
                    // If there are possible moves and they are all captures (distance of 2)
                    !possible_moves.is_empty()
                        && possible_moves.iter().all(|(to_row, to_col)| {
                            ((*to_row as i32 - row as i32).abs() == 2)
                                && ((*to_col as i32 - col as i32).abs() == 2)
                        })
                } else {
                    false
                }
            } else {
                false
            }
        }
    }
}
