use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
};

struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> SpinLock<T> {
    const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    #[allow(clippy::mut_from_ref)]
    fn lock<'a>(&'a self) -> &'a mut T {
        while self.locked.swap(true, Ordering::Acquire) {
            std::hint::spin_loop();
        }

        unsafe { &mut *self.value.get() }
    }

    fn release(&self) {
        self.locked.swap(false, Ordering::Release);
    }
}

fn main() {
    println!("Hello, world!");
}
