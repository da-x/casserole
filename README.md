# casserole &emsp; [![Build Status]][travis] [![Latest Version]][crates.io] [![Docs badge]][Docs link] [![License badge]][License link]

[Build Status]: https://api.travis-ci.org/da-x/casserole.svg?branch=master
[travis]: https://travis-ci.org/da-x/casserole
[Latest Version]: https://img.shields.io/crates/v/casserole.svg
[crates.io]: https://crates.io/crates/casserole
[License badge]: https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg
[License link]: https://travis-ci.org/da-x/casserole
[Docs badge]: https://docs.rs/casserole/badge.svg
[Docs link]: https://docs.rs/casserole

The `casserole` crate provides a custom derive and a trait to perform
break-down serialization and de-serialization of Rust types to into stores.

Most common use case is to break down large objects to be stored in
content-addressable storage, like in a Git database. Hence the name
'CAS-ser-role'.

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

## Bigger example

See the output of the [example](example/main.rs), that demonstrates how a big map
can be stored.


```
cargo run --example casserole-main --features="json-store"
```

```
Casseroled value:

    {"header":"This is a header","map":"<~qeR*N3Rnt7@>:%S/YY.75bj&+~>"}

Stored:

    <~H6^Sl@aFY>Z7&GQ)I7e6";^uZ~> : {"header":"A different sub item","map":"<~1rX#GJ]0qpYGM>gGMhu8ZU6AI~>"}
    <~9da9I?4'pU=L)h3U:qYePcH-U~> : {"header":"Header","map":"<~(2R.\\0-3,2.L4\"Y5>.hBh=;7A~>"}
    <~1rX#GJ]0qpYGM>gGMhu8ZU6AI~> : {"map":{}}
    <~Od]K[j+dl0Zp#g^7PMg1gAL&k~> : {"header":"Dublicate sub item","map":"<~1rX#GJ]0qpYGM>gGMhu8ZU6AI~>"}
    <~(2R.\0-3,2.L4"Y5>.hBh=;7A~> : {"map":{"x":"<~Od]K[j+dl0Zp#g^7PMg1gAL&k~>","y":"<~Od]K[j+dl0Zp#g^7PMg1gAL&k~>","z":"<~H6^Sl@aFY>Z7&GQ)I7e6\";^uZ~>"}}
    <~qeR*N3Rnt7@>:%S/YY.75bj&+~> : {"map":{"A large duplicate sub-tree":"<~9da9I?4'pU=L)h3U:qYePcH-U~>","A unique-subtree":"<~Od]K[j+dl0Zp#g^7PMg1gAL&k~>"}}
```


## License

`casserole` is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `casserole` by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
