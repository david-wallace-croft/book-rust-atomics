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
    Arc, Condvar, LazyLock, Mutex, MutexGuard, Once,
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

pub struct Channel<T> {
  message: UnsafeCell<MaybeUninit<T>>,
  ready: AtomicBool,
}

impl<T> Channel<T> {
  pub const fn new() -> Self {
    Self {
      message: UnsafeCell::new(MaybeUninit::uninit()),
      ready: AtomicBool::new(false),
    }
  }

  pub fn split<'a>(&'a mut self) -> (Sender<'a, T>, Receiver<'a, T>) {
    *self = Self::new();

    (
      Sender {
        channel: self,
      },
      Receiver {
        channel: self,
      },
    )
  }
}

impl<T> Drop for Channel<T> {
  fn drop(&mut self) {
    if *self.ready.get_mut() {
      unsafe { self.message.get_mut().assume_init_drop() }
    }
  }
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

pub struct Receiver<'a, T> {
  channel: &'a Channel<T>,
}

impl<T> Receiver<'_, T> {
  pub fn is_ready(&self) -> bool {
    self.channel.ready.load(Relaxed)
  }

  pub fn receive(self) -> T {
    if !self.channel.ready.swap(false, Acquire) {
      panic!("no message available!");
    }

    unsafe { (*self.channel.message.get()).assume_init_read() }
  }
}

pub struct Sender<'a, T> {
  channel: &'a Channel<T>,
}

impl<T> Sender<'_, T> {
  pub fn send(
    self,
    message: T,
  ) {
    unsafe {
      (*self.channel.message.get()).write(message);
    }

    self.channel.ready.store(true, Release);
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    crate::init_tracing();

    let mut channel: Channel<&'static str> = Channel::new();

    thread::scope(|s: &Scope<'_, '_>| {
      let (sender, receiver) = channel.split();

      let consumer_thread: Thread = thread::current();

      const MESSAGE: &'static str = "Hello, World!";

      s.spawn(move || {
        thread::sleep(Duration::from_millis(100));

        info!("Producer thread sending message");

        sender.send(MESSAGE);

        info!("Producer thread sent message");

        info!("Producer thread unparking consumer thread");

        consumer_thread.unpark();

        info!("Producer thread unparked consumer thread");
      });

      while !receiver.is_ready() {
        info!("Channel is not ready; parking consumer thread");

        thread::park();

        info!("Consumer thread is unparked");
      }

      info!("Channel is ready");

      let received: &'static str = receiver.receive();

      info!("Consumer thread received message: {received}");

      assert_eq!(MESSAGE, received);
    });
  }
}
