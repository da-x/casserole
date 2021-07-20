use casserole::Casserole;
use std::collections::BTreeMap;
use maplit::btreemap;

#[derive(Casserole, Debug, Eq, PartialEq)]
struct Node {
    header: String,

    #[casserole(store)]
    map: BTreeMap<String, Node>,
}

impl Node {
    fn new(header: &'static str, map: BTreeMap<String, Node>) -> Self {
        Node { header: header.to_owned(), map }
    }
}

fn main() {
    let big_value = Node::new(
        "This is a header",
        btreemap!{
            "A unique-subtree".to_owned() => Node::new(
                "Dublicate sub item", btreemap!{}
            ),
            "A large duplicate sub-tree".to_owned() => Node::new(
                "Header",
                btreemap!{
                    "x".to_owned() => Node::new("Dublicate sub item", btreemap!{}),
                    "y".to_owned() => Node::new("Dublicate sub item", btreemap!{}),
                    "z".to_owned() => Node::new("A different sub item", btreemap!{}),
                }
            ),
            "A large duplicate sub-tree".to_owned() => Node::new(
                "Header",
                btreemap!{
                    "x".to_owned() => Node::new("Dublicate sub item", btreemap!{}),
                    "y".to_owned() => Node::new("Dublicate sub item", btreemap!{}),
                    "z".to_owned() => Node::new("A different sub item", btreemap!{}),
                }
            )
        }
    );

    let mut store = casserole::store::json::JSONMemorySHA1::new();

    // Store
    let stored = big_value.casserole(&mut store).unwrap();

    println!("Casseroled value:");
    println!("");
    println!("    {}", serde_json::ser::to_string(&stored).unwrap());
    println!("");
    println!("Stored:");
    println!("");

    for (k, v) in store.items() {
        println!("    {} : {}", k, std::str::from_utf8(&v).unwrap());
    }

    let restored: Node = Casserole::decasserole(&stored, &mut store).unwrap();
    assert_eq!(restored, big_value);
}
