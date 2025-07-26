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
use rand::prelude::*;
#[allow(unused)]
use tracing::info;

#[cfg(test)]
mod test {
  use super::*;

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

    let num_done: &AtomicUsize = &Default::default();

    let total_time: &AtomicU64 = &Default::default();

    let max_time: &AtomicU64 = &Default::default();

    thread::scope(|s| {
      for t in 0..4 {
        s.spawn(move || {
          for i in 0..25 {
            let start: Instant = Instant::now();

            process_item(t * 25 + i, 300);

            let time_taken: u64 = start.elapsed().as_micros() as u64;

            num_done.fetch_add(1, Relaxed);

            total_time.fetch_add(time_taken, Relaxed);

            max_time.fetch_max(time_taken, Relaxed);
          }
        });
      }

      loop {
        let total_time: Duration =
          Duration::from_micros(total_time.load(Relaxed));

        let max_time: Duration = Duration::from_micros(max_time.load(Relaxed));

        let n: usize = num_done.load(Relaxed);

        info!("{n} of 100 done");

        if n == 0 {
          info!("Working... nothing done yet");
        } else {
          info!(
            "Working... {n} of 100 done, {:?} average, {:?} peak",
            total_time / n as u32,
            max_time
          );
        }

        if n == 100 {
          break;
        }

        thread::sleep(Duration::from_secs(1));
      }
    });

    info!("Done!");
  }
}
