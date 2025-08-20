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
  pub weak: MyWeak<T>,
}

unsafe impl<T: Send + Sync> Send for MyArc<T> {}

unsafe impl<T: Send + Sync> Sync for MyArc<T> {}

impl<T> MyArc<T> {
  pub fn new(data: T) -> MyArc<T> {
    MyArc {
      weak: MyWeak {
        ptr: NonNull::from(Box::leak(Box::new(MyArcData {
          alloc_ref_count: AtomicUsize::new(1),
          data: UnsafeCell::new(Some(data)),
          data_ref_count: AtomicUsize::new(1),
        }))),
      },
    }
  }

  pub fn downgrade(my_arc: &Self) -> MyWeak<T> {
    my_arc.weak.clone()
  }

  pub fn get_mut(my_arc: &mut Self) -> Option<&mut T> {
    if my_arc.weak.data().alloc_ref_count.load(Relaxed) == 1 {
      atomic::fence(Acquire);

      let arcdata: &mut MyArcData<T> = unsafe { my_arc.weak.ptr.as_mut() };

      let option: &mut Option<T> = arcdata.data.get_mut();

      let data: &mut T = option.as_mut().unwrap();

      Some(data)
    } else {
      None
    }
  }

  // fn data(&self) -> &MyArcData<T> {
  //   unsafe { self.ptr.as_ref() }
  // }
}

impl<T> Clone for MyArc<T> {
  fn clone(&self) -> Self {
    let weak: MyWeak<T> = self.weak.clone();

    if weak.data().data_ref_count.fetch_add(1, Relaxed) > usize::MAX / 2 {
      process::abort();
    }

    MyArc {
      weak,
    }
  }
}

impl<T> Deref for MyArc<T> {
  type Target = T;

  fn deref(&self) -> &T {
    let ptr: *mut Option<T> = self.weak.data().data.get();

    unsafe { (*ptr).as_ref().unwrap() }
  }
}

impl<T> Drop for MyArc<T> {
  fn drop(&mut self) {
    if self.weak.data().data_ref_count.fetch_sub(1, Release) == 1 {
      atomic::fence(Acquire);

      let ptr: *mut Option<T> = self.weak.data().data.get();

      unsafe {
        (*ptr) = None;
      }
    }
  }
}
