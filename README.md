# Terminal Checkers in Rust

A terminal-based Checkers game built with Rust.

**Features:**
- Terminal interface using crossterm
- AI opponent powered by Gemini API (when API key is available)
- Two-player hot-seat mode (when no API key is present)
- Arrow key navigation with visual feedback
- Full checkers rules including forced captures and king promotion

**Getting Started:**

1. **Install Rust:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   
2. **Clone the repository:**
   ```bash
   git clone https://github.com/anatoly-chichikov/checkers-rs.git
   cd checkers-rs
   ```
   
3. **Run the game:**
   ```bash
   cargo run --release
   ```

**Game Modes:**
- **With AI (requires API key):** Human plays as White vs AI as Black
- **Two-player mode (no API key):** Players take turns on the same keyboard

**Controls:**
- **Arrow keys:** Navigate the board
- **Space or Enter:** Select/deselect pieces and make moves
- **Esc or Q:** Exit the game

**Rules:**
- White pieces start at the bottom
- Black pieces start at the top
- Regular pieces can only move diagonally forward
- Kings (marked with brackets) can move in any diagonal direction
- Captures are mandatory - if you can capture, you must
- Multiple captures must be completed in one turn
- Pieces become kings when reaching the opposite end of the board

**Environment Setup (Optional):**
To enable AI opponent, create a `.env` file:
```env
GEMINI_API_KEY=your_gemini_api_key_here
```
*Without an API key, the game runs in two-player mode*

**Testing:**
```bash
cargo test
```