use crate::{ser::Casserole, store::Store};
use std::ops::Deref;

impl<S, T> Casserole<S> for Box<T>
where
    S: Store,
    T: Casserole<S>,
{
    type Target = Box<<T as Casserole<S>>::Target>;

    fn casserole(&self, store: &mut S) -> Result<Self::Target, S::Error> {
        Ok(Box::new(self.deref().casserole(store)?))
    }

    fn decasserole(target: &Self::Target, store: &mut S) -> Result<Self, S::Error> {
        Ok(Box::new(<T as Casserole<S>>::decasserole(
            target.deref(),
            store,
        )?))
    }
}
