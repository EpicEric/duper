use std::{borrow::Cow, collections::HashMap};

use serde::{Deserialize, Serialize};
use serde_duper::bytes::{self, ByteBuf};

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

#[test]
fn deserialize_struct() {
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
