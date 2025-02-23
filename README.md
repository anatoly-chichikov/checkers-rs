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
   git clone https://github.com/yourusername/checkers-rs.git
   cd checkers-rs
   ```
3. **Run the game:**
   ```bash
   cargo run --release
   ```

**Controls:**
- **Arrow keys:** Navigate the board
- **Enter:** Select and move pieces
- **Esc:** Exit the game

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