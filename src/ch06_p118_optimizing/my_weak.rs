#![allow(unused)]

use super::my_arc::MyArc;
use super::my_arc_data::MyArcData;
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

pub struct MyWeak<T> {
  pub ptr: NonNull<MyArcData<T>>,
}

impl<T> MyWeak<T> {
  pub fn upgrade(&self) -> Option<MyArc<T>> {
    let mut n: usize = self.data().data_ref_count.load(Relaxed);

    loop {
      if n == 0 {
        return None;
      }

      assert!(n < usize::MAX);

      if let Err(e) = self.data().data_ref_count.compare_exchange_weak(
        n,
        n + 1,
        Relaxed,
        Relaxed,
      ) {
        n = e;

        continue;
      }

      return Some(MyArc {
        ptr: self.ptr,
      });
    }
  }

  fn data(&self) -> &MyArcData<T> {
    unsafe { self.ptr.as_ref() }
  }
}

impl<T> Clone for MyWeak<T> {
  fn clone(&self) -> Self {
    if self.data().alloc_ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
      process::abort();
    }

    MyWeak {
      ptr: self.ptr,
    }
  }
}

impl<T> Drop for MyWeak<T> {
  fn drop(&mut self) {
    if self.data().alloc_ref_count.fetch_sub(1, Release) == 1 {
      atomic::fence(Acquire);

      unsafe {
        drop(Box::from_raw(self.ptr.as_ptr()));
      }
    }
  }
}

unsafe impl<T: Send + Sync> Send for MyWeak<T> {}

unsafe impl<T: Send + Sync> Sync for MyWeak<T> {}
