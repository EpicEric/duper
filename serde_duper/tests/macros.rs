#![cfg(feature = "macros")]
#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};
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
            #[duper(Bytes)]
            bytes: ByteBuf,
            plain: (u64,),
        }
    }

    let mut map = HashMap::new();
    map.insert("a".to_string(), (1, b"abc".to_vec().into()));
    let value = MyStruct {
        map,
        bytes: b"duper".to_vec().into(),
        plain: (1234,),
    };
    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"MyStruct({map: CustomMap({a: (1, b"abc")}), bytes: Bytes(b"duper"), plain: (1234)})"#
    );

    let deserialized: MyStruct = serde_duper::from_string(&serialized).unwrap();
    assert_eq!(deserialized.map.len(), 1);
    assert_eq!(deserialized.map["a"].0, 1);
    assert_eq!(deserialized.map["a"].1, b"abc");
    assert_eq!(deserialized.bytes, b"duper");
    assert_eq!(deserialized.plain, (1234,));
}

#[test]
fn serialize_only() {
    duper! {
        #[derive(Debug, Clone, Serialize)]
        struct MyStruct {
            #[duper(CustomMap)]
            map: HashMap<String, (u32, ByteBuf)>,
            #[duper(Bytes)]
            bytes: ByteBuf,
            plain: (u64,),
        }
    }

    let mut map = HashMap::new();
    map.insert("a".to_string(), (1, b"abc".to_vec().into()));
    let value = MyStruct {
        map,
        bytes: b"duper".to_vec().into(),
        plain: (1234,),
    };
    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"MyStruct({map: CustomMap({a: (1, b"abc")}), bytes: Bytes(b"duper"), plain: (1234)})"#
    );
}

#[test]
fn deserialize_only() {
    duper! {
        #[derive(Debug, Clone, Deserialize)]
        struct MyStruct {
            #[duper(CustomMap)]
            map: HashMap<String, (u32, ByteBuf)>,
            #[duper(Bytes)]
            bytes: ByteBuf,
            plain: (u64,),
        }
    }

    let deserialized: MyStruct = serde_duper::from_string(
        r#"AnotherName({map: {a: (1, b"abc")}, bytes: b"duper", plain: Foo((Bar(1234),))})"#,
    )
    .unwrap();
    assert_eq!(deserialized.map.len(), 1);
    assert_eq!(deserialized.map["a"].0, 1);
    assert_eq!(deserialized.map["a"].1, b"abc");
    assert_eq!(deserialized.bytes, b"duper");
    assert_eq!(deserialized.plain, (1234,));
}

#[test]
fn duper_macro_for_tuple_struct() {
    duper! {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
        struct MyTuple(i32, #[duper(BookName)] String, ());
    }

    duper! {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct MyStruct {
            tup: MyTuple,
        }
    }

    let value = MyStruct {
        tup: MyTuple(1948, "1984".into(), ()),
    };
    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"MyStruct({tup: MyTuple((1948, BookName("1984"), ()))})"#
    );

    let deserialized: MyStruct = serde_duper::from_string(&serialized).unwrap();
    assert_eq!(deserialized.tup, value.tup);
}

#[test]
#[cfg(feature = "chrono")]
fn duper_macro_for_external_type() {
    duper! {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct MyStruct {
            #[duper(DateTime)]
            datetime: DateTime<Utc>,
        }
    }

    let value: MyStruct = MyStruct {
        datetime: "2023-10-05T14:30:00Z".parse().unwrap(),
    };
    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"MyStruct({datetime: DateTime("2023-10-05T14:30:00Z")})"#
    );

    let deserialized: MyStruct = serde_duper::from_string(&serialized).unwrap();
    assert_eq!(deserialized.datetime, value.datetime);
}
