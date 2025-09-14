#![allow(unused)]

use super::ch09_p188_optimizing::MyMutex;
use super::ch09_p188_optimizing::MyMutexGuard;
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

pub struct MyReadGuard<'a, T> {
  my_rw_lock: &'a MyRwLock<T>,
}

impl<T> Deref for MyReadGuard<'_, T> {
  type Target = T;

  fn deref(&self) -> &T {
    unsafe { &*self.my_rw_lock.value.get() }
  }
}

impl<T> Drop for MyReadGuard<'_, T> {
  fn drop(&mut self) {
    if self.my_rw_lock.state.fetch_sub(1, Release) == 1 {
      atomic_wait::wake_one(&self.my_rw_lock.state);
    }
  }
}

pub struct MyWriteGuard<'a, T> {
  my_rw_lock: &'a MyRwLock<T>,
}

impl<T> Deref for MyWriteGuard<'_, T> {
  type Target = T;

  fn deref(&self) -> &T {
    unsafe { &*self.my_rw_lock.value.get() }
  }
}

impl<T> DerefMut for MyWriteGuard<'_, T> {
  fn deref_mut(&mut self) -> &mut T {
    unsafe { &mut *self.my_rw_lock.value.get() }
  }
}

impl<T> Drop for MyWriteGuard<'_, T> {
  fn drop(&mut self) {
    self.my_rw_lock.state.store(0, Release);

    atomic_wait::wake_all(&self.my_rw_lock.state);
  }
}

pub struct MyRwLock<T> {
  state: AtomicU32,
  value: UnsafeCell<T>,
}

impl<T> MyRwLock<T> {
  pub const fn new(value: T) -> Self {
    Self {
      state: AtomicU32::new(0),
      value: UnsafeCell::new(value),
    }
  }

  pub fn read(&'_ self) -> MyReadGuard<'_, T> {
    let mut s: u32 = self.state.load(Relaxed);

    loop {
      if s < u32::MAX {
        assert!(s != u32::MAX - 1, "too many readers");

        match self.state.compare_exchange_weak(s, s + 1, Acquire, Relaxed) {
          Ok(_) => {
            return MyReadGuard {
              my_rw_lock: self,
            };
          },
          Err(e) => s = e,
        }
      }

      if s == u32::MAX {
        atomic_wait::wait(&self.state, u32::MAX);

        s = self.state.load(Relaxed);
      }
    }
  }

  pub fn write(&'_ self) -> MyWriteGuard<'_, T> {
    while let Err(s) =
      self.state.compare_exchange(0, u32::MAX, Acquire, Relaxed)
    {
      atomic_wait::wait(&self.state, s);
    }

    MyWriteGuard {
      my_rw_lock: self,
    }
  }
}

unsafe impl<T> Sync for MyRwLock<T> where T: Sync {}

#[cfg(test)]
mod test {

  use super::*;

  #[test]
  fn test1() {
    crate::init_tracing();

    // Test code adapted from main() function on Chapter 4 page 82

    let x: MyRwLock<Vec<i32>> = MyRwLock::new(Vec::new());

    thread::scope(|s| {
      s.spawn(|| x.write().push(1));

      s.spawn(|| {
        let mut g: MyWriteGuard<'_, Vec<i32>> = x.write();

        g.push(2);

        g.push(2);
      });
    });

    let g: MyReadGuard<'_, Vec<i32>> = x.read();

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
