// This module provides access to text prompts used in the game

pub fn get_hint_prompt() -> &'static str {
    include_str!("hint.txt")
}

pub fn get_ai_move_prompt() -> &'static str {
    include_str!("ai_move.txt")
}
