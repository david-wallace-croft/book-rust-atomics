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
}

impl MyCondvar {
  pub const fn new() -> Self {
    Self {
      counter: AtomicU32::new(0),
    }
  }

  pub fn notify_one(&self) {
    self.counter.fetch_add(1, Relaxed);

    atomic_wait::wake_one(&self.counter);
  }

  pub fn notify_all(&self) {
    self.counter.fetch_add(1, Relaxed);

    atomic_wait::wake_all(&self.counter);
  }

  pub fn wait<'a, T>(
    &self,
    guard: MyMutexGuard<'a, T>,
  ) -> MyMutexGuard<'a, T> {
    let counter_value = self.counter.load(Relaxed);

    let mutex = guard.my_mutex;

    drop(guard);

    atomic_wait::wait(&self.counter, counter_value);

    mutex.lock()
  }
}

#[cfg(test)]
mod test {

  use super::*;

  #[test]
  fn test1() {
    crate::init_tracing();

    let my_mutex = MyMutex::new(0);

    let my_condvar = MyCondvar::new();

    let mut wakeups = 0;

    thread::scope(|s| {
      s.spawn(|| {
        thread::sleep(Duration::from_secs(1));

        *my_mutex.lock() = 123;

        my_condvar.notify_one();
      });

      let mut m = my_mutex.lock();

      while *m < 100 {
        m = my_condvar.wait(m);

        wakeups += 1;
      }

      assert_eq!(*m, 123);
    });

    assert!(wakeups < 10);
  }
}
