use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::io;

#[derive(Debug, PartialEq)]
pub enum GameInput {
    MoveCursor(CursorDirection),
    Select,
    Hint,
    Quit,
}

#[derive(Debug, PartialEq)]
pub enum CursorDirection {
    Up,
    Down,
    Left,
    Right,
}

pub fn read_input() -> io::Result<Option<GameInput>> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            return Ok(Some(match code {
                KeyCode::Up => GameInput::MoveCursor(CursorDirection::Up),
                KeyCode::Down => GameInput::MoveCursor(CursorDirection::Down),
                KeyCode::Left => GameInput::MoveCursor(CursorDirection::Left),
                KeyCode::Right => GameInput::MoveCursor(CursorDirection::Right),
                KeyCode::Char(' ') | KeyCode::Enter => GameInput::Select,
                KeyCode::Char('h') | KeyCode::Char('H') => GameInput::Hint,
                KeyCode::Char('q')
                | KeyCode::Char('Q')
                | KeyCode::Char('й')
                | KeyCode::Char('Й')
                | KeyCode::Char('a')
                | KeyCode::Char('A')
                | KeyCode::Char('ק')
                | KeyCode::Char('ض')
                | KeyCode::Char('θ')
                | KeyCode::Char('Θ')
                | KeyCode::Esc => GameInput::Quit,
                _ => return Ok(None),
            }));
        }
    }
    Ok(None)
}
