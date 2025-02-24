# Terminal Checkers in Rust

A simple terminal-based Checkers game with arrow key navigation.

**Features:**
- Terminal interface using crossterm
- Move pieces using arrow keys
- AI opponent with adjustable difficulty

**Getting Started:**
1. **Install Rust using Homebrew:**
   ```bash
   brew install rust
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

**Controls:**
- **Arrow keys:** Navigate the board
- **Space or Enter:** Select/deselect pieces and make moves
- **Esc or Q:** Exit the game

**Project Structure:**
```
src/
  ├── core/          # Core game components
  │   ├── board.rs   # Game board representation
  │   ├── game.rs    # Game state and rules
  │   ├── game_logic.rs # Game logic functions
  │   └── piece.rs   # Piece representation
  ├── interface/     # User interaction
  │   ├── input.rs   # Input handling
  │   ├── messages.rs # Game messages
  │   └── ui.rs      # User interface rendering
  ├── ai/            # AI components
  │   └── ai.rs      # AI opponent logic
  └── utils/         # Utilities
      ├── markdown/  # Markdown parsing for text display
      └── prompts/   # Game text and prompts
```

**Testing:**
Run tests with:
```bash
cargo test
```

**Environment Variables:**
Create a `.env` file with:
```env
NEBIUS_API_KEY=your_nebius_api_key_here
```
*(This enables AI-powered rule explanations; the game will still work without it.)*