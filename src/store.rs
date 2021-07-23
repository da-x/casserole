//! Definition of the `Store` trait.
use serde::{de::DeserializeOwned, Serialize};

/// A `Store` is an abstraction for Casserole to put the translated values.
pub trait Store {
    /// Error returned by the Store's operation.
    type Error;

    /// The keys used to fetch stored values.
    type Key: Serialize + DeserializeOwned;

    /// Save a value to the store and return a key.
    fn put<T>(&mut self, value: &T) -> Result<Self::Key, Self::Error>
    where
        T: Serialize;

    /// Fetch a value from the store based on a key.
    fn get<T>(&mut self, key: &Self::Key) -> Result<Option<T>, Self::Error>
    where
        T: DeserializeOwned;
}

#[cfg(feature = "example-store")]
pub mod json;
