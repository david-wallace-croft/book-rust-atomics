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
  thread::{self, JoinHandle, Scope, ScopedJoinHandle, Thread, ThreadId},
  time::{Duration, Instant},
};

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let queue: Mutex<VecDeque<i32>> = Default::default();

    thread::scope(|s: &Scope<'_, '_>| {
      let t: ScopedJoinHandle<'_, ()> = s.spawn(|| {
        loop {
          let item: Option<i32> = queue.lock().unwrap().pop_front();

          println!("Consumer thread: Popped item: {item:?}");

          match item {
            Some(-1) => {
              println!("Consumer thread: Breaking out of loop");

              break;
            },
            Some(item) => {
              println!("Consumer thread: Consuming item {item}");
            },
            None => {
              println!("Consumer thread: Parking");

              thread::park();

              println!("Consumer thread: Unparked");
            },
          }
        }

        println!("Consumer thread: Ending");
      });

      for i in 0..3 {
        queue.lock().unwrap().push_back(i);

        println!("Producer thread: Pushed item: {i}");

        println!("Producer thread: Unparking consumer thread");

        t.thread().unpark();

        println!("Producer thread: Sleep starting");

        thread::sleep(Duration::from_secs(1));

        println!("Producer thread: Sleep finished");
      }

      queue.lock().unwrap().push_back(-1);

      println!("Producer thread: Pushed item: -1");

      println!("Producer thread: Unparking consumer thread");

      t.thread().unpark();

      println!("Producer thread: Ending");
    });
  }

  #[test]
  fn test2() {
    let queue: Mutex<VecDeque<i32>> = Default::default();

    let not_empty: Condvar = Condvar::new();

    thread::scope(|s: &Scope<'_, '_>| {
      let _t: ScopedJoinHandle<'_, ()> = s.spawn(|| {
        loop {
          println!("Consumer thread: Locking queue");

          let mut guard: MutexGuard<'_, VecDeque<i32>> = queue.lock().unwrap();

          println!("Consumer thread: Locked queue");

          let item = loop {
            if let Some(item) = guard.pop_front() {
              println!("Consumer thread: Popped item {item}");

              break item;
            } else {
              println!("Consumer thread: Popped None");

              println!(
                "Consumer thread: Unlocking queue and waiting for notification"
              );

              guard = not_empty.wait(guard).unwrap();

              println!("Consumer thread: Locked queue after notification");
            }
          };

          println!("Consumer thread: Unlocking queue by dropping guard");

          drop(guard);

          println!("Consumer thread: Consuming item {item}");

          if item == -1 {
            println!("Consumer thread: Breaking out of loop");

            break;
          }
        }

        println!("Consumer thread: Ending");
      });

      for i in 0..3 {
        println!("Producer thread: Locking queue");

        let mut guard: MutexGuard<'_, VecDeque<i32>> = queue.lock().unwrap();

        guard.push_back(i);

        println!("Producer thread: Pushed item: {i}");

        println!("Producer thread: Unlocking queue by dropping guard");

        drop(guard);

        println!("Producer thread: Notifying one");

        not_empty.notify_one();

        println!("Producer thread: Sleep starting");

        thread::sleep(Duration::from_secs(1));

        println!("Producer thread: Sleep finished");
      }

      queue.lock().unwrap().push_back(-1);

      println!("Producer thread: Pushed item: -1");

      println!("Producer thread: Notifying one");

      not_empty.notify_one();

      println!("Producer thread: Ending");
    });
  }
}
