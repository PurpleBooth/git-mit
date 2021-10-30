//! Standard ways to interact with different parts of the tool

#![warn(
    rust_2018_idioms,
    unused,
    rust_2021_compatibility,
    nonstandard_style,
    future_incompatible,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs
)]

pub mod console;
pub mod external;
pub mod lints;
pub mod mit;

pub mod relates;
/// Quickcheck support
#[cfg(test)]
extern crate quickcheck;

/// Quickcheck support
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
