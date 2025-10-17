use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};
use serde_duper::bytes::{self, ByteBuf};

#[test]
fn handle_struct() {
    #[derive(Debug, Serialize, Deserialize)]
    struct Test<'a> {
        int: usize,
        string: String,
        str: &'a str,
        bools: Vec<bool>,
        #[serde(borrow, with = "bytes")]
        cow: Cow<'a, [u8]>,
        map: HashMap<String, (i32, (), ByteBuf)>,
    }

    let value: Test = serde_duper::from_string(
        r##"
        {
            int: 42,
            "string": r#"Hello   world!"#,
            str: "duper",
            bools: [true, true, false,],
            cow: b"cool",
            map: {
                r#"quantum"#: Measurement((-7, "whatever", b"crazy")),
            },
        }
    "##,
    )
    .unwrap();
    assert_eq!(value.int, 42);
    assert_eq!(value.string, "Hello   world!");
    assert_eq!(value.str, "duper");
    assert_eq!(value.bools, vec![true, true, false]);
    assert_eq!(value.map.len(), 1);
    assert_eq!(value.map["quantum"], (-7, (), b"crazy".to_vec().into()));
    assert_eq!(
        serde_duper::to_string(&value).unwrap(),
        r#"Test({int: 42, string: "Hello   world!", str: "duper", bools: [true, true, false], cow: b"cool", map: {quantum: (-7, (,), b"crazy")}})"#
    );
}

#[test]
fn handle_enum() {
    #[derive(Debug, Serialize, Deserialize)]
    enum Test {
        Foo { a: String, b: i32 },
        Bar(String),
        Baz(f64, ByteBuf),
        Dup(HashMap<String, bool>),
        Qux,
    }

    let value: Test = serde_duper::from_string(
        r##"
        Test({
            Foo: {
                a: "Hello world!",
                b: 42,
            },
        })
    "##,
    )
    .unwrap();
    let Test::Foo { a, b } = value else {
        panic!("invalid value: {value:?}");
    };
    assert_eq!(a, "Hello world!");
    assert_eq!(b, 42);
    assert_eq!(
        serde_duper::to_string(&Test::Foo { a, b }).unwrap(),
        r#"Test({Foo: {a: "Hello world!", b: 42}})"#
    );

    let value: Test = serde_duper::from_string(
        r##"
        Test({
            Bar: "duper",
        })
    "##,
    )
    .unwrap();
    let Test::Bar(string) = value else {
        panic!("invalid value: {value:?}");
    };
    assert_eq!(string, "duper");
    assert_eq!(b, 42);
    assert_eq!(
        serde_duper::to_string(&Test::Bar(string)).unwrap(),
        r#"Test({Bar: "duper"})"#
    );

    let value: Test = serde_duper::from_string(
        r##"
        Test({
            Baz: (
                1.23,
                b"bytes",
            ),
        })
    "##,
    )
    .unwrap();
    let Test::Baz(float, bytes) = value else {
        panic!("invalid value: {value:?}");
    };
    assert_eq!(float, 1.23);
    assert_eq!(bytes, b"bytes");
    assert_eq!(
        serde_duper::to_string(&Test::Baz(float, bytes)).unwrap(),
        r#"Test({Baz: (1.23, b"bytes")})"#
    );

    let value: Test = serde_duper::from_string(
        r##"
        Test({
            Qux: (,),
        })
    "##,
    )
    .unwrap();
    let Test::Qux = value else {
        panic!("invalid value: {value:?}");
    };
    let value: Test = serde_duper::from_string(
        r##"
        Test({
            Qux: null,
        })
    "##,
    )
    .unwrap();
    let Test::Qux = value else {
        panic!("invalid value: {value:?}");
    };
    let value: Test = serde_duper::from_string(
        r##"
        Test("Qux")
    "##,
    )
    .unwrap();
    let Test::Qux = value else {
        panic!("invalid value: {value:?}");
    };
    assert_eq!(
        serde_duper::to_string(&Test::Qux).unwrap(),
        r#"Test("Qux")"#
    );
}

