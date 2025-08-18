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

struct MyArcData<T> {
  data: T,
  ref_count: AtomicUsize,
}

pub struct MyArc<T> {
  ptr: NonNull<MyArcData<T>>,
}

unsafe impl<T: Send + Sync> Send for MyArc<T> {}

unsafe impl<T: Send + Sync> Sync for MyArc<T> {}

impl<T> MyArc<T> {
  pub fn new(data: T) -> MyArc<T> {
    MyArc {
      ptr: NonNull::from(Box::leak(Box::new(MyArcData {
        data,
        ref_count: AtomicUsize::new(1),
      }))),
    }
  }

  fn data(&self) -> &MyArcData<T> {
    unsafe { self.ptr.as_ref() }
  }
}

impl<T> Clone for MyArc<T> {
  fn clone(&self) -> Self {
    if self.data().ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
      process::abort();
    }

    Self {
      ptr: self.ptr,
    }
  }
}

impl<T> Deref for MyArc<T> {
  type Target = T;

  fn deref(&self) -> &T {
    &self.data().data
  }
}

impl<T> Drop for MyArc<T> {
  fn drop(&mut self) {
    if self.data().ref_count.fetch_sub(1, Release) == 1 {
      atomic::fence(Acquire);

      unsafe {
        drop(Box::from_raw(self.ptr.as_ptr()));
      }
    }
  }
}
#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    crate::init_tracing();

    static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);

    struct DetectDrop;

    impl Drop for DetectDrop {
      fn drop(&mut self) {
        info!("DetectDrop.drop()");

        NUM_DROPS.fetch_add(1, Relaxed);
      }
    }

    let x: Arc<(&'static str, DetectDrop)> = Arc::new(("hello", DetectDrop));

    let y: Arc<(&'static str, DetectDrop)> = x.clone();

    let t: JoinHandle<()> = thread::spawn(move || {
      assert_eq!(x.0, "hello");
    });

    assert_eq!(y.0, "hello");

    t.join().unwrap();

    assert_eq!(NUM_DROPS.load(Relaxed), 0);

    drop(y);

    assert_eq!(NUM_DROPS.load(Relaxed), 1);
  }
}
