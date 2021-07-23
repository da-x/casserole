use crate::{ser::Casserole, store::Store};
use std::sync::Arc;
use std::ops::Deref;

impl<S, T> Casserole<S> for Arc<T>
where
    S: Store,
    T: Casserole<S>,
{
    type Target = Arc<<T as Casserole<S>>::Target>;

    fn casserole(&self, store: &mut S) -> Result<Self::Target, S::Error> {
        Ok(Arc::new(self.deref().casserole(store)?))
    }

    fn decasserole(target: &Self::Target, store: &mut S) -> Result<Self, S::Error> {
        Ok(Arc::new(<T as Casserole<S>>::decasserole(
            target.deref(),
            store,
        )?))
    }
}
