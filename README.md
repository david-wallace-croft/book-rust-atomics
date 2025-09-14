# Book Rust Atomics

[![MIT licensed][mit-badge]][mit-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/david-wallace-croft/book-rust-atomics/blob/main/LICENSE.txt

- Code adapted from the book "Rust Atomics and Locks" by Mara Bos
  - https://www.oreilly.com/library/view/rust-atomics-and/9781098119430/
  - https://www.oreilly.com/catalog/errata.csp?isbn=0636920635048
  - https://marabos.nl/atomics/
  - https://github.com/m-ou-se/rust-atomics-and-locks

## Usage

- cargo test --lib -- --show-output ch01_p002_threads::test::test1
- cargo test --lib -- --show-output ch01_p002_threads::test::test2
- cargo test --lib -- --show-output ch01_p002_threads::test::test3
- cargo test --lib -- --show-output ch01_p005_scoped::test::test1
- cargo test --lib -- --show-output ch01_p007_shared::test::test1
- cargo test --lib -- --show-output ch01_p007_shared::test::test2
- cargo test --lib -- --show-output ch01_p007_shared::test::test3
- cargo test --lib -- --show-output ch01_p007_shared::test::test4
- cargo test --lib -- --show-output ch01_p018_locking::test::test1
- cargo test --lib -- --show-output ch01_p018_locking::test::test2
- cargo test --lib -- --show-output ch01_p018_locking::test::test3
- cargo test --lib -- ch01_p024_waiting::test::test1
- cargo test --lib -- ch01_p024_waiting::test::test2
- cargo test --lib -- ch02_p032_atomic::test::test1
- cargo test --lib -- ch02_p032_atomic::test::test2
- cargo test --lib -- ch02_p032_atomic::test::test3
- cargo test --lib -- ch02_p035_example::test::test1
- cargo test --lib -- ch02_p038_example::test::test1
- cargo test --lib -- ch02_p039_example::test::test1
- cargo test --lib -- ch02_p041_example::test::test1
- cargo test --lib -- ch02_p042_compare::test::test1
- cargo test --lib -- ch02_p044_example::test::test1
- cargo test --lib -- ch02_p044_example::test::test2
- cargo test --lib -- ch02_p046_example::test::test1
- cargo test --lib -- ch03_p057_release::test::test1
- cargo test --lib -- ch03_p060_example::test::test1
- cargo test --lib -- ch03_p062_example::test::test1
- cargo test --lib -- ch03_p066_sequentially::test::test1
- cargo test --lib -- ch03_p067_fences::test::test1
- cargo test --lib -- ch04_p075_minimal::test::test1
- cargo test --lib -- ch04_p078_unsafe::test::test1
- cargo test --lib -- ch04_p080_safe::test::test1
- cargo test --lib -- ch05_p085_simple::test::test1
- cargo test --lib -- ch05_p087_unsafe::test::test1
- cargo test --lib -- ch05_p090_safety::test::test1
- cargo test --lib -- ch05_p093_using::test::test1
- cargo test --lib -- ch05_p094_safety::test::test1
- cargo test --lib -- ch05_p098_borrowing::test::test1
- cargo test --lib -- ch05_p101_blocking::test::test1
- cargo test --lib -- ch06_p105_basic::test::test1
- cargo test --lib -- ch06_p105_basic::test::test2
- cargo test --lib -- ch06_p111_weak::test::test1
- cargo test --lib -- ch06_p118_optimizing::test::test1
- cargo test --lib -- ch09_p183_mutex::test::test1
- cargo test --lib -- ch09_p186_avoiding::test::test1
- cargo test --lib -- ch09_p188_optimizing::test::test1
- cargo test --lib -- ch09_p191_benchmarking::test::test1
- cargo test --lib -- ch09_p191_benchmarking::test::test2
- cargo test --lib -- ch09_p193_condition::test::test1
- cargo test --lib -- ch09_p198_avoiding::test::test1

## History

- 2025-07-14: Initial release
