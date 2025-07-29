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

  static DATA: AtomicU64 = AtomicU64::new(0);

  static READY: AtomicBool = AtomicBool::new(false);

  #[test]
  fn test1() {
    crate::init_tracing();

    thread::spawn(|| {
      DATA.store(123, Relaxed);

      READY.store(true, Release);
    });

    while !READY.load(Acquire) {
      thread::sleep(Duration::from_millis(100));

      info!("waiting");
    }

    info!("{}", DATA.load(Relaxed));
  }
}
