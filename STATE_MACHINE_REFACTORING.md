# State Machine Architecture Refactoring V2

## Изменения относительно первой версии

1. **Синхронный AI** - убираем async/await, AI блокирует игру как сейчас
2. **Поэтапная декомпозиция** - сначала разделяем данные, потом внедряем State Machine
3. **Минимальные изменения в core** - сохраняем существующую логику
4. **Курсор остаётся в UI** - избегаем проблем с владением

## План декомпозиции CheckersGame

### Текущая структура (монолит)
```rust
pub struct CheckersGame {
    // Core game state
    pub board: Board,
    pub current_player: Color,
    pub is_game_over: bool,
    pub move_history: MoveHistory,
    
    // UI state (не должно быть здесь)
    pub selected_piece: Option<(usize, usize)>,
    pub possible_moves: Option<Vec<(usize, usize)>>,
    
    // AI state (не должно быть здесь)
    pub ai_thinking: bool,
    pub ai_error: Option<String>,
}
```

### Шаг 1: Разделение на три структуры
```rust
// core/game.rs - только игровая логика
pub struct Game {
    pub board: Board,
    pub current_player: Color,
    pub is_game_over: bool,
    pub move_history: MoveHistory,
}

// state/ui_state.rs
pub struct UIState {
    pub selected_piece: Option<(usize, usize)>,
    pub possible_moves: Vec<(usize, usize)>,
    pub cursor_pos: (usize, usize), // переносим из UI
}

// state/ai_state.rs
pub struct AIState {
    pub is_thinking: bool,
    pub last_error: Option<String>,
}
```

### Шаг 2: Создание GameSession (временная структура)
```rust
// Промежуточная структура для плавного перехода
pub struct GameSession {
    pub game: Game,
    pub ui_state: UIState,
    pub ai_state: AIState,
    pub hint: Option<Hint>,
}
```

## Обновлённая структура файлов

```
src/
├── core/              # Минимальные изменения
│   ├── board.rs       
│   ├── piece.rs       
│   ├── game.rs        # Убираем UI/AI поля
│   ├── game_logic.rs  # Без изменений
│   └── mod.rs
│
├── state/             # НОВЫЙ модуль
│   ├── mod.rs         
│   ├── machine.rs     # StateMachine
│   ├── transition.rs  # StateTransition enum
│   ├── ui_state.rs    # UIState struct
│   ├── ai_state.rs    # AIState struct
│   ├── game_session.rs # Временная структура
│   └── states/        
│       ├── mod.rs
│       ├── welcome.rs        # WelcomeState
│       ├── playing.rs        # PlayingState
│       ├── piece_selected.rs # PieceSelectedState  
│       ├── ai_turn.rs        # AITurnState (синхронный)
│       ├── multi_capture.rs  # MultiCaptureState
│       └── game_over.rs      # GameOverState
│
├── interface/         
│   ├── mod.rs
│   ├── ui_ratatui.rs  # Адаптируем под ViewData
│   ├── view_data.rs   # НОВЫЙ - ViewData struct
│   └── widgets/       # Минимальные изменения
│
└── main.rs            # Постепенная миграция
```

## Обновлённые состояния

### 1. State trait (упрощённый, без async)
```rust
pub trait State {
    fn handle_input(&mut self, session: &mut GameSession, key: KeyEvent) 
        -> StateTransition;
    
    fn on_enter(&mut self, session: &mut GameSession);
    
    fn on_exit(&mut self, session: &mut GameSession);
    
    fn get_view_data(&self, session: &GameSession) -> ViewData;
    
    fn name(&self) -> &'static str; // для отладки
}
```

