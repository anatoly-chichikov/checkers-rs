# Checkers Game States

This document contains three key states of the checkers game.

## 1. Welcome State

**Description**: Initial screen shown when game starts
**File**: [states/welcome_state.txt](states/welcome_state.txt)

Shows the CHECKERS ASCII art logo, "Did You Know?" section, "Tip of the Day", and "Today's Challenge".

## 2. Initial Board State

**Description**: Game board at start with all pieces in starting positions
**File**: [states/initial_board_state.txt](states/initial_board_state.txt)

Shows the 8x8 board with:
- White pieces (w) in rows 1-3
- Black pieces (b) in rows 6-8
- Cursor at position A3
- "Current Turn: White" status

## 3. After First Move with AI Hint

**Description**: Board state after completing first move, showing AI hint
**File**: [states/after_move_with_hint_state.txt](states/after_move_with_hint_state.txt)

Shows the board after a move from C3 to D4, with an AI hint box at the bottom suggesting the next move from D4 to B6.
