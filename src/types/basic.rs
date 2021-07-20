use crate::{ser::Casserole, store::Store};

macro_rules! identity_type {
    ($ty:ty) => {
        impl<S> Casserole<S> for $ty
        where
            S: Store,
        {
            type Target = $ty;

            fn casserole(&self, _store: &mut S) -> Result<Self::Target, S::Error> {
                Ok(*self)
            }

            fn decasserole(target: &Self::Target, _store: &mut S) -> Result<Self, S::Error> {
                Ok(*target)
            }
        }
    };
}

macro_rules! identity_clone_type {
    ($ty:ty) => {
        impl<S> Casserole<S> for $ty
        where
            S: Store,
        {
            type Target = $ty;

            fn casserole(&self, _store: &mut S) -> Result<Self::Target, S::Error> {
                Ok(self.clone())
            }

            fn decasserole(target: &Self::Target, _store: &mut S) -> Result<Self, S::Error> {
                Ok(target.clone())
            }
        }
    };
}


identity_type!(());
identity_type!(bool);
identity_type!(u8);
identity_type!(u16);
identity_type!(u32);
identity_type!(u64);
identity_type!(usize);
identity_type!(i8);
identity_type!(i16);
identity_type!(i32);
identity_type!(i64);
identity_type!(isize);
identity_clone_type!(String);
