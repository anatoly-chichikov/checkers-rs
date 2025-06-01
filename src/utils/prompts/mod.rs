// This module provides access to text prompts used in the game
#[allow(dead_code)]
pub fn get_story() -> &'static str {
    include_str!("story.txt")
}

pub fn get_hint_prompt() -> &'static str {
    include_str!("hint.txt")
}

pub fn get_ai_move_prompt() -> &'static str {
    include_str!("ai_move.txt")
}
