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
    Condvar, LazyLock, Mutex, MutexGuard, Once,
    atomic::{Ordering::*, *},
  },
  thread::{self, JoinHandle, Scope, ScopedJoinHandle, Thread, ThreadId},
  time::{Duration, Instant},
};
#[allow(unused)]
use tracing::info;

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    crate::init_tracing();

    let queue: Mutex<VecDeque<i32>> = Default::default();

    thread::scope(|s: &Scope<'_, '_>| {
      let t: ScopedJoinHandle<'_, ()> = s.spawn(|| {
        loop {
          let item: Option<i32> = queue.lock().unwrap().pop_front();

          info!("Consumer thread: Popped item: {item:?}");

          match item {
            Some(-1) => {
              info!("Consumer thread: Breaking out of loop");

              break;
            },
            Some(item) => {
              info!("Consumer thread: Consuming item {item}");
            },
            None => {
              info!("Consumer thread: Parking");

              thread::park();

              info!("Consumer thread: Unparked");
            },
          }
        }

        info!("Consumer thread: Ending");
      });

      for i in 0..3 {
        queue.lock().unwrap().push_back(i);

        info!("Producer thread: Pushed item: {i}");

        info!("Producer thread: Unparking consumer thread");

        t.thread().unpark();

        info!("Producer thread: Sleep starting");

        thread::sleep(Duration::from_secs(1));

        info!("Producer thread: Sleep finished");
      }

      queue.lock().unwrap().push_back(-1);

      info!("Producer thread: Pushed item: -1");

      info!("Producer thread: Unparking consumer thread");

      t.thread().unpark();

      info!("Producer thread: Ending");
    });
  }

  #[test]
  fn test2() {
    crate::init_tracing();

    let queue: Mutex<VecDeque<i32>> = Default::default();

    let not_empty: Condvar = Condvar::new();

    thread::scope(|s: &Scope<'_, '_>| {
      let _t: ScopedJoinHandle<'_, ()> = s.spawn(|| {
        loop {
          info!("Consumer thread: Locking queue");

          let mut guard: MutexGuard<'_, VecDeque<i32>> = queue.lock().unwrap();

          info!("Consumer thread: Locked queue");

          let item = loop {
            if let Some(item) = guard.pop_front() {
              info!("Consumer thread: Popped item {item}");

              break item;
            } else {
              info!("Consumer thread: Popped None");

              info!(
                "Consumer thread: Unlocking queue and waiting for notification"
              );

              guard = not_empty.wait(guard).unwrap();

              info!("Consumer thread: Locked queue after notification");
            }
          };

          info!("Consumer thread: Unlocking queue by dropping guard");

          drop(guard);

          info!("Consumer thread: Consuming item {item}");

          if item == -1 {
            info!("Consumer thread: Breaking out of loop");

            break;
          }
        }

        info!("Consumer thread: Ending");
      });

      for i in 0..3 {
        info!("Producer thread: Locking queue");

        let mut guard: MutexGuard<'_, VecDeque<i32>> = queue.lock().unwrap();

        guard.push_back(i);

        info!("Producer thread: Pushed item: {i}");

        info!("Producer thread: Unlocking queue by dropping guard");

        drop(guard);

        info!("Producer thread: Notifying one");

        not_empty.notify_one();

        info!("Producer thread: Sleep starting");

        thread::sleep(Duration::from_secs(1));

        info!("Producer thread: Sleep finished");
      }

      queue.lock().unwrap().push_back(-1);

      info!("Producer thread: Pushed item: -1");

      info!("Producer thread: Notifying one");

      not_empty.notify_one();

      info!("Producer thread: Ending");
    });
  }
}
