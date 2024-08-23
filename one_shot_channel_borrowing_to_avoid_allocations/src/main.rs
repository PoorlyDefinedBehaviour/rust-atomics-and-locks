use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::atomic::{AtomicBool, Ordering},
};

struct Sender<'a, T> {
    channel: &'a Channel<T>,
}

struct Receiver<'a, T> {
    channel: &'a Channel<T>,
}

impl<'a, T> Sender<'a, T> {
    fn send(self, message: T) {}
}

impl<'a, T> Receiver<'a, T> {
    fn is_ready(&self) -> bool {
        self.channel.is_ready()
    }

    fn receive(self) -> T {
        self.channel.receive()
    }
}

struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }

    fn split<'a>(&'a mut self) -> (Sender<'a, T>, Receiver<'a, T>) {
        *self = Self::new();
        (Sender { channel: self }, Receiver { channel: self })
    }

    // SAFETY: Only call this once!
    fn send(&self, message: T) {
        unsafe { (*self.message.get()).write(message) };
        self.ready.store(true, Ordering::Release);
    }

    fn is_ready(&self) -> bool {
        self.ready.load(Ordering::Relaxed)
    }

    // SAFETY: Only call this once and after is_ready returns true.
    fn receive(&self) -> T {
        assert!(
            self.ready.swap(false, Ordering::Acquire),
            "no message available"
        );
        unsafe { (*self.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe {
                self.message.get_mut().assume_init_drop();
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
