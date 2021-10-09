extern crate serde;

pub mod console;
pub mod external;
pub mod lints;
pub mod mit;
pub mod relates;
#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;
