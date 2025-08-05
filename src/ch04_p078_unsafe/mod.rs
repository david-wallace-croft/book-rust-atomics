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

pub struct SpinLock<T> {
  locked: AtomicBool,
  value: UnsafeCell<T>,
}

impl<T> SpinLock<T> {
  // Note the use of const
  pub const fn new(value: T) -> Self {
    Self {
      locked: AtomicBool::new(false),
      value: UnsafeCell::new(value),
    }
  }

  pub fn lock(
    &self,
    i: T,
  ) -> &mut T
  where
    T: Display,
  {
    while self.locked.swap(true, Acquire) {
      info!("Thread {i}: spinning");

      hint::spin_loop();
    }

    unsafe { &mut *self.value.get() }
  }

  pub fn unlock(&self) {
    self.locked.store(false, Release);
  }
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

fn do_work<T>(
  i: T,
  spin_lock: &SpinLock<T>,
) where
  T: Copy + Display,
{
  let mut rng: ThreadRng = rand::rng();

  let millis: u64 = rng.random_range(0..1);

  thread::sleep(Duration::from_millis(millis));

  info!("Thread {i}: acquiring the lock");

  let value: &mut T = spin_lock.lock(i);

  *value = i;

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

    let spin_lock: &SpinLock<usize> = &SpinLock::new(0);

    thread::scope(|s: &Scope<'_, '_>| {
      for i in 1..5 {
        s.spawn(move || do_work(i, spin_lock));
      }
    });

    // SAFETY: We have exclusive access after all threads have joined
    info!("final value: {}", unsafe { *spin_lock.value.get() });
  }
}
