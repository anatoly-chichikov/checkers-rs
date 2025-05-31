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

pub fn start_loading_animation() -> Result<(Arc<AtomicBool>, thread::JoinHandle<()>), io::Error> {
    let mut stdout = io::stdout();
    stdout.execute(Hide)?;

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    let loading_thread = thread::spawn(move || {
        let spinner_frames = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
        let mut frame_idx = 0;
        let message = "Waiting for the magic...";

        while running_clone.load(Ordering::Relaxed) {
            let mut stdout = io::stdout();

            // Get terminal size and calculate position
            if let Ok((width, _)) = size() {
                // Calculate board dimensions
                let board_width = 3 + (7 * 8); // 59 chars total
                let board_offset = if width as usize > board_width {
                    (width as usize - board_width) / 2
                } else {
                    0
                };

                // Position aligned with board's right edge
                let message_len = message.len() + 2; // +2 for spinner and space
                let x_pos = board_offset + board_width - message_len - 1;
                let y_pos = 1; // Second row, below top border

                // Save cursor position, move to upper right, print, restore cursor
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

        // Clear the spinner when done
        let mut stdout = io::stdout();
        if let Ok((width, _)) = size() {
            // Calculate board dimensions
            let board_width = 3 + (7 * 8); // 59 chars total
            let board_offset = if width as usize > board_width {
                (width as usize - board_width) / 2
            } else {
                0
            };

            // Position aligned with board's right edge
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

pub fn stop_loading_animation(
    running: Arc<AtomicBool>,
    loading_thread: thread::JoinHandle<()>,
) -> Result<(), io::Error> {
    running.store(false, Ordering::Relaxed);
    let _ = loading_thread.join();

    let mut stdout = io::stdout();
    stdout.execute(Show)?;

    Ok(())
}