### 2. ViewData (адаптер для UI)
```rust
pub struct ViewData {
    // Прямые ссылки для эффективности
    pub board: &Board,
    pub current_player: Color,
    pub cursor_pos: (usize, usize),
    pub selected_piece: Option<(usize, usize)>,
    pub possible_moves: &[Move],
    
    // UI подсказки
    pub status_message: String,
    pub show_ai_thinking: bool,
    pub error_message: Option<&str>,
    
    // Дополнительная информация
    pub last_move: Option<&Move>,
    pub hint: Option<&Hint>,
    pub is_game_over: bool,
}
```

## Детализация состояний

### WelcomeState
```rust
pub struct WelcomeState {
    content: WelcomeContent, // загружается один раз
}

impl State for WelcomeState {
    fn handle_input(&mut self, _: &mut GameSession, key: KeyEvent) -> StateTransition {
        match key.code {
            KeyCode::Enter => StateTransition::To(Box::new(PlayingState::new())),
            KeyCode::Esc | KeyCode::Char('q') => StateTransition::Exit,
            _ => StateTransition::None,
        }
    }
}
```

### PlayingState
```rust
pub struct PlayingState;

impl State for PlayingState {
    fn handle_input(&mut self, session: &mut GameSession, key: KeyEvent) -> StateTransition {
        // Если ход AI - переходим в AITurnState
        if session.game.current_player == Color::Black && ai_enabled() {
            return StateTransition::To(Box::new(AITurnState::new()));
        }
        
        match key.code {
            KeyCode::Up => session.ui_state.move_cursor_up(),
            KeyCode::Space => {
                if let Some(piece) = get_piece_at_cursor(session) {
                    if piece.color == session.game.current_player {
                        return StateTransition::To(Box::new(
                            PieceSelectedState::new(cursor_pos)
                        ));
                    }
                }
            }
            // ...
        }
        
        StateTransition::None
    }
}
```

### PieceSelectedState
```rust
pub struct PieceSelectedState {
    selected_pos: (usize, usize),
}

impl State for PieceSelectedState {
    fn on_enter(&mut self, session: &mut GameSession) {
        session.ui_state.selected_piece = Some(self.selected_pos);
        session.ui_state.possible_moves = calculate_moves(&session.game, self.selected_pos);
    }
    
    fn handle_input(&mut self, session: &mut GameSession, key: KeyEvent) -> StateTransition {
        match key.code {
            KeyCode::Esc => StateTransition::To(Box::new(PlayingState::new())),
            KeyCode::Space => {
                let cursor = session.ui_state.cursor_pos;
                
                // Deselect if same piece
                if cursor == self.selected_pos {
                    return StateTransition::To(Box::new(PlayingState::new()));
                }
                
                // Try move
                if session.ui_state.possible_moves.contains(&cursor) {
                    match make_move(session, self.selected_pos, cursor) {
                        Ok(capture_continues) => {
                            if capture_continues {
                                StateTransition::To(Box::new(
                                    MultiCaptureState::new(cursor)
                                ))
                            } else {
                                check_game_over_transition(session)
                            }
                        }
                        Err(_) => StateTransition::None,
                    }
                } else {
                    StateTransition::None
                }
            }
            // Arrow keys for cursor movement...
        }
    }
}
```

### AITurnState (синхронный)
```rust
pub struct AITurnState {
    thinking_start: Instant,
}

impl State for AITurnState {
    fn on_enter(&mut self, session: &mut GameSession) {
        session.ai_state.is_thinking = true;
        session.ai_state.last_error = None;
    }
    
    fn handle_input(&mut self, session: &mut GameSession, _: KeyEvent) -> StateTransition {
        // AI ход выполняется прямо в handle_input (блокирующий)
        match get_ai_move(&session.game) {
            Ok(ai_move) => {
                make_move(session, ai_move.from, ai_move.to).ok();
                check_game_over_transition(session)
            }
            Err(e) => {
                session.ai_state.last_error = Some(e.to_string());
                StateTransition::To(Box::new(PlayingState::new()))
            }
        }
    }
    
    fn on_exit(&mut self, session: &mut GameSession) {
        session.ai_state.is_thinking = false;
    }
}
```

