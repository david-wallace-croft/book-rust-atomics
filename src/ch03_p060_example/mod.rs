#[allow(unused)]
use ::std::{
  any::Any,
  cell::{Cell, RefCell, UnsafeCell},
  collections::VecDeque,
  io,
  marker::PhantomData,
  mem::{ManuallyDrop, MaybeUninit},
  ops::{Deref, DerefMut},
  ptr::NonNull,
  rc::Rc,
  sync::{
    Condvar, LazyLock, Mutex, MutexGuard, Once,
    atomic::{
      AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering::Acquire,
      Ordering::Relaxed, Ordering::Release,
    },
  },
  thread::{self, JoinHandle, Scope, ScopedJoinHandle, Thread, ThreadId},
  time::{Duration, Instant},
};
#[allow(unused)]
use rand::prelude::*;
#[allow(unused)]
use tracing::info;

#[cfg(test)]
mod test {
  use super::*;

  static mut DATA: String = String::new();

  static LOCKED: AtomicBool = AtomicBool::new(false);

  fn f() {
    if LOCKED
      .compare_exchange(false, true, Acquire, Relaxed)
      .is_ok()
    {
      #[allow(static_mut_refs)]
      unsafe {
        DATA.push('!');
      }

      LOCKED.store(false, Release);
    }
  }

  #[test]
  fn test1() {
    crate::init_tracing();

    thread::scope(|s| {
      for _ in 0..100 {
        s.spawn(f);
      }
    });

    #[allow(static_mut_refs)]
    unsafe {
      info!("{DATA}");
    }
  }
}
