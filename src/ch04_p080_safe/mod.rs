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

pub struct Guard<'a, T> {
  lock: &'a SpinLock<T>,
}

impl<T> Deref for Guard<'_, T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    unsafe { &*self.lock.value.get() }
  }
}

impl<T> DerefMut for Guard<'_, T> {
  fn deref_mut(&mut self) -> &mut T {
    unsafe { &mut *self.lock.value.get() }
  }
}

impl<T> Drop for Guard<'_, T> {
  fn drop(&mut self) {
    self.lock.locked.store(false, Release);
  }
}

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
  ) -> Guard<T>
  where
    T: Display,
  {
    while self.locked.swap(true, Acquire) {
      info!("Thread {i}: spinning");

      hint::spin_loop();
    }

    Guard {
      lock: self,
    }
  }

  // pub fn unlock(&self) {
  //   self.locked.store(false, Release);
  // }
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

  info!("Thread {i}: acquiring the guard");

  let mut guard: Guard<'_, T> = spin_lock.lock(i);

  *guard = i;

  info!("Thread {i}: acquired the guard");

  let millis: u64 = rng.random_range(0..1);

  thread::sleep(Duration::from_millis(millis));

  info!("Thread {i}: dropping the guard");

  drop(guard);

  info!("Thread {i}: dropped the guard");
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

    let guard: Guard<'_, usize> = spin_lock.lock(0);

    let value: usize = *guard;

    info!("final value: {value}");
  }
}
