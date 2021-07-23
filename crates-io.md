The `casserole` crate provides a custom derive and a trait to perform
break-down serialization and de-serialization of Rust types into stores.

The most common use case is to break down large objects to be stored in
content-addressable storage, like in a Git database. Hence the name
'CAS-ser-role'.

The trait which Casserole auto-derives generates smaller types that contain
references to keys instead of the original data. For example `HashMap<String,
BigValue>` is replaced with `HashMap<String, S::Key>` where `S` is a type
parameter to a user-provided storage engine. In addition, fields on which the
`store` attribute is given e.g. `#[casserole(store)]`, are also replaced with
`S::Key`.

For example:

```rust
/// Example Tree to be stored in the database
#[derive(Casserole)]
struct Node {
    header: String,

    // Tells that 'map' is replaced by a database key in the type returned from
    // the 'casserole' trait method. The 'decasserole' trait method will do the
    // reverse, restoring it from the database.
    #[casserole(store)]
    map: BTreeMap<String, Node>,
}
```

Basic usage demonstration (given `big_value` as a large value to work with):

```rust
// Create a our serde-ready type for the root. `stored_root` is our unique
// representation for `big_value`, but it is very small, like a Git hash.
let stored_root = big_value.casserole(&mut store).unwrap();

// <...do other stuff...>

// Restore the origin value from the database
let restored_big_value = Casserole::decasserole(&stored, &mut store).unwrap();
assert_eq!(restored_big_value, big_value);
```
