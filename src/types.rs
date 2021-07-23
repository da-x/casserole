/// Implementation of various types for Casserole.
mod basic;
mod r#box;
mod core;
#[cfg(feature = "rc")]
mod sync;
mod map;
mod tuples;
