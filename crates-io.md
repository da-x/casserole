The `casserole` crate provides a custom derive and a trait to perform break-down
serialization and serialization of Rust types to into content-addressable storage.

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
// Obtain a reference to an example store (can be persistent or in-memory).
// `JSONMemorySHA1` is like the Git database, but with base64 string as keys,
// and serde-json output as values.
let mut store = casserole::store::json::JSONMemorySHA1::new();

// Create a our serde-ready type for the root. `stored_root` is our unique
// representation for `big_value`, but it is very small, like a 'Git hash'.
let stored_root = big_value.casserole(&mut store).unwrap();

// <...do other stuff...>

// Restore the origin value from the database
let restored_big_value = Casserole::decasserole(&stored, &mut store).unwrap();
assert_eq!(restored_big_value, big_value);
```
