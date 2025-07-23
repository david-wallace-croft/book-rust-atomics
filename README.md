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

## History

- 2025-07-14: Initial release
