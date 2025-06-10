use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use ctrlc;
use libc;

#[test]
#[ignore] // This test sends SIGINT to the current process, which can disrupt
          // the test runner or other parallel tests, especially in CI.
          // It tests global state and is better suited for manual/integration testing.
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
