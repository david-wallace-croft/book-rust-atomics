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
    let id = NEXT_ID.fetch_add(1, Relaxed);

    info!("inner id: {id}");

    if id >= 10 {
      NEXT_ID.fetch_sub(1, Relaxed);

      return Err("too many IDs!".into());
    }

    Ok(id)
  }

  #[test]
  fn test1() {
    crate::init_tracing();

    thread::scope(|s| {
      for _ in 0..20 {
        s.spawn(|| match allocate_new_id() {
          Ok(id) => info!("outer id: {id}"),
          Err(message) => info!("error: {message}"),
        });
      }
    });

    info!("next id: {NEXT_ID:?}");
  }
}
