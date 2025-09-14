use std::sync::Once;

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
mod ch02_p044_example;
mod ch02_p046_example;
mod ch03_p057_release;
mod ch03_p060_example;
mod ch03_p062_example;
mod ch03_p066_sequentially;
mod ch03_p067_fences;
mod ch04_p075_minimal;
mod ch04_p078_unsafe;
mod ch04_p080_safe;
mod ch05_p085_simple;
mod ch05_p087_unsafe;
mod ch05_p090_safety;
mod ch05_p093_using;
mod ch05_p094_safety;
mod ch05_p098_borrowing;
mod ch05_p101_blocking;
mod ch06_p105_basic;
mod ch06_p111_weak;
mod ch06_p118_optimizing;
mod ch09_p183_mutex;
mod ch09_p186_avoiding;
mod ch09_p188_optimizing;

static TRACING_INIT: Once = Once::new();

#[allow(dead_code)]
fn init_tracing() {
  TRACING_INIT.call_once(|| {
    // https://www.reddit.com/r/rust/
    //  comments/18shil2/idiomatic_way_to_use_tracing_log_framework_in/

    tracing_subscriber::fmt::init();
  });
}
