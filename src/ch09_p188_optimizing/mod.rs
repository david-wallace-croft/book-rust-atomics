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
  pub(crate) my_mutex: &'a MyMutex<T>,
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
