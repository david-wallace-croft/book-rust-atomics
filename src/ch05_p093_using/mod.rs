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
      self, AtomicBool, AtomicPtr, AtomicU8, AtomicU32, AtomicU64, AtomicUsize,
      Ordering::{Acquire, Relaxed, Release, SeqCst},
    },
  },
  thread::{self, JoinHandle, Scope, ScopedJoinHandle, Thread, ThreadId},
  time::{Duration, Instant},
};
use rand::prelude::*;
use tracing::info;

const EMPTY: u8 = 0;
const READING: u8 = 1;
const READY: u8 = 2;
const WRITING: u8 = 3;

pub struct Channel<T> {
  message: UnsafeCell<MaybeUninit<T>>,
  state: AtomicU8,
}

impl<T> Channel<T> {
  pub const fn new() -> Self {
    Self {
      message: UnsafeCell::new(MaybeUninit::uninit()),
      state: AtomicU8::new(EMPTY),
    }
  }

  pub fn is_ready(&self) -> bool {
    self.state.load(Relaxed) == READY
  }

  pub fn send(
    &self,
    message: T,
  ) {
    if self
      .state
      .compare_exchange(EMPTY, WRITING, Relaxed, Relaxed)
      .is_err()
    {
      info!("Panicking");

      panic!("can't send more than one message!");
    }

    unsafe {
      (*self.message.get()).write(message);
    }

    self.state.store(READY, Release);
  }

  pub fn receive(&self) -> T {
    if self
      .state
      .compare_exchange(READY, READING, Acquire, Relaxed)
      .is_err()
    {
      panic!("no message available!");
    }

    unsafe { (*self.message.get()).assume_init_read() }
  }
}

impl<T> Drop for Channel<T> {
  fn drop(&mut self) {
    if *self.state.get_mut() == READY {
      unsafe { self.message.get_mut().assume_init_drop() }
    }
  }
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    crate::init_tracing();

    let consumer_thread: Thread = thread::current();

    let channel: &Channel<&'static str> = &Channel::new();

    let message: &'static str = "Hello, World!";

    thread::scope(|s: &Scope<'_, '_>| {
      s.spawn(move || {
        thread::sleep(Duration::from_millis(100));

        info!("Producer thread sending message");

        channel.send(message);

        info!("Producer thread sent message");

        info!("Producer thread unparking consumer thread");

        consumer_thread.unpark();

        info!("Producer thread unparked consumer thread");
      });

      while !channel.is_ready() {
        info!("Channel is not ready; parking consumer thread");

        thread::park();

        info!("Consumer thread is unparked");
      }

      info!("Channel is ready");

      let received: &'static str = channel.receive();

      info!("Consumer thread received message: {received}");

      assert_eq!(message, received);
    });
  }
}
