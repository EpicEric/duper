use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    int: usize,
    string: String,
    bools: Vec<bool>,
    map: HashMap<String, (i32, ())>,
}

#[test]
fn serialize() {
    let value: Test = serde_duper::from_str(
        r##"
        {
            int: 42,
            "string": r#"Hello   world!"#,
            bools: [true, true, false,],
            map: {
                r#"quantum"#: X-Measurement((-7, null)),
            },
        }
    "##,
    )
    .unwrap();
    assert_eq!(value.int, 42);
    assert_eq!(value.string, "Hello   world!");
    assert_eq!(value.bools, vec![true, true, false]);
    assert_eq!(value.map.len(), 1);
    assert_eq!(value.map["quantum"], (-7, ()));
    serde_duper::to_string(&value).unwrap();
}
