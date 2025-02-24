use crossterm::event::{self, Event, KeyCode, KeyEvent};
use std::io;

#[derive(Debug, PartialEq)]
pub enum GameInput {
    MoveCursor(CursorDirection),
    Select,
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
                KeyCode::Char('q') | KeyCode::Esc => GameInput::Quit,
                _ => return Ok(None),
            }));
        }
    }
    Ok(None)
}
