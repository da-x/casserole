use casserole_derive::derive_casserole_prelude;

derive_casserole_prelude! {
    enum Option<T> {
        None,
        Some(T),
    }
}

derive_casserole_prelude! {
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}
