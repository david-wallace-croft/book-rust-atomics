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

  fn process_item(_i: usize) {
    // info!("Worker thread: Processing item {i} starting");

    let mut rng: ThreadRng = rand::rng();

    let millis: u64 = rng.random_range(10..=100);

    thread::sleep(Duration::from_millis(millis));

    // info!("Worker thread: Processing item {i} finished");
  }

  fn some_work() {
    info!("Background thread: Work starting");

    thread::sleep(Duration::from_secs(3));

    info!("Background thread: Work finished");
  }

  #[test]
  fn test1() {
    crate::init_tracing();

    static STOP: AtomicBool = AtomicBool::new(false);

    let background_thread: JoinHandle<()> = thread::spawn(|| {
      while !STOP.load(Relaxed) {
        some_work();
      }

      info!("Background thread: Stopping")
    });

    for line in io::stdin().lines() {
      match line.unwrap().as_str() {
        "help" => info!("commands: help, stop"),
        "stop" => break,
        cmd => info!("unknown command: {cmd:?}"),
      }
    }

    info!("Main thread: Raising stop flag");

    STOP.store(true, Relaxed);

    info!("Main thread: Joining background thread");

    background_thread.join().unwrap();
  }

  #[test]
  fn test2() {
    crate::init_tracing();

    let num_done: AtomicUsize = Default::default();

    thread::scope(|s: &Scope<'_, '_>| {
      s.spawn(|| {
        for i in 0..100 {
          process_item(i);

          num_done.store(i + 1, Relaxed);
        }
      });

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
