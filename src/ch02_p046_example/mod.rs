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

  static KEY: AtomicU64 = AtomicU64::new(0);

  fn generate_random_key() -> u64 {
    let mut rng: ThreadRng = rand::rng();

    rng.random_range(1..=10)
  }

  fn get_key() -> u64 {
    let key: u64 = KEY.load(Relaxed);

    if key == 0 {
      let new_key: u64 = generate_random_key();

      info!("rnd: {new_key}");

      let mut rng: ThreadRng = rand::rng();

      let ms: u64 = rng.random_range(0..1);

      thread::sleep(Duration::from_millis(ms));

      match KEY.compare_exchange(0, new_key, Relaxed, Relaxed) {
        Ok(_) => new_key,
        Err(k) => k,
      }
    } else {
      key
    }
  }

  #[test]
  fn test1() {
    crate::init_tracing();

    thread::scope(|s| {
      for _ in 0..20 {
        s.spawn(|| info!("key: {}", get_key()));
      }
    });
  }
}