### MultiCaptureState
```rust
pub struct MultiCaptureState {
    capturing_piece: (usize, usize),
}

impl State for MultiCaptureState {
    fn on_enter(&mut self, session: &mut GameSession) {
        session.ui_state.selected_piece = Some(self.capturing_piece);
        session.ui_state.possible_moves = get_capture_moves(&session.game, self.capturing_piece);
    }
    
    fn handle_input(&mut self, session: &mut GameSession, key: KeyEvent) -> StateTransition {
        // Только захваты разрешены, ESC не работает
        match key.code {
            KeyCode::Space => {
                let cursor = session.ui_state.cursor_pos;
                if session.ui_state.possible_moves.contains(&cursor) {
                    make_move(session, self.capturing_piece, cursor).ok();
                    
                    // Проверяем продолжение захватов
                    if has_more_captures(&session.game, cursor) {
                        self.capturing_piece = cursor;
                        self.on_enter(session); // обновляем possible_moves
                        StateTransition::None
                    } else {
                        check_game_over_transition(session)
                    }
                } else {
                    StateTransition::None
                }
            }
            // Arrow keys...
        }
    }
}
```

## План миграции (поэтапный)

### Этап 1: Декомпозиция данных (без State Machine)
1. Создать `UIState` и `AIState` структуры
2. Создать `GameSession` как обёртку
3. Обновить `CheckersGame`, убрав UI/AI поля
4. Адаптировать main.rs для работы с `GameSession`
5. Запустить все тесты, убедиться что ничего не сломалось

### Этап 2: Подготовка инфраструктуры
1. Создать модуль `state` с базовыми трейтами
2. Создать `ViewData` структуру
3. Адаптировать UI для работы с `ViewData` вместо прямого доступа
4. Создать заглушки для всех состояний

### Этап 3: Параллельная работа
```rust
// main.rs - работают оба подхода
let mut session = GameSession::new();
let mut state_machine = StateMachine::new(WelcomeState::new());
let use_state_machine = env::var("USE_STATE_MACHINE").is_ok();

loop {
    if use_state_machine {
        let view = state_machine.get_view_data(&session);
        ui.render_view(terminal, view)?;
        
        if let Some(key) = get_key_event()? {
            state_machine.handle_input(&mut session, key);
        }
    } else {
        // Старый код с GameSession
        render_old_way(&session)?;
        handle_input_old_way(&mut session)?;
    }
}
```

### Этап 4: Миграция по состояниям
1. WelcomeState - самое простое, независимое
2. GameOverState - тоже простое
3. PlayingState - базовая навигация
4. AITurnState - изолированная AI логика
5. PieceSelectedState - сложнее, но управляемо
6. MultiCaptureState - самое сложное

### Этап 5: Удаление старого кода
1. Удалить старый game loop
2. Удалить `GameSession` (данные переедут в состояния)
3. Очистить main.rs до минимума
4. Обновить тесты под новую архитектуру

## Решение выявленных проблем

1. **Смешение ответственностей** → Декомпозиция на этапе 1
2. **Курсор в UI** → Остаётся в UI, передаётся в ViewData
3. **Синхронный AI** → AITurnState блокирует в handle_input
4. **Multi-capture** → Выделен в отдельное состояние
5. **ViewData** → Вводится постепенно на этапе 2
6. **Welcome Screen** → Первое состояние для миграции
7. **Hints** → Хранятся в GameSession/состояниях
8. **Неявные переходы** → Явные StateTransition enum
9. **Обработка ошибок** → Унификация через состояния
10. **Тестирование** → Поэтапная адаптация

## Преимущества обновлённого плана

1. **Минимальный риск** - поэтапная миграция с возможностью отката
2. **Сохранение логики** - core остаётся почти без изменений  
3. **Простота** - нет async сложностей
4. **Тестируемость** - можно тестировать параллельно
5. **Отладка** - ENV переменная для переключения