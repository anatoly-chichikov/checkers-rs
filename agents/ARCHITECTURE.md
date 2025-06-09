# Checkers-RS Architecture Overview

## Core Architectural Pattern

State Machine Architecture with **immutable state management**.

## Critical Design Decision: Immutable State Management

### The Power of Immutability

The architecture uses **immutable state updates** exclusively:

```rust
fn handle_input(&self, session: &GameSession, key: KeyEvent) -> (GameSession, StateTransition);
```

This means:
- States cannot modify themselves or session data
- All state changes create new instances
- Compile-time guarantees about data flow

### Why This Matters

1. **States cannot break invariants** - States can only return new data, not modify existing
2. **Perfect isolation of concerns** - Each state creates exactly the changes it needs
3. **Testing is trivial** - Just verify the returned state has expected values
4. **Refactoring is safe** - Compiler ensures all data flows are explicit

### The Architectural Contract (Compiler Enforced!)

States MUST follow these rules, enforced by the type system:

1. **WelcomeState** - Returns unmodified session or transitions to PlayingState
2. **PlayingState** - Returns session with cursor movements or piece selection
3. **PieceSelectedState** - Returns session with moves executed or deselection
4. **AITurnState** - Returns session with AI moves and hint updates
5. **MultiCaptureState** - Returns session with capture sequences completed
6. **GameOverState** - Returns unmodified session, only handles exit

**GUARANTEED**: States receive `&GameSession` and CANNOT modify it directly.

## Key Components and Their Actual Boundaries

### GameSession - The Mutable Aggregate
```rust
pub struct GameSession {
    pub game: CheckersGame,      // Core game state
    pub ui_state: UIState,       // UI-specific state
    pub ai_state: AIState,       // AI-specific state
    pub hint: Option<Hint>,      // Current hint
    pub welcome_content: String, // Welcome screen text
}
```

**Problem**: Everything is public and mutable when passed to states.

### State Trait - Pure Functional Design
```rust
pub trait State {
    fn handle_input(&self, session: &GameSession, key: KeyEvent) -> (GameSession, StateTransition);
    fn get_view_data(&self, session: &GameSession) -> ViewData;
    fn state_type(&self) -> StateType;
}
```

All methods take immutable references and return new values. No mutation possible.

### ViewData - The Only True Immutability
This is the ONLY place where immutability is enforced:
- UI receives ViewData (owned data or immutable references)
- UI cannot modify game state through ViewData
- This boundary IS actually safe

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

## Benefits of Immutable Architecture

### Advantages

1. **No State Corruption**: Impossible to accidentally modify wrong data
2. **Explicit Dependencies**: All data changes are visible in return values  
3. **Easy Maintenance**: Changes are localized and predictable
4. **Complete Type Safety**: Rust enforces immutability at compile time

### Implementation Details

1. **Clone for Updates**: All state structs implement Clone
2. **Pure Functions**: Methods take `&self` and return new instances
3. **Explicit Flow**: Session updates happen only in Application::run
4. **Performance**: Minimal overhead due to small state size

## Why This Architecture Is Superior

1. **Correctness**: Impossible to have race conditions or corruption
2. **Testability**: Pure functions are trivial to test
3. **Reasoning**: Easy to understand what each state does
4. **Debugging**: Can log/inspect every state transition

## Guidelines for Developers

### DO:
- Return new state instances from all methods
- Use helper methods like `with_ui_state` for efficient updates
- Compose small state changes into larger ones
- Trust the compiler to enforce correctness

### DON'T:
- Try to mutate state directly (compiler won't let you)
- Worry about performance (cloning is cheap for our use case)
- Add mutable fields to states
- Use RefCell or other interior mutability

## The Bottom Line

This architecture provides **both structure and safety**. The immutable State pattern enforces boundaries at compile time, making it impossible to violate architectural contracts. The code is more predictable, testable, and maintainable.