use crossterm::{
    cursor::{Hide, Show},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::{
    io,
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
        while running_clone.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_millis(100));
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
    stdout.execute(Clear(ClearType::CurrentLine))?;
    stdout.execute(Show)?;

    Ok(())
}
