#![allow(unused)]

use self::my_arc::MyArc;
use self::my_weak::MyWeak;
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

mod my_arc;
mod my_arc_data;
mod my_weak;

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

    let x: MyArc<(&'static str, DetectDrop)> =
      MyArc::new(("hello", DetectDrop));

    let y: MyWeak<(&'static str, DetectDrop)> = MyArc::downgrade(&x);

    let z: MyWeak<(&'static str, DetectDrop)> = MyArc::downgrade(&x);

    let t: JoinHandle<()> = thread::spawn(move || {
      let y: MyArc<(&'static str, DetectDrop)> = y.upgrade().unwrap();

      assert_eq!(y.0, "hello");
    });

    assert_eq!(x.0, "hello");

    t.join().unwrap();

    assert_eq!(NUM_DROPS.load(Relaxed), 0);

    assert!(z.upgrade().is_some());

    drop(x);

    assert_eq!(NUM_DROPS.load(Relaxed), 1);

    assert!(z.upgrade().is_none());
  }
}
