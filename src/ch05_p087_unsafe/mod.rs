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
  message: UnsafeCell<MaybeUninit<T>>,
  ready: AtomicBool,
}

impl<T> Channel<T> {
  pub fn new() -> Self {
    Self {
      message: UnsafeCell::new(MaybeUninit::uninit()),
      ready: AtomicBool::new(false),
    }
  }

  pub fn is_ready(&self) -> bool {
    self.ready.load(Acquire)
  }

  pub unsafe fn send(
    &self,
    message: T,
  ) {
    unsafe {
      (*self.message.get()).write(message);
    }

    self.ready.store(true, Release);
  }

  pub unsafe fn receive(&self) -> T {
    unsafe { (*self.message.get()).assume_init_read() }
  }
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    crate::init_tracing();

    let channel: &Channel<usize> = &Channel::new();

    thread::scope(|s: &Scope<'_, '_>| {
      s.spawn(move || {
        thread::sleep(Duration::from_millis(100));

        let message: usize = 1;

        info!("Producer thread sending message {message}");

        unsafe {
          channel.send(message);
        }

        info!("Producer thread sent message {message}");
      });

      s.spawn(move || {
        loop {
          if channel.is_ready() {
            let message: usize = unsafe { channel.receive() };

            info!("Consumer thread received message {message}");

            break;
          }

          hint::spin_loop();
        }
      });
    });
  }
}
