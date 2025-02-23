# Checkers-RS

A terminal-based Checkers game implemented in Rust, featuring arrow key navigation and AI opponent capabilities.

## Features

- Terminal-based user interface with crossterm
- Arrow key navigation for piece movement
- AI opponent with multiple difficulty levels
- Real-time game state management
- Modern Rust implementation (2021 edition)

## Project Structure

- `src/`
  - `main.rs` - Application entry point and game loop
  - `game.rs` - Core game logic and state management
  - `board.rs` - Checkers board representation and operations
  - `piece.rs` - Piece types and movement rules
  - `ai.rs` - AI opponent implementation
  - `ui.rs` - Terminal UI rendering
  - `input.rs` - User input handling
  - `lib.rs` - Library exports and common utilities

## Dependencies

- `crossterm` (0.27) - Terminal manipulation and UI
- `tokio` (1.0) - Async runtime
- `serde` (1.0) - Serialization/deserialization
- `reqwest` (0.11) - HTTP client functionality
- `dotenv` (0.15) - Environment variable management
- Additional utility crates: thiserror, ctrlc, libc

## Getting Started

1. Ensure you have Rust installed on your system:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/checkers-rs.git
   cd checkers-rs
   ```

3. Build the project:
   ```bash
   cargo build --release
   ```

4. Run the game:
   ```bash
   cargo run --release
   ```

## Game Controls

- Arrow keys: Navigate the board
- Enter/Return: Select and move pieces
- Ctrl+C: Exit the game

## Development

### Running Tests

The project includes several types of tests:

1. Run all tests:
   ```bash
   cargo test
   ```

2. Run unit tests only:
   ```bash
   cargo test --lib
   ```

3. Run specific test file:
   ```bash
   cargo test --test game_tests    # Run game logic tests
   cargo test --test board_tests   # Run board tests
   ```

4. Run tests with output:
   ```bash
   cargo test -- --nocapture
   ```

### Debug Mode

For development and debugging, run the game in debug mode:
```bash
cargo run
```

This will include additional debug information and slower performance but faster compilation.

### Release Mode

For the best performance when playing:
```bash
cargo run --release
```

## Environment Variables

The project uses `.env` for configuration. Create a `.env` file in the root directory with the following variables:

- `NEBIUS_API_KEY` - API key for the Nebius AI service (required for game rules explanation)

Example `.env` file:
```env
NEBIUS_API_KEY=your_nebius_api_key_here
```

Note: The game will still work without these API keys, but the AI-powered rules explanation feature will not be available.
