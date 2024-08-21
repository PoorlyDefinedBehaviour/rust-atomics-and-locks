use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
};

struct Channel<T> {
    queue: Mutex<VecDeque<T>>,
    item_ready: Condvar,
}

impl<T> Channel<T> {
    fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            item_ready: Condvar::new(),
        }
    }

    fn send(&self, message: T) {
        self.queue.lock().unwrap().push_back(message);
        self.item_ready.notify_one();
    }

    fn receive(&self) -> T {
        let mut b = self.queue.lock().unwrap();
        loop {
            if let Some(v) = b.pop_front() {
                return v;
            }
            b = self.item_ready.wait(b).unwrap();
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::quickcheck;

    quickcheck! {
      fn property(messages: Vec<u8>) ->  bool {
        let channel = Channel::new();
        let messages_received = Mutex::new(Vec::new());

        std::thread::scope(|s|{
          let mut handles = Vec::new();
          for message in messages.iter(){
            {
              let channel = &channel;
              handles.push(s.spawn(move || channel.send(*message)));
            }

            {
              let channel = &channel;
              let messages_received = &messages_received;
              handles.push(s.spawn(move || messages_received.lock().unwrap().push(channel.receive())));
            }
          }
        });

        let mut expected = messages;
        expected.sort_unstable();
        let mut messages_received = messages_received.lock().unwrap();
        messages_received.sort_unstable();

        expected == *messages_received
      }
    }
}
