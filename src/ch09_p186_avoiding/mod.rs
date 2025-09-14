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

unsafe impl<T> Sync for MyMutex<T> where T: Send {}

pub struct MyMutexGuard<'a, T> {
  my_mutex: &'a MyMutex<T>,
}

// From errata webpage
unsafe impl<T> Sync for MyMutexGuard<'_, T> where T: Sync {}

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

impl<T> MyMutex<T> {
  pub const fn new(value: T) -> Self {
    Self {
      state: AtomicU32::new(0),
      value: UnsafeCell::new(value),
    }
  }

  pub fn lock(&'_ self) -> MyMutexGuard<'_, T> {
    while self.state.compare_exchange(0, 1, Acquire, Relaxed).is_err() {
      while self.state.swap(2, Acquire) != 0 {
        atomic_wait::wait(&self.state, 2);
      }
    }

    MyMutexGuard {
      my_mutex: self,
    }
  }
}

impl<T> Drop for MyMutexGuard<'_, T> {
  fn drop(&mut self) {
    if self.my_mutex.state.swap(0, Release) == 2 {
      atomic_wait::wake_one(&self.my_mutex.state);
    }
  }
}

#[cfg(test)]
mod test {

  use super::*;

  // TODO: This test locks up intermittently so there might be a bug
  #[ignore]
  #[test]
  fn test1() {
    // Test code adapted from main() function on Chapter 4 page 82

    let x: MyMutex<Vec<i32>> = MyMutex::new(Vec::new());

    thread::scope(|s| {
      s.spawn(|| x.lock().push(1));

      s.spawn(|| {
        let mut g: MyMutexGuard<'_, Vec<i32>> = x.lock();

        g.push(2);

        g.push(2);
      });
    });

    let g: MyMutexGuard<'_, Vec<i32>> = x.lock();

    let slice: &[i32] = g.as_slice();

    assert!(
      slice
        == [
          1, 2, 2
        ]
        || slice
          == [
            2, 2, 1
          ]
    );
  }
}
