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

    pub fn move_cursor_up(&mut self) {
        if self.cursor_pos.0 > 0 {
            self.cursor_pos.0 -= 1;
        }
    }

    pub fn move_cursor_down(&mut self, max_row: usize) {
        if self.cursor_pos.0 < max_row {
            self.cursor_pos.0 += 1;
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_pos.1 > 0 {
            self.cursor_pos.1 -= 1;
        }
    }

    pub fn move_cursor_right(&mut self, max_col: usize) {
        if self.cursor_pos.1 < max_col {
            self.cursor_pos.1 += 1;
        }
    }

    pub fn clear_selection(&mut self) {
        self.selected_piece = None;
        self.possible_moves.clear();
    }

    pub fn select_piece(&mut self, pos: (usize, usize), board: &Board) {
        self.selected_piece = Some(pos);
        self.possible_moves = get_all_possible_moves(board, pos.0, pos.1);
    }
}

impl Default for UIState {
    fn default() -> Self {
        Self::new()
    }
}
