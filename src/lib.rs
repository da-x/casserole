//! Casserole - a serde-based serializer/deserializer with a pluggable
//! Content-Addressable Store as backing store so that internal data use
//! references into the store.
//!
//! The result serialized objects are small and are gaining dedup and efficient
//! diffability due to content-addressable references.
#![deny(missing_docs)]

mod ser;
pub mod store;
pub use casserole_derive::Casserole;

pub use store::Store;
pub use ser::Casserole;

mod types;
