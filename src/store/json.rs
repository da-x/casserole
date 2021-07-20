//! A simple store of serialized casserole values, based on a SHA1 key
//! of the serialized values. Similar in fashion to the Git database.

use std::collections::HashMap;

use crate::store::Store;
use crypto::{digest::Digest, sha1::Sha1};
use serde::{de::DeserializeOwned, Serialize};

/// A simple store of serialized casserole values, based on a SHA1 key
/// of the serialized values. Similar in fashion to the Git database.
pub struct JSONMemorySHA1 {
    map: HashMap<String, Vec<u8>>,
}

impl JSONMemorySHA1 {
    /// Create a new JSONMemorySHA1 map.
    pub fn new() -> Self {
        JSONMemorySHA1 {
            map: HashMap::new(),
        }
    }

    /// Return a reference to the inner HashMap of this object.
    pub fn items(&self) -> &HashMap<String, Vec<u8>> {
        &self.map
    }
}

impl Store for JSONMemorySHA1 {
    type Error = serde_json::Error;
    type Key = String;

    fn put<T>(&mut self, value: &T) -> Result<Self::Key, Self::Error>
    where
        T: Serialize,
    {
        let value = serde_json::ser::to_vec(&value)?;
        use std::collections::hash_map;

        let mut hasher = Sha1::new();
        hasher.input(value.as_ref());

        let mut key = vec![];
        key.resize(hasher.output_bytes(), 0);
        hasher.result(&mut key);

        let encoded = ascii85::encode(&key);

        match self.map.entry(encoded.clone()) {
            hash_map::Entry::Vacant(v) => v.insert(value),
            hash_map::Entry::Occupied(o) => o.into_mut(),
        };

        Ok(encoded)
    }

    fn get<T>(&mut self, key: &Self::Key) -> Result<Option<T>, Self::Error>
    where
        T: DeserializeOwned,
    {
        if let Some(value) = self.map.get(key) {
            Ok(Some(serde_json::de::from_slice(value)?))
        } else {
            Ok(None)
        }
    }
}
