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
  process,
  ptr::{self, NonNull},
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

pub struct MyMutex<T> {
  state: AtomicU32,
  value: UnsafeCell<T>,
}

impl<T> MyMutex<T> {
  pub const fn new(value: T) -> Self {
    Self {
      state: AtomicU32::new(0),
      value: UnsafeCell::new(value),
    }
  }

  pub fn lock(&'_ self) -> MyMutexGuard<'_, T> {
    if self.state.compare_exchange(0, 1, Acquire, Relaxed).is_err() {
      Self::lock_contended(&self.state);
    }

    MyMutexGuard {
      my_mutex: self,
    }
  }

  fn lock_contended(state: &AtomicU32) {
    let mut spin_count = 0;

    while state.load(Relaxed) == 1 && spin_count < 100 {
      spin_count += 1;

      hint::spin_loop();
    }

    if state.compare_exchange(0, 1, Acquire, Relaxed).is_ok() {
      return;
    }

    while state.swap(2, Acquire) != 0 {
      atomic_wait::wait(state, 2);
    }
  }
}

unsafe impl<T> Sync for MyMutex<T> where T: Send {}

pub struct MyMutexGuard<'a, T> {
  my_mutex: &'a MyMutex<T>,
}

impl<T> Deref for MyMutexGuard<'_, T> {
  type Target = T;

  fn deref(&self) -> &T {
    unsafe { &*self.my_mutex.value.get() }
  }
}

impl<T> DerefMut for MyMutexGuard<'_, T> {
  fn deref_mut(&mut self) -> &mut T {
    unsafe { &mut *self.my_mutex.value.get() }
  }
}

impl<T> Drop for MyMutexGuard<'_, T> {
  fn drop(&mut self) {
    if self.my_mutex.state.swap(0, Release) == 2 {
      atomic_wait::wake_one(&self.my_mutex.state);
    }
  }
}

// From errata webpage
unsafe impl<T> Sync for MyMutexGuard<'_, T> where T: Sync {}

#[cfg(test)]
mod test {

  use super::*;

  #[test]
  fn test1() {
    crate::init_tracing();

    let m: MyMutex<i32> = MyMutex::new(0);

    hint::black_box(&m);

    let start: Instant = Instant::now();

    for _ in 0..5_000_000 {
      *m.lock() += 1;
    }

    let duration: Duration = start.elapsed();

    info!("locked {} times in {:?}", *m.lock(), duration);
  }

  #[test]
  fn test2() {
    crate::init_tracing();

    let m: MyMutex<i32> = MyMutex::new(0);

    hint::black_box(&m);

    let start: Instant = Instant::now();

    thread::scope(|s| {
      for _ in 0..4 {
        s.spawn(|| {
          for _ in 0..5_000_000 {
            *m.lock() += 1;
          }
        });
      }
    });

    let duration: Duration = start.elapsed();

    info!("locked {} times in {:?}", *m.lock(), duration);
  }
}
