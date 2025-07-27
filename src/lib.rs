use std::sync::{LazyLock, Once};

mod ch01_p002_threads;
mod ch01_p005_scoped;
mod ch01_p007_shared;
mod ch01_p018_locking;
mod ch01_p024_waiting;
mod ch02_p032_atomic;
mod ch02_p035_example;
mod ch02_p038_example;
mod ch02_p039_example;
mod ch02_p041_example;
mod ch02_p042_compare;

static TRACING_INIT: LazyLock<Once> = LazyLock::new(|| Once::new());

#[allow(dead_code)]
fn init_tracing() {
  TRACING_INIT.call_once(|| {
    // https://www.reddit.com/r/rust/
    //  comments/18shil2/idiomatic_way_to_use_tracing_log_framework_in/

    tracing_subscriber::fmt::init();
  });
}
