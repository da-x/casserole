use crate::{ser::Casserole, store::Store};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[serde(
    bound = "S::Key: DeserializeOwned + Serialize, <K as Casserole<S>>::Target: DeserializeOwned \
             + Serialize"
)]
pub struct StoredValuesBTreeMap<S, K>
where
    K: Casserole<S>,
    <K as Casserole<S>>::Target: Eq + std::hash::Hash + Serialize + Ord,
    S: Store,
{
    map: BTreeMap<<K as Casserole<S>>::Target, S::Key>,
}

/// Implementation of HashMap in casserole is done as a sorted map, where the
/// values are proxied by Store keys. In the future, we may use a tree of
/// key-value pairs so that slightly different maps will be stored more
/// efficiently.
impl<S, K, V> Casserole<S> for HashMap<K, V>
where
    S: Store,
    K: DeserializeOwned + Eq + std::hash::Hash + Ord,
    K: Casserole<S>,
    V: Casserole<S>,
    <K as Casserole<S>>::Target: DeserializeOwned + Eq + std::hash::Hash + Ord,
    <V as Casserole<S>>::Target: DeserializeOwned,
{
    type Target = StoredValuesBTreeMap<S, K>;

    fn casserole(&self, store: &mut S) -> Result<Self::Target, S::Error> {
        let mut v = StoredValuesBTreeMap {
            map: BTreeMap::new(),
        };

        for (key, value) in self.iter() {
            let key = Casserole::<S>::casserole(key, store)?;
            let value = Casserole::<S>::casserole(value, store)?;
            let value = store.put(&value)?;
            v.map.insert(key, value);
        }

        Ok(v)
    }

    fn decasserole(target: &Self::Target, store: &mut S) -> Result<Self, S::Error> {
        let mut v = HashMap::new();

        for (key, value) in target.map.iter() {
            let key = Casserole::<S>::decasserole(key, store)?;
            let value: <V as Casserole<S>>::Target = store.get(value)?.expect("missing key");
            let value = Casserole::<S>::decasserole(&value, store)?;
            v.insert(key, value);
        }

        Ok(v)
    }
}

/// Implementation of HashMap in casserole is done as a sorted map, where the
/// values are proxied by Store keys. In the future, we may use a tree of
/// key-value pairs so that slightly different maps will be stored more
/// efficiently.
impl<S, K, V> Casserole<S> for BTreeMap<K, V>
where
    S: Store,
    K: DeserializeOwned + Eq + std::hash::Hash + Ord,
    K: Casserole<S>,
    V: Casserole<S>,
    <K as Casserole<S>>::Target: DeserializeOwned + Eq + std::hash::Hash + Ord,
    <V as Casserole<S>>::Target: DeserializeOwned,
{
    type Target = StoredValuesBTreeMap<S, K>;

    fn casserole(&self, store: &mut S) -> Result<Self::Target, S::Error> {
        let mut v = StoredValuesBTreeMap {
            map: BTreeMap::new(),
        };

        for (key, value) in self.iter() {
            let key = Casserole::<S>::casserole(key, store)?;
            let value = Casserole::<S>::casserole(value, store)?;
            let value = store.put(&value)?;
            v.map.insert(key, value);
        }

        Ok(v)
    }

    fn decasserole(target: &Self::Target, store: &mut S) -> Result<Self, S::Error> {
        let mut v = BTreeMap::new();

        for (key, value) in target.map.iter() {
            let key = Casserole::<S>::decasserole(key, store)?;
            let value: <V as Casserole<S>>::Target = store.get(value)?.expect("missing key");
            let value = Casserole::<S>::decasserole(&value, store)?;
            v.insert(key, value);
        }

        Ok(v)
    }
}
