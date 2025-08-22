#![allow(unused)]

use super::my_arc_data::MyArcData;
use super::my_weak::MyWeak;
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

pub struct MyArc<T> {
  pub ptr: NonNull<MyArcData<T>>,
}

unsafe impl<T: Send + Sync> Send for MyArc<T> {}

unsafe impl<T: Send + Sync> Sync for MyArc<T> {}

impl<T> MyArc<T> {
  pub fn new(data: T) -> MyArc<T> {
    MyArc {
      ptr: NonNull::from(Box::leak(Box::new(MyArcData {
        alloc_ref_count: AtomicUsize::new(1),
        data: UnsafeCell::new(ManuallyDrop::new(data)),
        data_ref_count: AtomicUsize::new(1),
      }))),
    }
  }

  pub fn downgrade(my_arc: &Self) -> MyWeak<T> {
    let mut n = my_arc.data().alloc_ref_count.load(Relaxed);

    loop {
      if n == usize::MAX {
        hint::spin_loop();

        n = my_arc.data().alloc_ref_count.load(Relaxed);

        continue;
      }

      assert!(n < usize::MAX - 1);

      if let Err(e) = my_arc.data().alloc_ref_count.compare_exchange_weak(
        n,
        n + 1,
        Acquire,
        Relaxed,
      ) {
        n = e;

        continue;
      }

      return MyWeak {
        ptr: my_arc.ptr,
      };
    }
  }

  pub fn get_mut(my_arc: &mut Self) -> Option<&mut T> {
    if my_arc
      .data()
      .alloc_ref_count
      .compare_exchange(1, usize::MAX, Acquire, Relaxed)
      .is_err()
    {
      return None;
    }

    let is_unique = my_arc.data().data_ref_count.load(Relaxed) == 1;

    my_arc.data().alloc_ref_count.store(1, Release);

    if !is_unique {
      return None;
    }

    atomic::fence(Acquire);

    unsafe { Some(&mut *my_arc.data().data.get()) }
  }

  fn data(&self) -> &MyArcData<T> {
    unsafe { self.ptr.as_ref() }
  }
}

impl<T> Clone for MyArc<T> {
  fn clone(&self) -> Self {
    if self.data().data_ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
      process::abort();
    }

    MyArc {
      ptr: self.ptr,
    }
  }
}

impl<T> Deref for MyArc<T> {
  type Target = T;

  fn deref(&self) -> &T {
    unsafe { &*self.data().data.get() }
  }
}

impl<T> Drop for MyArc<T> {
  fn drop(&mut self) {
    if self.data().data_ref_count.fetch_sub(1, Release) == 1 {
      atomic::fence(Acquire);

      unsafe {
        ManuallyDrop::drop(&mut *self.data().data.get());
      }

      drop(MyWeak {
        ptr: self.ptr,
      });
    }
  }
}
