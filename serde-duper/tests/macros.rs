use serde::{Deserialize, Serialize};
use serde_duper::{bytes::ByteBuf, duper};
use std::collections::HashMap;

#[test]
fn both_derives() {
    duper! {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct MyStruct {
            #[duper(CustomMap)]
            map: HashMap<String, (u32, ByteBuf)>,
        }
    }

    let mut map = HashMap::new();
    map.insert("a".to_string(), (1, b"abc".to_vec().into()));
    let value = MyStruct { map };
    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"MyStruct({map: CustomMap({a: (1, b"abc")})})"#
    );

    let deserialized: MyStruct = serde_duper::from_string(&serialized).unwrap();
    assert_eq!(deserialized.map.len(), 1);
    assert_eq!(deserialized.map["a"].0, 1);
    assert_eq!(deserialized.map["a"].1, b"abc");
}

#[test]
fn serialize_only() {
    duper! {
        #[derive(Debug, Clone, Serialize)]
        struct MyStruct {
            #[duper(CustomMap)]
            map: HashMap<String, (u32, ByteBuf)>,
        }
    }

    let mut map = HashMap::new();
    map.insert("a".to_string(), (1, b"abc".to_vec().into()));
    let value = MyStruct { map };
    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"MyStruct({map: CustomMap({a: (1, b"abc")})})"#
    );
}

#[test]
fn deserialize_only() {
    duper! {
        #[derive(Debug, Clone, Deserialize)]
        struct MyStruct {
            #[duper(CustomMap)]
            map: HashMap<String, (u32, ByteBuf)>,
        }
    }

    let deserialized: MyStruct =
        serde_duper::from_string(r#"MyStruct({map: {a: (1, b"abc")}})"#).unwrap();
    assert_eq!(deserialized.map.len(), 1);
    assert_eq!(deserialized.map["a"].0, 1);
    assert_eq!(deserialized.map["a"].1, b"abc");
}
