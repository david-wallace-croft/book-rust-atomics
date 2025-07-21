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
    atomic::{AtomicBool, Ordering::Relaxed},
  },
  thread::{self, JoinHandle, Scope, ScopedJoinHandle, Thread, ThreadId},
  time::{Duration, Instant},
};
#[allow(unused)]
use tracing::info;

#[cfg(test)]
mod test {
  use super::*;

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
}
