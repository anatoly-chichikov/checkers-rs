// This module provides access to text prompts used in the game
pub fn get_story() -> &'static str {
    include_str!("story.txt")
}
