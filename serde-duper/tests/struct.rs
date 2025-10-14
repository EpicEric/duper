use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test<'a> {
    int: usize,
    string: String,
    str: &'a str,
    bools: Vec<bool>,
    map: HashMap<String, (i32, (), &'a [u8])>,
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
            map: {
                r#"quantum"#: Measurement((-7, "whatever", b"abc")),
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
    assert_eq!(value.map["quantum"], (-7, (), &b"abc"[..]));
    serde_duper::to_string(&value).unwrap();
}
