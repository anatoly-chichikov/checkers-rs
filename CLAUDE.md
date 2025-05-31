# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Common Commands

**Build & Run:**
```bash
cargo run --release              # Run the game (release mode recommended for performance)
cargo build                      # Build debug version
cargo build --release            # Build optimized version
```

**Testing:**
```bash
cargo test                       # Run all tests
cargo test core::                # Run core game logic tests
cargo test ai::                  # Run AI tests
cargo test -- --nocapture        # Run tests with stdout output
```

**Linting & Formatting:**
```bash
cargo fmt                        # Format code
cargo clippy -- -D warnings      # Run linter (fail on warnings)
```

## Pre-commit Checklist

Before committing changes, always run:
```bash
cargo fmt                        # Format code
cargo clippy -- -D warnings      # Check for linting issues
cargo test                       # Run all tests
```

## Architecture Overview

This is a terminal-based checkers game built in Rust with a modular architecture:

1. **Core Game Logic** (`src/core/`):
   - `board.rs`: 8x8 board representation with piece positions
   - `game.rs`: Game state management, turn handling, win conditions
   - `game_logic.rs`: Move validation, capture logic, king promotion
   - `piece.rs`: Piece types (regular/king) and colors (red/black)

2. **Terminal Interface** (`src/interface/`):
   - `ui.rs`: Board rendering using crossterm, handles enlarged cells for better visibility
   - `input.rs`: Arrow key navigation, space/enter for selection, ESC/Q to quit
   - `messages.rs`: User-facing game messages

3. **AI Integration** (`src/ai/`):
   - `gemini_client.rs`: Gemini API client for AI opponent moves
   - Requires `GEMINI_API_KEY` in `.env` file
   - Game functions without API key but AI opponent won't work

4. **Key Design Decisions**:
   - Uses crossterm for cross-platform terminal manipulation
   - Async runtime (tokio) for non-blocking AI API calls
   - Board coordinates use (row, col) with 0-based indexing
   - Red pieces start at top (rows 0-2), Black at bottom (rows 5-7)
   - Forced capture rule: if a capture is available, it must be taken

## Testing Approach

Tests are in `tests/` directory mirroring the source structure:
- `tests/core/`: Game logic unit tests
- `tests/ai/`: AI integration tests
- Tests use the library crate exposed via `src/lib.rs`