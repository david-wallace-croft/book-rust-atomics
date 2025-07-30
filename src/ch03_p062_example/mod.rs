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
      Ordering::Acquire, Ordering::Relaxed, Ordering::Release,
    },
  },
  thread::{self, JoinHandle, Scope, ScopedJoinHandle, Thread, ThreadId},
  time::{Duration, Instant},
};
#[allow(unused)]
use rand::prelude::*;
#[allow(unused)]
use tracing::info;

#[allow(unused)]
#[derive(Debug)]
struct Data {
  i: u32,
}

#[allow(unused)]
fn generate_data(i: u32) -> Data {
  let mut rng: ThreadRng = rand::rng();

  let millis: u64 = rng.random_range(0..1);

  thread::sleep(Duration::from_millis(millis));

  Data {
    i,
  }
}

#[cfg(test)]
mod test {
  use super::*;

  fn get_data(i: u32) -> &'static Data {
    static PTR: AtomicPtr<Data> = AtomicPtr::new(ptr::null_mut());

    let mut p: *mut Data = PTR.load(Acquire);

    if p.is_null() {
      p = Box::into_raw(Box::new(generate_data(i)));

      if let Err(e) = PTR.compare_exchange(ptr::null_mut(), p, Release, Acquire)
      {
        drop(unsafe { Box::from_raw(p) });

        p = e;
      }
    }

    unsafe { &*p }
  }

  #[test]
  fn test1() {
    crate::init_tracing();

    thread::scope(|s: &Scope<'_, '_>| {
      for i in 0..10 {
        s.spawn(move || {
          let a: &'static Data = get_data(i);

          info!("{a:?}");
        });
      }
    });
  }
}
