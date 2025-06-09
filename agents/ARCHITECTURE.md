# Checkers-RS Architecture Overview

## Core Architectural Pattern

State Machine Architecture with **immutable state management** at the application level.

## Critical Design Decision: Immutable State Management

### Current Implementation Status

The architecture successfully implements immutable state updates at the state machine level:

```rust
fn handle_input(&self, session: &GameSession, key: KeyEvent) -> (GameSession, StateTransition);
```

This means:
- States cannot modify themselves or session data directly
- All state changes create new `GameSession` instances
- Compile-time guarantees about data flow at the state level
- Mixed approach: immutable at high level, some mutability at component level

### Implementation Details

#### ✅ Fully Immutable Components:
1. **State Trait** - Uses `&self` and returns new instances
2. **GameSession** - Implements `Clone`, all methods return `Self`
3. **CheckersGame::make_move** - Takes `&self`, returns `Result<(Self, bool), GameError>`
4. **All State Implementations** - Follow pure functional pattern

#### ⚠️ Components with Internal Mutability:
1. **Board** - Has methods like `set_piece(&mut self)`, `move_piece(&mut self)`
2. **MoveHistory** - Has `add_move(&mut self)`
3. **Piece** - Has `promote_to_king(&mut self)`

These are used internally within `CheckersGame::make_move` after cloning.

### Why This Hybrid Approach Works

1. **High-level immutability** - Game state transitions are predictable and safe
2. **Performance optimization** - Internal mutability avoids unnecessary allocations
3. **Testing remains simple** - State transitions are still pure functions
4. **Encapsulation** - Mutable operations are hidden within immutable interfaces

### The Architectural Contract

States MUST follow these rules, enforced by the type system:

1. **WelcomeState** - Returns unmodified session or transitions to PlayingState
2. **PlayingState** - Returns session with cursor movements or piece selection
3. **PieceSelectedState** - Returns session with moves executed or deselection
4. **AITurnState** - Returns session with AI moves and hint updates
5. **MultiCaptureState** - Returns session with capture sequences completed
6. **GameOverState** - Returns unmodified session, only handles exit

**GUARANTEED**: States receive `&GameSession` and CANNOT modify it directly.

## Key Components and Their Boundaries

### GameSession - The Immutable Aggregate
```rust
#[derive(Clone)]
pub struct GameSession {
    pub game: CheckersGame,                    // Core game state
    pub ui_state: UIState,                    // UI-specific state
    pub ai_state: AIState,                    // AI-specific state
    pub hint: Option<Hint>,                   // Current hint
    pub hint_provider: Option<HintProvider>,  // AI hint provider
    pub welcome_content: Option<WelcomeContent>, // Welcome screen content
}
```

**Design**: All fields are public but states receive `&GameSession` (immutable reference), preventing direct modification. Updates happen through methods that return new instances.

### State Trait - Pure Functional Design
```rust
pub trait State {
    fn handle_input(&self, session: &GameSession, key: KeyEvent) -> (GameSession, StateTransition);
    fn get_view_data<'a>(&self, session: &'a GameSession) -> ViewData<'a>;
    fn state_type(&self) -> StateType;
}
```

All methods take immutable references and return new values. No mutation possible. The lifetime parameter in `get_view_data` ensures safe borrowing for the view layer.

### ViewData - Safe View Layer Boundary
Provides a read-only view of the game state:
- UI receives ViewData with borrowed references to game state
- UI cannot modify game state through ViewData
- Lifetime parameters ensure memory safety
- Clear separation between state management and presentation

## Actual Data Flow

```
User Input 
    ↓
State Machine (holds immutable current_state)
    ↓
State::handle_input(&self, &session, key)
    ↓
State returns (new_session, transition) ← PURE FUNCTION
    ↓
Application updates its session = new_session
    ↓
State::get_view_data(&self, &session) 
    ↓
ViewData (immutable snapshot)
    ↓
UI renders
```

Every step is immutable and predictable!

## Benefits of the Hybrid Immutable Architecture

### Advantages

1. **No State Corruption**: State-level immutability prevents accidental modifications
2. **Explicit Dependencies**: All state changes are visible in return values  
3. **Easy Maintenance**: Changes are localized and predictable
4. **Type Safety**: Rust enforces immutability at the state machine level
5. **Performance**: Internal mutability optimizes hot paths without sacrificing safety

### Implementation Details

1. **Clone for Updates**: GameSession and high-level structs implement Clone
2. **Pure State Functions**: State methods take `&self` and return new GameSession
3. **Helper Methods**: Methods like `with_ui_state()` provide efficient updates
4. **Explicit Flow**: Session updates happen only in Application::run
5. **Internal Optimization**: Components like Board use mutability after cloning

## Why This Architecture Is Superior

1. **Correctness**: Impossible to have race conditions or corruption
2. **Testability**: Pure functions are trivial to test
3. **Reasoning**: Easy to understand what each state does
4. **Debugging**: Can log/inspect every state transition

## Guidelines for Developers

### DO:
- Return new GameSession instances from state handlers
- Use helper methods like `with_ui_state()`, `select_piece()`, `make_move()`
- Clone at the appropriate level (GameSession for states, components internally)
- Follow the existing pattern: immutable interfaces, optimized internals
- Trust the compiler to enforce state-level immutability

### DON'T:
- Try to mutate GameSession directly in states (compiler won't let you)
- Add mutable methods to GameSession or State trait
- Worry about cloning GameSession (it's optimized for this use case)
- Use RefCell or other interior mutability at the state level
- Break the immutable contract at the state machine boundary

## The Bottom Line

This hybrid architecture provides **both safety and performance**:
- **Safety**: Immutable state transitions at the application level
- **Performance**: Optimized internal operations where needed
- **Predictability**: Pure functions at state boundaries
- **Testability**: State transitions are easily testable
- **Pragmatism**: Balances ideal design with practical performance

The immutable State pattern enforces boundaries at compile time, making it impossible to violate architectural contracts at the state level, while allowing performance optimizations internally.