use crate::ai::Hint;
use crate::core::board::Board;
use crate::core::move_history::Move;
use crate::core::piece::Color;

pub struct ViewData<'a> {
    pub board: &'a Board,
    pub current_player: Color,
    pub cursor_pos: (usize, usize),
    pub selected_piece: Option<(usize, usize)>,
    pub possible_moves: &'a [(usize, usize)],
    pub pieces_with_captures: Vec<(usize, usize)>,

    pub status_message: String,
    pub show_ai_thinking: bool,
    pub error_message: Option<&'a str>,

    #[allow(dead_code)]
    pub last_move: Option<&'a Move>,
    pub hint: Option<&'a Hint>,
    pub is_game_over: bool,

    // Welcome screen data (optional)
    pub welcome_content: Option<(&'a str, &'a str, &'a str)>,
}
