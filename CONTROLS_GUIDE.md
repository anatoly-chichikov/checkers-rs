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
- Cursor position (highlighted square with bold border)
- Selected piece highlighting (bold double-line border)
- Possible moves highlighting (green background color)
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
- Selected piece is highlighted with special border (bold double lines)
- **Possible valid moves are shown with green background highlighting**
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

To properly display the game with colors in tmux:

```bash
# Create a tmux session
tmux new-session -d -s checkers

# Start the game with color support
tmux send-keys -t checkers 'TERM=xterm-256color cargo run --release' Enter
```

**Important**: Use the `-d` flag to create the session in detached mode, otherwise the command may hang when run programmatically.

### Verify true color support (optional)

Inside tmux, you can test if true color is working:

```bash
printf "\x1b[38;2;255;100;0mTRUECOLOR\x1b[0m\n"
```

If you see orange text, true color is working correctly.

### Terminal requirements

- Minimum terminal size: 80x24 characters
- The board requires at least 53 characters width and 18 lines height
- If the terminal is too small, the board will not render

### Capturing colors in tmux (for automation/agents)

When automating the game or using agents, you MUST capture the pane with ANSI color codes to see highlighted moves:

```bash
# Capture with colors preserved - CRITICAL for seeing valid moves
tmux capture-pane -t <pane-id> -p -e

# Example to see the board with highlighted moves
tmux capture-pane -t %6 -p -e | head -40
```

**CRITICAL**: When using tmux automation tools (like MCP tmux tools), use the native capture functionality with color support, NOT command execution to run the capture command. The difference:
- ✅ CORRECT: Use the capture tool directly with color flag enabled
- ❌ WRONG: Execute `tmux capture-pane` as a shell command through command execution

This is essential because:
- Selected pieces are highlighted with bold borders (╔═════╗)
- **Possible moves are highlighted with green background color** (RGB: 120,140,100)
- Without color capture, these visual cues are invisible and you cannot see where you can move

## Automation Best Practices

### Step-by-Step Move Process for Agents

When automating moves, follow this exact sequence:

1. **Capture with colors** to see the current board state
2. **Navigate to your piece** using arrow keys
3. **Select the piece** with Space
4. **Capture with colors again** to see highlighted valid moves (green background)
5. **Navigate to a highlighted square** (only green squares are valid)
6. **Confirm the move** with Space

### Example Automation Workflow

```bash
# 1. Start the game in tmux pane %6
tmux send-keys -t %6 'cargo run --release' Enter

# 2. Wait for welcome screen, then start game
tmux send-keys -t %6 Enter

# 3. Capture board WITH COLORS to see pieces and valid moves
# Use your tool's native capture with color support, not shell commands

# 4. Make a move (example: piece at A3 to B4)
tmux send-keys -t %6 Space              # Select piece at A3
# NOW CAPTURE WITH COLORS to see green highlighted valid moves
tmux send-keys -t %6 Up Right Space     # Move to B4 and confirm
```

### Common Automation Mistakes

1. **Not capturing with colors** - You won't see valid moves
2. **Using shell commands for capture** instead of native tool functions
3. **Not checking for highlighted moves** after selecting a piece
4. **Trying to move to non-highlighted squares** - Move will be rejected

## Efficient Keyboard Input (for automation/agents)

When automating gameplay through tmux, you can send multiple key combinations at once for efficiency:

```bash
# Send multiple arrow keys in one command
tmux send-keys -t <pane-id> Up Up Right Right

# Navigate diagonally with combined commands
tmux send-keys -t <pane-id> Down Down Down Left Left Space

# Example: Move from A3 to C5 in one command
tmux send-keys -t <pane-id> Right Right Up Up Space
```

This is much more efficient than sending individual keystrokes:
- Combine multiple directional movements into one command
- Add the selection/confirmation key (Space) at the end
- Reduces latency and speeds up automation

## Notes

- The game maintains a clean, focused interface with no menus or configuration screens
- All game options are determined by environment variables at startup
- Board cells are enlarged for better visibility in terminal
- Red pieces start at top (rows 0-2), Black at bottom (rows 5-7)
- The game uses RGB colors which require true color terminal support