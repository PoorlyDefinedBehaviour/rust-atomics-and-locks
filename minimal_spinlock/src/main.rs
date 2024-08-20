use std::sync::atomic::{AtomicBool, Ordering};

struct SpinLock {
    locked: AtomicBool,
}

impl SpinLock {
    fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
        }
    }

    fn lock(&self) {
        while self.locked.swap(true, Ordering::Acquire) {
            std::hint::spin_loop();
        }
    }

    fn release(&self) {
        self.locked.swap(false, Ordering::Release);
    }
}

fn main() {
    println!("Hello, world!");
}
