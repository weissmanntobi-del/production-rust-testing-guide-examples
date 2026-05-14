//! Production Rust Testing Guide — compilable examples.
//!
//! This crate contains small, focused examples that demonstrate production-friendly
//! Rust testing patterns:
//!
//! - unit tests close to the code,
//! - a valid `#[should_panic]` example,
//! - property-based tests with `proptest`,
//! - mocking with `mockall`,
//! - integration tests under `tests/`,
//! - a Criterion benchmark under `benches/`.
//!
//! # Example
//!
//! ```
//! use production_rust_testing_guide_examples::checked_divide;
//!
//! assert_eq!(checked_divide(10, 2), Ok(5));
//! assert!(checked_divide(10, 0).is_err());
//! ```

pub mod calculator;
pub mod greeter;
pub mod reverse;

pub use calculator::{checked_divide, divide_or_panic, DivideError};
pub use greeter::{render_welcome, EnglishGreeter, Greeter};
pub use reverse::{reverse_chars, reverse_vec};
