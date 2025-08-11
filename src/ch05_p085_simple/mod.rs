#![allow(unused)]

use ::std::{
  any::Any,
  array,
  cell::{Cell, RefCell, UnsafeCell},
  collections::VecDeque,
  fmt::Display,
  hint, io,
  marker::PhantomData,
  mem::{ManuallyDrop, MaybeUninit},
  ops::{Deref, DerefMut},
  ptr,
  rc::Rc,
  sync::{
    Condvar, LazyLock, Mutex, MutexGuard, Once,
    atomic::{
      self, AtomicBool, AtomicPtr, AtomicU32, AtomicU64, AtomicUsize,
      Ordering::{Acquire, Relaxed, Release, SeqCst},
    },
  },
  thread::{self, JoinHandle, Scope, ScopedJoinHandle, Thread, ThreadId},
  time::{Duration, Instant},
};
use rand::prelude::*;
use tracing::info;

pub struct Channel<T> {
  queue: Mutex<VecDeque<T>>,
  item_ready: Condvar,
}

impl<T> Channel<T> {
  pub fn new() -> Self {
    Self {
      queue: Mutex::new(VecDeque::new()),
      item_ready: Condvar::new(),
    }
  }

  pub fn send(
    &self,
    message: T,
  ) {
    self.queue.lock().unwrap().push_back(message);

    self.item_ready.notify_one();
  }

  pub fn receive(&self) -> T {
    let mut b: MutexGuard<'_, VecDeque<T>> = self.queue.lock().unwrap();

    loop {
      if let Some(message) = b.pop_front() {
        return message;
      }

      b = self.item_ready.wait(b).unwrap();
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    crate::init_tracing();

    let channel: &Channel<usize> = &Channel::new();

    thread::scope(|s: &Scope<'_, '_>| {
      s.spawn(move || {
        let mut rng: ThreadRng = rand::rng();

        for message in 1..=3 {
          let millis: u64 = rng.random_range(1..=100);

          thread::sleep(Duration::from_millis(millis));

          info!("Producer thread sending message {message}");

          channel.send(message);

          info!("Producer thread sent message {message}");
        }

        channel.send(0);
      });

      s.spawn(move || {
        loop {
          let message: usize = channel.receive();

          info!("Consumer thread received message {message}");

          if message == 0 {
            break;
          }
        }
      });
    });
  }
}
