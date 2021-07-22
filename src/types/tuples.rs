use crate::{ser::Casserole, store::Store};

macro_rules! tuple_type {
    ($($letters:ident: $numbers:tt),*) => {
        impl<S, $($letters),*> Casserole<S> for ($($letters),*)
        where
            S: Store,
            $($letters: Casserole<S>),*
        {
            type Target = (
                $(<$letters as Casserole<S>>::Target),*
            );

            fn casserole(&self, store: &mut S) -> Result<Self::Target, S::Error> {
                Ok((
                    $(self.$numbers.casserole(store)?),*
                ))
            }

            fn decasserole(target: &Self::Target, store: &mut S) -> Result<Self, S::Error> {
                Ok((
                    $(
                        (<$letters as Casserole<S>>::decasserole(
                            &target.$numbers,
                            store,
                        ))?
                    ),*
                ))
            }
        }
    };
}

tuple_type!{A:0 , B:1}
tuple_type!{A:0 , B:1 , C:2}
tuple_type!{A:0 , B:1 , C:2, D:3}
