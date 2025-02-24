// This module provides access to text prompts used in the game
#[allow(dead_code)]
pub fn get_story() -> &'static str {
    include_str!("story.txt")
}
