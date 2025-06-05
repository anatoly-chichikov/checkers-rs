use ratatui::style::Color;

/// Soft hipster color scheme
pub struct Theme;

impl Theme {
    // Primary colors
    pub const BORDER: Color = Color::Rgb(100, 100, 100); // Soft gray for borders
    pub const BORDER_FOCUSED: Color = Color::Rgb(200, 200, 200); // Light gray for focused

    // Text colors
    pub const TEXT_PRIMARY: Color = Color::Rgb(220, 220, 220); // Off-white for main text
    pub const TEXT_SECONDARY: Color = Color::Rgb(150, 150, 150); // Medium gray for secondary
    pub const TEXT_ACCENT: Color = Color::Rgb(150, 180, 200); // Soft blue-gray for accents

    // Game pieces
    pub const PIECE_BLACK: Color = Color::Rgb(180, 140, 100); // Warm brown/tan
    pub const PIECE_WHITE: Color = Color::Rgb(230, 230, 220); // Cream white

    // Board colors
    pub const BOARD_LIGHT: Color = Color::Rgb(60, 60, 60); // Pattern on light squares
    pub const POSSIBLE_MOVE: Color = Color::Rgb(120, 140, 100); // Soft olive green

    // UI elements
    pub const SEPARATOR: Color = Color::Rgb(80, 80, 80); // Subtle separators
    pub const HIGHLIGHT: Color = Color::Rgb(180, 180, 140); // Soft yellow highlight

    // Special - keep original
    pub const LOGO: Color = Color::Magenta; // Keep CHECKERS logo as is
    pub const EMOJI: Color = Color::Yellow; // Keep emoji colors
}
