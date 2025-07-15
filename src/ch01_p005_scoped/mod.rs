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
  thread::{self, JoinHandle, Scope, Thread, ThreadId},
};

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    let numbers: Vec<i32> = vec![
      1, 2, 3,
    ];

    thread::scope(|s: &Scope<'_, '_>| {
      s.spawn(|| {
        println!("length: {}", numbers.len());
      });

      s.spawn(|| {
        for n in &numbers {
          println!("{n}");
        }
      });
    });
  }
}
