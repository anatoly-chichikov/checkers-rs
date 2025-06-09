use crate::core::board::Board;
use crate::core::game_logic::get_all_possible_moves;

#[derive(Clone)]
pub struct UIState {
    pub selected_piece: Option<(usize, usize)>,
    pub possible_moves: Vec<(usize, usize)>,
    pub cursor_pos: (usize, usize),
}

impl UIState {
    pub fn new() -> Self {
        Self {
            selected_piece: None,
            possible_moves: Vec::new(),
            cursor_pos: (0, 0),
        }
    }

    pub fn move_cursor_up(&self) -> Self {
        let mut new_state = self.clone();
        if new_state.cursor_pos.0 > 0 {
            new_state.cursor_pos.0 -= 1;
        }
        new_state
    }

    pub fn move_cursor_down(&self, max_row: usize) -> Self {
        let mut new_state = self.clone();
        if new_state.cursor_pos.0 < max_row {
            new_state.cursor_pos.0 += 1;
        }
        new_state
    }

    pub fn move_cursor_left(&self) -> Self {
        let mut new_state = self.clone();
        if new_state.cursor_pos.1 > 0 {
            new_state.cursor_pos.1 -= 1;
        }
        new_state
    }

    pub fn move_cursor_right(&self, max_col: usize) -> Self {
        let mut new_state = self.clone();
        if new_state.cursor_pos.1 < max_col {
            new_state.cursor_pos.1 += 1;
        }
        new_state
    }

    pub fn clear_selection(&self) -> Self {
        let mut new_state = self.clone();
        new_state.selected_piece = None;
        new_state.possible_moves.clear();
        new_state
    }

    pub fn select_piece(&self, pos: (usize, usize), board: &Board) -> Self {
        let mut new_state = self.clone();
        new_state.selected_piece = Some(pos);
        new_state.possible_moves = get_all_possible_moves(board, pos.0, pos.1);
        new_state
    }
}

impl Default for UIState {
    fn default() -> Self {
        Self::new()
    }
}
