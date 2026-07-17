use std::sync::atomic::{AtomicBool, Ordering};

static NEWLINE_NEEDED: AtomicBool = AtomicBool::new(false);

pub fn used_print() {
    NEWLINE_NEEDED.store(true, Ordering::Relaxed);
}

pub fn used_println() {
    NEWLINE_NEEDED.store(false, Ordering::Relaxed);
}

pub fn take_newline_needed() -> bool {
    NEWLINE_NEEDED.swap(false, Ordering::Relaxed)
}