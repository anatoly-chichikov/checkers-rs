use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use ctrlc;
use libc;

#[test]
fn test_ctrl_c_handler() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    assert!(running.load(Ordering::SeqCst));
    unsafe {
        libc::raise(libc::SIGINT);
    }
    thread::sleep(Duration::from_millis(100));
    assert!(!running.load(Ordering::SeqCst));
}
