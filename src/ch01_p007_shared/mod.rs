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
};

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test1() {
    static X: [i32; 3] = [
      1, 2, 3,
    ];

    let join_handle_0: JoinHandle<&[i32; 3]> = thread::spawn(|| dbg!(&X));

    let join_handle_1: JoinHandle<&[i32; 3]> = thread::spawn(|| dbg!(&X));

    let result_0: Result<&[i32; 3], Box<dyn Any + Send + 'static>> =
      join_handle_0.join();

    let result_1: Result<&[i32; 3], Box<dyn Any + Send + 'static>> =
      join_handle_1.join();

    let x_0: &[i32; 3] = result_0.unwrap();

    let x_1: &[i32; 3] = result_1.unwrap();

    assert_eq!(x_0, &X);

    assert_eq!(x_1, &X);
  }

  #[test]
  fn test2() {
    let x: &'static [i32; 3] = Box::leak(Box::new([
      1, 2, 3,
    ]));

    let join_handle_0: JoinHandle<&[i32; 3]> = thread::spawn(move || dbg!(x));

    let join_handle_1: JoinHandle<&[i32; 3]> = thread::spawn(move || dbg!(x));

    let result_0: Result<&[i32; 3], Box<dyn Any + Send + 'static>> =
      join_handle_0.join();

    let result_1: Result<&[i32; 3], Box<dyn Any + Send + 'static>> =
      join_handle_1.join();

    let x_0: &[i32; 3] = result_0.unwrap();

    let x_1: &[i32; 3] = result_1.unwrap();

    assert_eq!(x_0, x);

    assert_eq!(x_1, x);
  }

  #[test]
  fn test3() {
    let a: Arc<[i32; 3]> = Arc::new([
      1, 2, 3,
    ]);

    let b: Arc<[i32; 3]> = a.clone();

    let join_handle_a: JoinHandle<Arc<[i32; 3]>> =
      thread::spawn(move || dbg!(a));

    let join_handle_b: JoinHandle<Arc<[i32; 3]>> =
      thread::spawn(move || dbg!(b));

    let result_a: Result<Arc<[i32; 3]>, Box<dyn Any + Send + 'static>> =
      join_handle_a.join();

    let result_b: Result<Arc<[i32; 3]>, Box<dyn Any + Send + 'static>> =
      join_handle_b.join();

    let x_a: Arc<[i32; 3]> = result_a.unwrap();

    let x_b: Arc<[i32; 3]> = result_b.unwrap();

    assert_eq!(*x_a, *x_b);
  }

  #[test]
  fn test4() {
    let a: Arc<[i32; 3]> = Arc::new([
      1, 2, 3,
    ]);

    let join_handle_a: JoinHandle<Arc<[i32; 3]>> = thread::spawn({
      let a: Arc<[i32; 3]> = a.clone();

      move || dbg!(a)
    });

    let join_handle_b: JoinHandle<Arc<[i32; 3]>> = thread::spawn({
      let a: Arc<[i32; 3]> = a.clone();

      move || dbg!(a)
    });

    let result_a: Result<Arc<[i32; 3]>, Box<dyn Any + Send + 'static>> =
      join_handle_a.join();

    let result_b: Result<Arc<[i32; 3]>, Box<dyn Any + Send + 'static>> =
      join_handle_b.join();

    let x_a: Arc<[i32; 3]> = result_a.unwrap();

    let x_b: Arc<[i32; 3]> = result_b.unwrap();

    assert_eq!(*x_a, *a);

    assert_eq!(*x_b, *a);
  }
}
