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

pub struct MyCondvar {
  counter: AtomicU32,
  num_waiters: AtomicUsize,
}

impl MyCondvar {
  pub const fn new() -> Self {
    Self {
      counter: AtomicU32::new(0),
      num_waiters: AtomicUsize::new(0),
    }
  }

  pub fn notify_one(&self) {
    if self.num_waiters.load(Relaxed) > 0 {
      self.counter.fetch_add(1, Relaxed);

      atomic_wait::wake_one(&self.counter);
    }
  }

  pub fn notify_all(&self) {
    if self.num_waiters.load(Relaxed) > 0 {
      self.counter.fetch_add(1, Relaxed);

      atomic_wait::wake_all(&self.counter);
    }
  }

  pub fn wait<'a, T>(
    &self,
    guard: MyMutexGuard<'a, T>,
  ) -> MyMutexGuard<'a, T> {
    self.num_waiters.fetch_add(1, Relaxed);

    let counter_value: u32 = self.counter.load(Relaxed);

    let mutex: &'a MyMutex<T> = guard.my_mutex;

    drop(guard);

    atomic_wait::wait(&self.counter, counter_value);

    self.num_waiters.fetch_sub(1, Relaxed);

    mutex.lock()
  }
}

#[cfg(test)]
mod test {

  use super::*;

  #[test]
  fn test1() {
    crate::init_tracing();

    let my_mutex: MyMutex<i32> = MyMutex::new(0);

    let my_condvar: MyCondvar = MyCondvar::new();

    let mut wakeups: i32 = 0;

    thread::scope(|s| {
      s.spawn(|| {
        thread::sleep(Duration::from_secs(1));

        *my_mutex.lock() = 123;

        my_condvar.notify_one();
      });

      let mut m: MyMutexGuard<'_, i32> = my_mutex.lock();

      while *m < 100 {
        m = my_condvar.wait(m);

        wakeups += 1;
      }

      assert_eq!(*m, 123);
    });

    assert!(wakeups < 10);
  }
}
