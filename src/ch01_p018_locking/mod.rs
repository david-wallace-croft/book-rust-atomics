#[allow(unused)]
use ::std::{
  any::Any,
  cell::{Cell, RefCell, UnsafeCell},
  collections::VecDeque,
  marker::PhantomData,
  mem::{ManuallyDrop, MaybeUninit},
  ops::{Deref, DerefMut},
  ptr::NonNull,
  rc::Rc,
  sync::{
    atomic::{Ordering::*, *},
    *,
  },
  thread::{self, JoinHandle, Scope, Thread, ThreadId},
  time::{Duration, Instant},
};

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let n: Mutex<i32> = Default::default();

    thread::scope(|s: &Scope<'_, '_>| {
      for _ in 0..10 {
        s.spawn(|| {
          let mut guard: MutexGuard<'_, i32> = n.lock().unwrap();

          for _ in 0..100 {
            *guard += 1;
          }
        });
      }
    });

    assert_eq!(n.into_inner().unwrap(), 1_000);
  }

  #[test]
  fn test2() {
    let n: Mutex<i32> = Default::default();

    let start: Instant = Instant::now();

    thread::scope(|s: &Scope<'_, '_>| {
      for _ in 0..10 {
        s.spawn(|| {
          let mut guard: MutexGuard<'_, i32> = n.lock().unwrap();

          for _ in 0..100 {
            *guard += 1;
          }

          thread::sleep(Duration::from_secs(1));

          println!("Spawned thread is done.");
        });
      }
    });

    assert_eq!(n.into_inner().unwrap(), 1_000);

    let duration: Duration = start.elapsed();

    println!("Duration: {duration:?}");

    assert!(duration > Duration::from_secs(10));
  }

  #[test]
  fn test3() {
    let n: Mutex<i32> = Default::default();

    let start: Instant = Instant::now();

    thread::scope(|s: &Scope<'_, '_>| {
      for _ in 0..10 {
        s.spawn(|| {
          let mut guard: MutexGuard<'_, i32> = n.lock().unwrap();

          for _ in 0..100 {
            *guard += 1;
          }

          drop(guard);

          thread::sleep(Duration::from_secs(1));

          println!("Spawned thread is done.");
        });
      }
    });

    assert_eq!(n.into_inner().unwrap(), 1_000);

    let duration: Duration = start.elapsed();

    println!("Duration: {duration:?}");

    assert!(duration < Duration::from_secs(2));
  }
}
