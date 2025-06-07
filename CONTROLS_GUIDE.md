# Checkers Game Controls Guide

## Game States Overview

The game has several main states, each with its own set of controls.

## 1. Welcome Screen

**Description**: Initial screen displayed when the game starts, showing AI-generated content or default messages.

**UI Elements**:
- ASCII art "CHECKERS" header
- "Did You Know?" section (interesting facts about checkers)
- "💡 Tip of the Day" section
- "🎯 Today's Challenge" section
- Instructions at the bottom

**Controls**:
- `ENTER` - Start the game
- `Q` / `ESC` - Quit the application

## 2. Playing State (Human vs AI)

**Description**: Main game state when playing against AI (requires GEMINI_API_KEY and GEMINI_MODEL in .env file).

**UI Elements**:
- Status bar showing current turn
- 8x8 checkerboard with pieces
- Cursor position (highlighted square)
- Selected piece highlighting
- Possible moves highlighting
- Controls reminder at bottom
- Hint display (if available)
- AI thinking indicator
- AI error messages (if any)

**Controls**:
- `↑` `↓` `←` `→` - Move cursor around the board
- `SPACE` / `ENTER` - Select/deselect piece or make move
- `Q` / `ESC` - Quit game
- Note: Also accepts Cyrillic `й` / `Й` for quit

## 3. Playing State (Human vs Human)

**Description**: Local two-player mode when AI is not available (no API keys).

**UI Elements**: Same as AI mode but with "[LOCAL MODE - Playing against another human]" indicator.

**Controls**: Same as AI mode.

## 4. AI Thinking State

**Description**: Temporary state while AI calculates its move.

**UI Elements**:
- Same as playing state
- "AI thinking..." indicator in status bar

**Controls**: None (input is processed but AI turn blocks interaction).

## 5. Game Over State

**Description**: Final state when game ends (win or stalemate).

**UI Elements**:
- Centered popup box with:
  - "Game Over" header
  - Winner announcement ("White wins!" / "Black wins!" / "Stalemate! No possible moves.")
  - "Press any key to exit..." instruction

**Controls**:
- Any key - Exit the game

## Game Flow and State Transitions

1. **Application Start** → Welcome Screen
2. **Welcome Screen**:
   - `ENTER` → Playing State
   - `Q`/`ESC` → Exit Application
3. **Playing State**:
   - Game continues until win/stalemate → Game Over State
   - `Q`/`ESC` → Exit Application (with terminal restoration)
4. **Game Over State**:
   - Any key → Exit Application

## How to Move a Piece - Step by Step

**The complete flow for moving a piece:**

1. **Navigate to your piece** - Use arrow keys (`↑` `↓` `←` `→`) to move the cursor to the piece you want to move
2. **Select the piece** - Press `SPACE` or `ENTER` to select it (the piece will be highlighted)
3. **Navigate to destination** - Use arrow keys (`↑` `↓` `←` `→`) to move the cursor to where you want to move the piece
4. **Confirm the move** - Press `SPACE` or `ENTER` again to complete the move

**Important:** After selecting a piece with `SPACE`, you immediately use arrow keys to navigate to the destination square, then press `SPACE` again to confirm the move.

## Piece Selection States

Within the Playing State, there are sub-states for piece selection:

### 1. No Piece Selected
- Cursor can move freely with arrow keys
- `SPACE`/`ENTER` attempts to select piece at cursor position

### 2. Piece Selected
- Selected piece is highlighted with special border
- Possible valid moves are shown on the board
- Use arrow keys to navigate to any valid move position
- `SPACE`/`ENTER` on same piece → Deselect the piece
- `SPACE`/`ENTER` on valid move → Execute the move
- `SPACE`/`ENTER` on another piece of same color → Select new piece

### 3. Multi-capture Mode
- After capturing, if more captures are available with the same piece
- Piece remains selected automatically
- Only capture moves are shown
- Must complete all available captures

## Game Rules Enforced

- **Forced Capture**: If a capture is available, it must be taken
- **Multi-capture**: After capturing, if the same piece can capture again, it must
- **King Promotion**: Pieces reaching the opposite end become kings
- **Turn Alternation**: White always starts, players alternate turns

## Error States

The game handles various error conditions gracefully:
- Invalid move attempts (no error display, move is rejected)
- AI failures (error message shown, control returns to human)
- Missing API keys (game runs in local two-player mode)
- Ctrl+C handling (graceful shutdown)

## Running in tmux

To properly display the game with colors in tmux, you need to configure tmux for true color support:

### 1. Configure tmux for true color

Add these lines to your `~/.tmux.conf`:

```bash
# For tmux 3.2+
set -g default-terminal "tmux-256color"
set -ag terminal-features ",xterm-256color:RGB"

# For older tmux versions
set -g default-terminal "screen-256color"
set -ga terminal-overrides ",*256col*:Tc"
```

### 2. Start tmux with proper environment

```bash
# Set terminal type before starting tmux
export TERM=xterm-256color
tmux new-session -s checkers
```

### 3. Verify true color support

Inside tmux, test if true color is working:

```bash
printf "\x1b[38;2;255;100;0mTRUECOLOR\x1b[0m\n"
```

If you see orange text, true color is working correctly.

### 4. Terminal requirements

- Minimum terminal size: 80x24 characters
- The board requires at least 53 characters width and 18 lines height
- If the terminal is too small, the board will not render

## Notes

- The game maintains a clean, focused interface with no menus or configuration screens
- All game options are determined by environment variables at startup
- Board cells are enlarged for better visibility in terminal
- Red pieces start at top (rows 0-2), Black at bottom (rows 5-7)
- The game uses RGB colors which require true color terminal support