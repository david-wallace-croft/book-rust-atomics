#[allow(unused)]
use ::std::{
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
  thread::{self, JoinHandle, Thread, ThreadId},
};

#[allow(dead_code)]
fn f() {
  println!("Hello from another thread!");

  let id: thread::ThreadId = thread::current().id();

  println!("This is my thread id: {id:?}");
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let t1: JoinHandle<()> = thread::spawn(f);

    let t2: JoinHandle<()> = thread::spawn(f);

    println!("Hello from the main thread.");

    t1.join().unwrap();

    t2.join().unwrap();
  }

  #[test]
  fn test2() {
    let numbers: Vec<i32> = vec![
      1, 2, 3,
    ];

    thread::spawn(move || {
      for n in &numbers {
        println!("{n}");
      }
    })
    .join()
    .unwrap();
  }

  #[test]
  fn test3() {
    let numbers: Vec<usize> = Vec::from_iter(0..=1_000);

    let t: JoinHandle<usize> = thread::spawn(move || {
      let len: usize = numbers.len();

      let sum: usize = numbers.iter().sum();

      sum / len
    });

    let average: usize = t.join().unwrap();

    println!("average: {average}");
  }
}