#[test]
fn handle_enum_internally_tagged() {
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(tag = "type")]
    enum Test {
        Foo { a: String, b: i32 },
        Dup(HashMap<String, bool>),
        Qux,
    }

    let value: Test = serde_duper::from_string(
        r##"
        Test({
            type: "Foo",
            a: "Hello world!",
            b: 42,
        })
    "##,
    )
    .unwrap();
    let Test::Foo { a, b } = value else {
        panic!("invalid value: {value:?}");
    };
    assert_eq!(a, "Hello world!");
    assert_eq!(b, 42);
    assert_eq!(
        serde_duper::to_string(&Test::Foo { a, b }).unwrap(),
        r#"Test({type: "Foo", a: "Hello world!", b: 42})"#
    );

    let value: Test = serde_duper::from_string(
        r##"
        Test({
            type: "Dup",
            duper: true,
        })
    "##,
    )
    .unwrap();
    let Test::Dup(map) = value else {
        panic!("invalid value: {value:?}");
    };
    assert_eq!(map.get("duper"), Some(&true));
    assert_eq!(
        serde_duper::to_string(&Test::Dup(map)).unwrap(),
        // TODO: Find a way to fix this
        // r#"Test({type: "Dup", duper: true})"#
        r#"{type: "Dup", duper: true}"#
    );

    let value: Test = serde_duper::from_string(
        r##"
        Test({
            type: "Qux",
        })
    "##,
    )
    .unwrap();
    let Test::Qux = value else {
        panic!("invalid value: {value:?}");
    };
    assert_eq!(
        serde_duper::to_string(&Test::Qux).unwrap(),
        r#"Test({type: "Qux"})"#
    );
}

#[test]
fn handle_enum_adjacently_tagged() {
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(tag = "tag", content = "content")]
    enum Test {
        Foo { a: String, b: i32 },
        Bar(String),
        Baz(f64, ByteBuf),
        Dup(HashMap<String, bool>),
        Qux,
    }

    let value: Test = serde_duper::from_string(
        r##"
        Test({
            tag: "Foo",
            content: {
                a: "Hello world!",
                b: 42,
            },
        })
    "##,
    )
    .unwrap();
    let Test::Foo { a, b } = value else {
        panic!("invalid value: {value:?}");
    };
    assert_eq!(a, "Hello world!");
    assert_eq!(b, 42);
    assert_eq!(
        serde_duper::to_string(&Test::Foo { a, b }).unwrap(),
        r#"Test({tag: Test("Foo"), content: Foo({a: "Hello world!", b: 42})})"#
    );

    let value: Test = serde_duper::from_string(
        r##"
        Test({
            tag: "Bar",
            content: "duper",
        })
    "##,
    )
    .unwrap();
    let Test::Bar(string) = value else {
        panic!("invalid value: {value:?}");
    };
    assert_eq!(string, "duper");
    assert_eq!(b, 42);
    assert_eq!(
        serde_duper::to_string(&Test::Bar(string)).unwrap(),
        r#"Test({tag: Test("Bar"), content: "duper"})"#
    );

    let value: Test = serde_duper::from_string(
        r##"
        Test({
            tag: "Baz",
            content: (
                1.23,
                b"bytes",
            ),
        })
    "##,
    )
    .unwrap();
    let Test::Baz(float, bytes) = value else {
        panic!("invalid value: {value:?}");
    };
    assert_eq!(float, 1.23);
    assert_eq!(bytes, b"bytes");
    assert_eq!(
        serde_duper::to_string(&Test::Baz(float, bytes)).unwrap(),
        r#"Test({tag: Test("Baz"), content: (1.23, b"bytes")})"#
    );

    let value: Test = serde_duper::from_string(
        r##"
        Test({
            tag: "Dup",
            content: {
                random: true,
            }
        })
    "##,
    )
    .unwrap();
    let Test::Dup(map) = value else {
        panic!("invalid value: {value:?}");
    };
    assert_eq!(map.get("random"), Some(&true));
    assert_eq!(
        serde_duper::to_string(&Test::Dup(map)).unwrap(),
        r#"Test({tag: Test("Dup"), content: {random: true}})"#
    );

    let value: Test = serde_duper::from_string(
        r##"
        Test({tag: "Qux"})
    "##,
    )
    .unwrap();
    let Test::Qux = value else {
        panic!("invalid value: {value:?}");
    };
    let value: Test = serde_duper::from_string(
        r##"
        Test({tag: "Qux", content: null})
    "##,
    )
    .unwrap();
    let Test::Qux = value else {
        panic!("invalid value: {value:?}");
    };
    let value: Test = serde_duper::from_string(
        r##"
        Test({tag: "Qux", content: (,)})
    "##,
    )
    .unwrap();
    let Test::Qux = value else {
        panic!("invalid value: {value:?}");
    };
    assert_eq!(
        serde_duper::to_string(&Test::Qux).unwrap(),
        r#"Test({tag: Test("Qux")})"#
    );
}

