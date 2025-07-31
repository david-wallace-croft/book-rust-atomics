#[allow(unused)]
use ::std::{
  any::Any,
  cell::{Cell, RefCell, UnsafeCell},
  collections::VecDeque,
  io,
  marker::PhantomData,
  mem::{ManuallyDrop, MaybeUninit},
  ops::{Deref, DerefMut},
  ptr,
  rc::Rc,
  sync::{
    Condvar, LazyLock, Mutex, MutexGuard, Once,
    atomic::{
      AtomicBool, AtomicPtr, AtomicU32, AtomicU64, AtomicUsize,
      Ordering::{Acquire, Relaxed, Release, SeqCst},
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

  #[test]
  fn test1() {
    crate::init_tracing();

    static A: AtomicBool = AtomicBool::new(false);

    static B: AtomicBool = AtomicBool::new(false);

    static mut S: String = String::new();

    let a: JoinHandle<()> = thread::spawn(|| {
      A.store(true, SeqCst);

      if !B.load(SeqCst) {
        unsafe {
          #[allow(static_mut_refs)]
          S.push('!');
        }
      }
    });

    let b: JoinHandle<()> = thread::spawn(|| {
      B.store(true, SeqCst);

      if !A.load(SeqCst) {
        unsafe {
          #[allow(static_mut_refs)]
          S.push('!');
        }
      }
    });

    a.join().unwrap();

    b.join().unwrap();

    unsafe {
      #[allow(static_mut_refs)]
      let s: String = S.clone();

      info!("S is {s:?}");

      assert_eq!("!", s.as_str());
    }
  }
}
