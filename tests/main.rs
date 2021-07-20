use casserole::Casserole;

use std::collections::BTreeMap;

#[derive(Casserole, Debug, Eq, PartialEq)]
struct Test {
    x: u32,
    y: u32,
    b: bool,

    #[casserole(store)]
    complex: MyMap,
}

#[derive(Casserole, Debug, Eq, PartialEq)]
enum TestX<T, X> {
    Bla { x: u32, y: X },
    Test(T),
    Test2,
}

#[derive(Casserole, Debug, Eq, PartialEq)]
struct MyMap {
    field: u32,

    #[casserole(store)]
    map: BTreeMap<u32, Test>,
}

impl MyMap {
    fn new() -> Self {
        Self {
            field: 0,
            map: BTreeMap::new(),
        }
    }
}

#[test]
fn it_works() {
    println!();

    let mut store = casserole::store::json::JSONMemorySHA1::new();

    let v = MyMap {
        field: 3,
        map: vec![
            (
                10000,
                Test {
                    x: 100,
                    y: 10,
                    b: true,
                    complex: MyMap {
                        field: 3,
                        map: vec![
                            (
                                999,
                                Test {
                                    x: 100,
                                    y: 10,
                                    b: true,
                                    complex: MyMap::new(),
                                },
                            ),
                            (
                                998,
                                Test {
                                    x: 100,
                                    y: 10,
                                    b: true,
                                    complex: MyMap::new(),
                                },
                            ),
                        ]
                        .into_iter()
                        .collect(),
                    },
                },
            ),
            (
                20000,
                Test {
                    x: 100,
                    y: 10,
                    b: true,
                    complex: MyMap {
                        field: 10,
                        map: vec![].into_iter().collect(),
                    },
                },
            ),
        ]
        .into_iter()
        .collect(),
    };

    let _stored = v.casserole(&mut store).unwrap();

    let mut vec: Vec<_> = store
        .items()
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    vec.sort();

    for (k, v) in vec.drain(..) {
        println!("{}: {}", k, std::str::from_utf8(&v).unwrap());
    }

    let restored: MyMap = Casserole::decasserole(&_stored, &mut store).unwrap();
    assert_eq!(restored, v);
}
