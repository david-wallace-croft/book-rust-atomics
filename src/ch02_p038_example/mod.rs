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
    atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed},
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

  fn process_item(
    _i: usize,
    max_delay: u64,
  ) {
    // info!("Worker thread: Processing item {i} starting");

    let mut rng: ThreadRng = rand::rng();

    let millis: u64 = rng.random_range(1..=max_delay);

    thread::sleep(Duration::from_millis(millis));

    // info!("Worker thread: Processing item {i} finished");
  }

  #[test]
  fn test1() {
    crate::init_tracing();

    let num_done = &AtomicUsize::new(0);

    thread::scope(|s| {
      for t in 0..4 {
        s.spawn(move || {
          for i in 0..25 {
            process_item(t * 25 + i, 300);

            num_done.fetch_add(1, Relaxed);
          }
        });
      }

      loop {
        let n: usize = num_done.load(Relaxed);

        info!("{n} of 100 done");

        if n == 100 {
          break;
        }

        thread::sleep(Duration::from_secs(1));
      }
    });

    info!("Done!");
  }
}
