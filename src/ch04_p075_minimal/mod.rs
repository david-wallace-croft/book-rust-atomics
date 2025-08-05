#![allow(unused)]

use ::std::{
  any::Any,
  array,
  cell::{Cell, RefCell, UnsafeCell},
  collections::VecDeque,
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

pub struct SpinLock {
  locked: AtomicBool,
}

impl SpinLock {
  // Note the use of const
  pub const fn new() -> Self {
    Self {
      locked: AtomicBool::new(false),
    }
  }

  pub fn lock(
    &self,
    i: usize,
  ) {
    while self.locked.swap(true, Acquire) {
      info!("Thread {i}: spinning");

      hint::spin_loop();
    }
  }

  pub fn unlock(&self) {
    self.locked.store(false, Release);
  }
}

fn do_work(
  i: usize,
  spin_lock: &SpinLock,
) {
  let mut rng: ThreadRng = rand::rng();

  let millis: u64 = rng.random_range(0..1);

  thread::sleep(Duration::from_millis(millis));

  info!("Thread {i}: acquiring the lock");

  spin_lock.lock(i);

  info!("Thread {i}: acquired the lock");

  let millis: u64 = rng.random_range(0..1);

  thread::sleep(Duration::from_millis(millis));

  info!("Thread {i}: releasing the lock");

  spin_lock.unlock();

  info!("Thread {i}: released the lock");
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    crate::init_tracing();

    let spin_lock: &SpinLock = &SpinLock::new();

    thread::scope(|s: &Scope<'_, '_>| {
      for i in 0..4 {
        s.spawn(move || do_work(i, spin_lock));
      }
    });
  }
}
