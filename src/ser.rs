use crate::store::Store;

/// The Casserole trait provides reduction and restoration of values to compact
/// form where sub-values are stored in a Store, which can be a content-addressable
/// database.
pub trait Casserole<S>: Sized
where
    S: Store,
{
    /// The target type. Usually contains all the details of the original types
    /// except some sub-values which are stored in a DB and are only accessible
    /// via a key.
    type Target: serde::Serialize;

    /// Serialization that handles folding. The returned value is a folded
    /// representation of the given type, where the folded portions are
    /// saved in a store.
    ///
    /// This function is expected to use the store to perform puts.
    fn casserole(&self, store: &mut S) -> Result<Self::Target, S::Error>;

    /// The reverse of `casserole`, returning the original object. This
    /// function is expected to use the given store to preform gets.
    fn decasserole(target: &Self::Target, store: &mut S) -> Result<Self, S::Error>;
}