#[test]
fn handle_enum_untagged() {
    #[derive(Debug, Serialize, Deserialize)]
    #[serde(untagged)]
    enum Test {
        Foo { a: String, b: i32 },
        Bar(String),
        Baz(f64, ByteBuf),
        Dup(HashMap<String, bool>),
        Qux,
    }

    let value: Test = serde_duper::from_string(
        r##"
        Test({
            a: "Hello world!",
            b: 42,
        })
    "##,
    )
    .unwrap();
    let Test::Foo { a, b } = value else {
        panic!("invalid value: {value:?}");
    };
    assert_eq!(a, "Hello world!");
    assert_eq!(b, 42);
    assert_eq!(
        serde_duper::to_string(&Test::Foo { a, b }).unwrap(),
        r#"Test({a: "Hello world!", b: 42})"#
    );

    let value: Test = serde_duper::from_string(
        r##"
        Test("duper")
    "##,
    )
    .unwrap();
    let Test::Bar(string) = value else {
        panic!("invalid value: {value:?}");
    };
    assert_eq!(string, "duper");
    assert_eq!(b, 42);
    assert_eq!(
        serde_duper::to_string(&Test::Bar(string)).unwrap(),
        r#""duper""#
    );

    let value: Test = serde_duper::from_string(
        r##"
        Test((
            1.23,
            b"bytes",
        ))
    "##,
    )
    .unwrap();
    let Test::Baz(float, bytes) = value else {
        panic!("invalid value: {value:?}");
    };
    assert_eq!(float, 1.23);
    assert_eq!(bytes, b"bytes");
    assert_eq!(
        serde_duper::to_string(&Test::Baz(float, bytes)).unwrap(),
        r#"(1.23, b"bytes")"#
    );

    let value: Test = serde_duper::from_string(
        r##"
        Test({
            random: true
        })
    "##,
    )
    .unwrap();
    let Test::Dup(map) = value else {
        panic!("invalid value: {value:?}");
    };
    assert_eq!(map.get("random"), Some(&true));
    assert_eq!(
        serde_duper::to_string(&Test::Dup(map)).unwrap(),
        r#"{random: true}"#
    );

    let value: Test = serde_duper::from_string(
        r##"
        Test((,))
    "##,
    )
    .unwrap();
    let Test::Qux = value else {
        panic!("invalid value: {value:?}");
    };
    let value: Test = serde_duper::from_string(
        r##"
        Test(null)
    "##,
    )
    .unwrap();
    let Test::Qux = value else {
        panic!("invalid value: {value:?}");
    };
    assert_eq!(serde_duper::to_string(&Test::Qux).unwrap(), r#"(,)"#);
}
