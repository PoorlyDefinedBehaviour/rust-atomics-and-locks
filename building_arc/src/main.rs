use std::{ops::Deref, sync::atomic::AtomicUsize};

struct ArcData<T> {
    ref_count: AtomicUsize,
    data: T,
}

struct Arc<T> {
    ptr: std::ptr::NonNull<ArcData<T>>,
}

unsafe impl<T: Send + Sync> Send for Arc<T> {}
unsafe impl<T: Send + Sync> Sync for Arc<T> {}

impl<T> Arc<T> {
    fn new(data: T) -> Self {
        Self {
            ptr: std::ptr::NonNull::from(Box::leak(Box::new(ArcData {
                ref_count: AtomicUsize::new(1),
                data,
            }))),
        }
    }

    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data().data
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        if self
            .data()
            .ref_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            > usize::MAX / 2
        {
            std::process::abort();
        }
        Arc { ptr: self.ptr }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self
            .data()
            .ref_count
            .fetch_sub(1, std::sync::atomic::Ordering::Release)
            == 1
        {
            std::sync::atomic::fence(std::sync::atomic::Ordering::Acquire);
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
