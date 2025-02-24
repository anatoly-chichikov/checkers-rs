pub const LOADING_MESSAGE: &str = "Waiting for the magic to happen...";

pub const STORY_PROMPT: &str = include_str!("../utils/prompts/story.txt");

pub const WELCOME_MESSAGE_NO_API: &str = "# Welcome to Checkers!\n\n\
This is a terminal-based Checkers game. Use arrow keys to navigate, Enter to select and move pieces.\n\n\
*Note: Add NEBIUS_API_KEY to your .env file to enable AI-powered stories about checkers.*\n\n\
Press Enter to start the game...";

pub const WELCOME_MESSAGE_ERROR: &str = "# Welcome to Checkers!\n\n\
This is a terminal-based Checkers game. Use arrow keys to navigate, Enter to select and move pieces.\n\n\
Press Enter to start the game...";

pub const ERROR_PREFIX: &str = "**Error:**";
