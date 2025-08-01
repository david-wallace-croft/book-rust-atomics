#[allow(unused)]
use ::std::{
  any::Any,
  array,
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
      self, AtomicBool, AtomicPtr, AtomicU32, AtomicU64, AtomicUsize,
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

#[allow(dead_code)]
fn some_calculation(i: usize) -> usize {
  let mut rng: ThreadRng = rand::rng();

  let millis: u64 = rng.random_range(0..1_000);

  thread::sleep(Duration::from_millis(millis));

  i
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    crate::init_tracing();

    static mut DATA: [usize; 10] = [0; 10];

    const ATOMIC_FALSE: AtomicBool = AtomicBool::new(false);

    static READY: [AtomicBool; 10] = [ATOMIC_FALSE; 10];

    for i in 0..10 {
      thread::spawn(move || {
        let data: usize = some_calculation(i);

        unsafe {
          DATA[i] = data;
        }

        // Every access before the Release stays before it.
        // This means the other thread will see the write to DATA before the
        // write to READY.
        READY[i].store(true, Release);
      });
    }

    thread::sleep(Duration::from_millis(500));

    let ready: [bool; 10] = array::from_fn(|i| READY[i].load(Relaxed));

    if ready.contains(&true) {
      // Every access after the Acquire stays after it.
      // TODO: This means the reads of DATA will be after the reads of READY?
      atomic::fence(Acquire);

      for i in 0..10 {
        if ready[i] {
          info!("data{i} = {}", unsafe { DATA[i] });
        }
      }
    }
  }
}
