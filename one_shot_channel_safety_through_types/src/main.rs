use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let chan = Arc::new(Channel::new());
    (
        Sender {
            channel: Arc::clone(&chan),
        },
        Receiver {
            channel: Arc::clone(&chan),
        },
    )
}

struct Sender<T> {
    channel: Arc<Channel<T>>,
}

struct Receiver<T> {
    channel: Arc<Channel<T>>,
}

impl<T> Sender<T> {
    fn send(self, message: T) {}
}

impl<T> Receiver<T> {
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
