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

  static NEXT_ID: AtomicU32 = AtomicU32::new(0);

  fn allocate_new_id() -> Result<u32, String> {
    let mut next_id: u32 = NEXT_ID.load(Relaxed);

    loop {
      info!("inner next id: {next_id}");

      if next_id >= 10 {
        return Err("too many IDs!".into());
      }

      match NEXT_ID.compare_exchange_weak(
        next_id,
        next_id + 1,
        Relaxed,
        Relaxed,
      ) {
        Ok(_) => return Ok(next_id),
        Err(v) => next_id = v,
      }
    }
  }

  fn allocate_new_id_2() -> Result<u32, String> {
    match NEXT_ID.fetch_update(Relaxed, Relaxed, |n| {
      if n >= 10 {
        return None;
      }

      n.checked_add(1)
    }) {
      Ok(next_id) => Ok(next_id),
      Err(_) => Err("too many IDs!".into()),
    }
  }

  #[test]
  fn test1() {
    crate::init_tracing();

    thread::scope(|s| {
      for _ in 0..20 {
        s.spawn(|| match allocate_new_id() {
          Ok(next_id) => info!("outer next_id...: {next_id}"),
          Err(message) => info!("error: {message}"),
        });
      }
    });

    info!("next id: {NEXT_ID:?}");
  }

  #[test]
  fn test2() {
    crate::init_tracing();

    thread::scope(|s| {
      for _ in 0..20 {
        s.spawn(|| match allocate_new_id_2() {
          Ok(next_id) => info!("outer next_id...: {next_id}"),
          Err(message) => info!("error: {message}"),
        });
      }
    });

    info!("next id: {NEXT_ID:?}");
  }
}
