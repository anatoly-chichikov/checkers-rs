use crossterm::{
    cursor::{Hide, MoveTo, Show},
    style::Print,
    terminal::{size, Clear, ClearType},
    ExecutableCommand,
};
use std::{
    io::{self, Write},
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
    thread,
    time::Duration,
};

/// Starts spinner with default message.
pub fn animate_start() -> Result<(Arc<AtomicBool>, thread::JoinHandle<()>), io::Error> {
    animate_start_with_message("Waiting for the magic...")
}

/// Starts spinner with provided message.
pub fn animate_start_with_message(
    message: &'static str,
) -> Result<(Arc<AtomicBool>, thread::JoinHandle<()>), io::Error> {
    let mut stdout = io::stdout();
    stdout.execute(Hide)?;
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    let loading_thread = thread::spawn(move || {
        let spinner_frames = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
        let mut frame_idx = 0;
        while running_clone.load(Ordering::Relaxed) {
            let mut stdout = io::stdout();
            if let Ok((width, _)) = size() {
                let board_width = 3 + (7 * 8);
                let board_offset = if width as usize > board_width {
                    (width as usize - board_width) / 2
                } else {
                    0
                };
                let message_len = message.len() + 2;
                let x_pos = board_offset + board_width - message_len - 1;
                let y_pos = 1;
                let _ = stdout.execute(crossterm::cursor::SavePosition);
                let _ = stdout.execute(MoveTo(x_pos as u16, y_pos));
                let _ = stdout.execute(Clear(ClearType::UntilNewLine));
                let _ = stdout.execute(Print(format!("{} {}", spinner_frames[frame_idx], message)));
                let _ = stdout.execute(crossterm::cursor::RestorePosition);
                let _ = stdout.flush();
            }
            frame_idx = (frame_idx + 1) % spinner_frames.len();
            thread::sleep(Duration::from_millis(100));
        }
        let mut stdout = io::stdout();
        if let Ok((width, _)) = size() {
            let board_width = 3 + (7 * 8);
            let board_offset = if width as usize > board_width {
                (width as usize - board_width) / 2
            } else {
                0
            };
            let message_len = message.len() + 2;
            let x_pos = board_offset + board_width - message_len - 1;
            let _ = stdout.execute(crossterm::cursor::SavePosition);
            let _ = stdout.execute(MoveTo(x_pos as u16, 1));
            let _ = stdout.execute(Clear(ClearType::UntilNewLine));
            let _ = stdout.execute(crossterm::cursor::RestorePosition);
            let _ = stdout.flush();
        }
    });
    Ok((running, loading_thread))
}

/// Stops spinner and restores cursor visibility.
pub fn animate_stop(
    running: Arc<AtomicBool>,
    loading_thread: thread::JoinHandle<()>,
) -> Result<(), io::Error> {
    running.store(false, Ordering::Relaxed);
    let _ = loading_thread.join();
    let mut stdout = io::stdout();
    stdout.execute(Show)?;
    Ok(())
}
