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
    atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering::Relaxed},
  },
  thread::{self, JoinHandle, Scope, ScopedJoinHandle, Thread, ThreadId},
  time::{Duration, Instant},
};
#[allow(unused)]
use tracing::info;

#[cfg(test)]
mod test {
  use super::*;
  use rand::prelude::*;

  fn calculate_x(i: u64) -> u64 {
    let mut rng: ThreadRng = rand::rng();

    let millis: u64 = rng.random_range(0..2);

    thread::sleep(Duration::from_millis(millis));

    i
  }

  fn get_x(i: u64) -> u64 {
    static X: AtomicU64 = AtomicU64::new(0);

    let mut x: u64 = X.load(Relaxed);

    if x == 0 {
      x = calculate_x(i);

      X.store(x, Relaxed);
    }

    x
  }

  #[test]
  fn test1() {
    crate::init_tracing();

    thread::scope(|s: &Scope<'_, '_>| {
      for i in 1..=10 {
        info!("Spawning thread {i}");

        s.spawn(move || {
          let x: u64 = get_x(i);

          info!("x is {x}");
        });
      }
    });
  }
}
