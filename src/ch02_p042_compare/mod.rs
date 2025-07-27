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
      AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering::Relaxed,
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

  fn increment(a: &AtomicU32) {
    let mut current: u32 = a.load(Relaxed);

    loop {
      let new: u32 = current + 1;

      let mut rng: ThreadRng = rand::rng();

      let ms: u64 = rng.random_range(0..1);

      thread::sleep(Duration::from_millis(ms));

      match a.compare_exchange(current, new, Relaxed, Relaxed) {
        Ok(_) => return,
        Err(v) => {
          info!("Try again");

          current = v;
        },
      }
    }
  }

  #[test]
  fn test1() {
    crate::init_tracing();

    static COUNTER: AtomicU32 = AtomicU32::new(0);

    thread::scope(|s| {
      for _ in 0..10 {
        s.spawn(|| increment(&COUNTER));
      }
    });

    info!("count: {COUNTER:?}");
  }
}
