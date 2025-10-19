use serde::{Deserialize, Serialize};

#[test]
fn net() {
    use serde_duper::types::net::{
        DuperIpAddr, DuperIpv4Addr, DuperIpv6Addr, DuperSocketAddr, DuperSocketAddrV4,
        DuperSocketAddrV6,
    };
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "DuperIpAddr")]
        ip_addr: IpAddr,
        #[serde(with = "DuperIpv4Addr")]
        ipv4_addr: Ipv4Addr,
        #[serde(with = "DuperIpv6Addr")]
        ipv6_addr: Ipv6Addr,
        #[serde(with = "DuperSocketAddr")]
        socket_addr: SocketAddr,
        #[serde(with = "DuperSocketAddrV4")]
        socket_addr_v4: SocketAddrV4,
        #[serde(with = "DuperSocketAddrV6")]
        socket_addr_v6: SocketAddrV6,
    }

    let value = Test {
        ip_addr: "192.168.1.1".parse().unwrap(),
        ipv4_addr: "10.0.0.1".parse().unwrap(),
        ipv6_addr: "::1".parse().unwrap(),
        socket_addr: "127.0.0.1:8080".parse().unwrap(),
        socket_addr_v4: "192.168.1.1:443".parse().unwrap(),
        socket_addr_v6: "[2001:db8::1]:80".parse().unwrap(),
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"Test({ip_addr: IpAddr("192.168.1.1"), ipv4_addr: Ipv4Addr("10.0.0.1"), ipv6_addr: Ipv6Addr("::1"), socket_addr: SocketAddr("127.0.0.1:8080"), socket_addr_v4: SocketAddrV4("192.168.1.1:443"), socket_addr_v6: SocketAddrV6("[2001:db8::1]:80")})"#
    );

    let deserialized: Test = serde_duper::from_string(&serialized).unwrap();
    assert_eq!(value.ip_addr, deserialized.ip_addr);
    assert_eq!(value.ipv4_addr, deserialized.ipv4_addr);
    assert_eq!(value.ipv6_addr, deserialized.ipv6_addr);
    assert_eq!(value.socket_addr, deserialized.socket_addr);
    assert_eq!(value.socket_addr_v4, deserialized.socket_addr_v4);
    assert_eq!(value.socket_addr_v6, deserialized.socket_addr_v6);
}

#[test]
fn num() {
    use serde_duper::types::num::{
        DuperNonZeroI8, DuperNonZeroI16, DuperNonZeroI32, DuperNonZeroI64, DuperNonZeroI128,
        DuperNonZeroIsize, DuperNonZeroU8, DuperNonZeroU16, DuperNonZeroU32, DuperNonZeroU64,
        DuperNonZeroU128, DuperNonZeroUsize, DuperSaturating, DuperWrapping,
    };
    use std::num::{
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize, NonZeroU8,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize, Saturating, Wrapping,
    };

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "DuperNonZeroI8")]
        nz_i8: NonZeroI8,
        #[serde(with = "DuperNonZeroI16")]
        nz_i16: NonZeroI16,
        #[serde(with = "DuperNonZeroI32")]
        nz_i32: NonZeroI32,
        #[serde(with = "DuperNonZeroI64")]
        nz_i64: NonZeroI64,
        #[serde(with = "DuperNonZeroI128")]
        nz_i128: NonZeroI128,
        #[serde(with = "DuperNonZeroIsize")]
        nz_isize: NonZeroIsize,
        #[serde(with = "DuperNonZeroU8")]
        nz_u8: NonZeroU8,
        #[serde(with = "DuperNonZeroU16")]
        nz_u16: NonZeroU16,
        #[serde(with = "DuperNonZeroU32")]
        nz_u32: NonZeroU32,
        #[serde(with = "DuperNonZeroU64")]
        nz_u64: NonZeroU64,
        #[serde(with = "DuperNonZeroU128")]
        nz_u128: NonZeroU128,
        #[serde(with = "DuperNonZeroUsize")]
        nz_usize: NonZeroUsize,
        #[serde(with = "DuperWrapping")]
        wrapping: Wrapping<i32>,
        #[serde(with = "DuperSaturating")]
        saturating: Saturating<u32>,
    }

    let value = Test {
        nz_i8: NonZeroI8::new(42).unwrap(),
        nz_i16: NonZeroI16::new(1234).unwrap(),
        nz_i32: NonZeroI32::new(123456).unwrap(),
        nz_i64: NonZeroI64::new(123456789).unwrap(),
        nz_i128: NonZeroI128::new(123456789012).unwrap(),
        nz_isize: NonZeroIsize::new(999).unwrap(),
        nz_u8: NonZeroU8::new(42).unwrap(),
        nz_u16: NonZeroU16::new(1234).unwrap(),
        nz_u32: NonZeroU32::new(123456).unwrap(),
        nz_u64: NonZeroU64::new(123456789).unwrap(),
        nz_u128: NonZeroU128::new(123456789012).unwrap(),
        nz_usize: NonZeroUsize::new(999).unwrap(),
        wrapping: Wrapping(100),
        saturating: Saturating(200),
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"Test({nz_i8: NonZeroI8(42), nz_i16: NonZeroI16(1234), nz_i32: NonZeroI32(123456), nz_i64: NonZeroI64(123456789), nz_i128: NonZeroI128(123456789012), nz_isize: NonZeroIsize(999), nz_u8: NonZeroU8(42), nz_u16: NonZeroU16(1234), nz_u32: NonZeroU32(123456), nz_u64: NonZeroU64(123456789), nz_u128: NonZeroU128(123456789012), nz_usize: NonZeroUsize(999), wrapping: Wrapping(100), saturating: Saturating(200)})"#
    );

    let deserialized: Test = serde_duper::from_string(&serialized).unwrap();
    assert_eq!(value.nz_i8, deserialized.nz_i8);
    assert_eq!(value.nz_i16, deserialized.nz_i16);
    assert_eq!(value.nz_i32, deserialized.nz_i32);
    assert_eq!(value.nz_i64, deserialized.nz_i64);
    assert_eq!(value.nz_i128, deserialized.nz_i128);
    assert_eq!(value.nz_isize, deserialized.nz_isize);
    assert_eq!(value.nz_u8, deserialized.nz_u8);
    assert_eq!(value.nz_u16, deserialized.nz_u16);
    assert_eq!(value.nz_u32, deserialized.nz_u32);
    assert_eq!(value.nz_u64, deserialized.nz_u64);
    assert_eq!(value.nz_u128, deserialized.nz_u128);
    assert_eq!(value.nz_usize, deserialized.nz_usize);
    assert_eq!(value.wrapping, deserialized.wrapping);
    assert_eq!(value.saturating, deserialized.saturating);
}

#[test]
fn atomic() {
    use serde_duper::types::atomic::{
        DuperAtomicBool, DuperAtomicI8, DuperAtomicI16, DuperAtomicI32, DuperAtomicI64,
        DuperAtomicIsize, DuperAtomicU8, DuperAtomicU16, DuperAtomicU32, DuperAtomicU64,
        DuperAtomicUsize,
    };
    use std::sync::atomic::{
        AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16,
        AtomicU32, AtomicU64, AtomicUsize,
    };

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "DuperAtomicBool")]
        atomic_bool: AtomicBool,
        #[serde(with = "DuperAtomicI8")]
        atomic_i8: AtomicI8,
        #[serde(with = "DuperAtomicI16")]
        atomic_i16: AtomicI16,
        #[serde(with = "DuperAtomicI32")]
        atomic_i32: AtomicI32,
        #[serde(with = "DuperAtomicI64")]
        atomic_i64: AtomicI64,
        #[serde(with = "DuperAtomicIsize")]
        atomic_isize: AtomicIsize,
        #[serde(with = "DuperAtomicU8")]
        atomic_u8: AtomicU8,
        #[serde(with = "DuperAtomicU16")]
        atomic_u16: AtomicU16,
        #[serde(with = "DuperAtomicU32")]
        atomic_u32: AtomicU32,
        #[serde(with = "DuperAtomicU64")]
        atomic_u64: AtomicU64,
        #[serde(with = "DuperAtomicUsize")]
        atomic_usize: AtomicUsize,
    }

    let value = Test {
        atomic_bool: AtomicBool::new(true),
        atomic_i8: AtomicI8::new(-42),
        atomic_i16: AtomicI16::new(-1234),
        atomic_i32: AtomicI32::new(-123456),
        atomic_i64: AtomicI64::new(-123456789),
        atomic_isize: AtomicIsize::new(-999),
        atomic_u8: AtomicU8::new(42),
        atomic_u16: AtomicU16::new(1234),
        atomic_u32: AtomicU32::new(123456),
        atomic_u64: AtomicU64::new(123456789),
        atomic_usize: AtomicUsize::new(999),
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"Test({atomic_bool: AtomicBool(true), atomic_i8: AtomicI8(-42), atomic_i16: AtomicI16(-1234), atomic_i32: AtomicI32(-123456), atomic_i64: AtomicI64(-123456789), atomic_isize: AtomicIsize(-999), atomic_u8: AtomicU8(42), atomic_u16: AtomicU16(1234), atomic_u32: AtomicU32(123456), atomic_u64: AtomicU64(123456789), atomic_usize: AtomicUsize(999)})"#
    );

    let deserialized: Test = serde_duper::from_string(&serialized).unwrap();
    assert_eq!(
        value.atomic_bool.load(std::sync::atomic::Ordering::SeqCst),
        deserialized
            .atomic_bool
            .load(std::sync::atomic::Ordering::SeqCst)
    );
    assert_eq!(
        value.atomic_i8.load(std::sync::atomic::Ordering::SeqCst),
        deserialized
            .atomic_i8
            .load(std::sync::atomic::Ordering::SeqCst)
    );
    assert_eq!(
        value.atomic_i16.load(std::sync::atomic::Ordering::SeqCst),
        deserialized
            .atomic_i16
            .load(std::sync::atomic::Ordering::SeqCst)
    );
    assert_eq!(
        value.atomic_i32.load(std::sync::atomic::Ordering::SeqCst),
        deserialized
            .atomic_i32
            .load(std::sync::atomic::Ordering::SeqCst)
    );
    assert_eq!(
        value.atomic_i64.load(std::sync::atomic::Ordering::SeqCst),
        deserialized
            .atomic_i64
            .load(std::sync::atomic::Ordering::SeqCst)
    );
    assert_eq!(
        value.atomic_isize.load(std::sync::atomic::Ordering::SeqCst),
        deserialized
            .atomic_isize
            .load(std::sync::atomic::Ordering::SeqCst)
    );
    assert_eq!(
        value.atomic_u8.load(std::sync::atomic::Ordering::SeqCst),
        deserialized
            .atomic_u8
            .load(std::sync::atomic::Ordering::SeqCst)
    );
    assert_eq!(
        value.atomic_u16.load(std::sync::atomic::Ordering::SeqCst),
        deserialized
            .atomic_u16
            .load(std::sync::atomic::Ordering::SeqCst)
    );
    assert_eq!(
        value.atomic_u32.load(std::sync::atomic::Ordering::SeqCst),
        deserialized
            .atomic_u32
            .load(std::sync::atomic::Ordering::SeqCst)
    );
    assert_eq!(
        value.atomic_u64.load(std::sync::atomic::Ordering::SeqCst),
        deserialized
            .atomic_u64
            .load(std::sync::atomic::Ordering::SeqCst)
    );
    assert_eq!(
        value.atomic_usize.load(std::sync::atomic::Ordering::SeqCst),
        deserialized
            .atomic_usize
            .load(std::sync::atomic::Ordering::SeqCst)
    );
}

#[test]
fn time() {
    use serde_duper::types::{DuperDuration, DuperSystemTime};
    use std::time::{Duration, SystemTime};

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "DuperDuration")]
        duration: Duration,
        #[serde(with = "DuperSystemTime")]
        system_time: SystemTime,
    }

    let value = Test {
        duration: Duration::from_secs(3600),
        system_time: SystemTime::now(),
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert!(
        serialized.starts_with(
        r#"Test({duration: Duration({secs: 3600, nanos: 0}), system_time: SystemTime({secs_since_epoch: "#)
    );

    let deserialized: Test = serde_duper::from_string(&serialized).unwrap();
    assert_eq!(value.duration, deserialized.duration);
    assert!(
        value
            .system_time
            .duration_since(deserialized.system_time)
            .unwrap()
            .as_secs()
            < 1
    );
}

#[test]
fn path() {
    use serde_duper::types::{DuperPath, DuperPathBuf};
    use std::path::{Path, PathBuf};

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "DuperPathBuf")]
        path_buf: PathBuf,
    }

    let value = Test {
        path_buf: PathBuf::from("/home/user/file.txt"),
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"Test({path_buf: PathBuf("/home/user/file.txt")})"#
    );

    let deserialized: Test = serde_duper::from_string(&serialized).unwrap();
    assert_eq!(value.path_buf, deserialized.path_buf);

    #[derive(Debug, Serialize)]
    struct TestSerializeOnly<'a> {
        #[serde(borrow, with = "DuperPath")]
        path: &'a Path,
    }

    let value = TestSerializeOnly {
        path: Path::new("/tmp/test"),
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"TestSerializeOnly({path: Path("/tmp/test")})"#
    );
}

#[test]
#[cfg(unix)]
fn ffi_unix() {
    use serde_duper::types::ffi::{DuperCStr, DuperCString, DuperOsStr, DuperOsString};
    use std::ffi::{CStr, CString, OsStr, OsString};

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "DuperCString")]
        c_string: CString,
        #[serde(with = "DuperOsString")]
        os_string: OsString,
    }

    let value = Test {
        c_string: CString::new("hello").unwrap(),
        os_string: OsString::from("test_os_string"),
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"Test({c_string: CString(b"hello"), os_string: OsString({Unix: b"test_os_string"})})"#
    );

    let deserialized: Test = serde_duper::from_string(&serialized).unwrap();
    assert_eq!(value.c_string, deserialized.c_string);
    assert_eq!(value.os_string, deserialized.os_string);

    #[derive(Debug, Serialize)]
    struct TestSerializeOnly<'a> {
        #[serde(with = "DuperCStr")]
        c_str: &'a CStr,
        #[serde(with = "DuperOsStr")]
        os_str: &'a OsStr,
    }

    let value = TestSerializeOnly {
        c_str: &CString::new("goodbye").unwrap(),
        os_str: &OsString::from("test_os_str"),
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"TestSerializeOnly({c_str: CStr(b"goodbye"), os_str: OsStr({Unix: b"test_os_str"})})"#
    );
}

#[test]
#[cfg(windows)]
fn ffi_windows() {
    use serde_duper::types::ffi::{DuperCStr, DuperCString, DuperOsStr, DuperOsString};
    use std::ffi::{CStr, CString, OsStr, OsString};

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "DuperCString")]
        c_string: CString,
        #[serde(with = "DuperOsString")]
        os_string: OsString,
    }

    let value = Test {
        c_string: CString::new("hello").unwrap(),
        os_string: OsString::from("test_os_string"),
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"Test({c_string: CString(b"hello"), os_string: OsString({Windows: [0]})})"#
    );

    let deserialized: Test = serde_duper::from_string(&serialized).unwrap();
    assert_eq!(value.c_string, deserialized.c_string);
    assert_eq!(value.os_string, deserialized.os_string);

    #[derive(Debug, Serialize)]
    struct TestSerializeOnly<'a> {
        #[serde(with = "DuperCStr")]
        c_str: &'a CStr,
        #[serde(with = "DuperOsStr")]
        os_str: &'a OsStr,
    }

    let value = TestSerializeOnly {
        c_str: &CString::new("goodbye").unwrap(),
        os_str: &OsString::from("test_os_str"),
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"TestSerializeOnly({c_str: CStr(b"goodbye"), os_str: OsStr({Windows: [0]})})"#
    );
}

#[test]
fn collections() {
    use serde_duper::types::DuperReverse;
    use serde_duper::types::collections::{
        DuperBTreeMap, DuperBTreeSet, DuperBinaryHeap, DuperHashMap, DuperHashSet, DuperLinkedList,
        DuperVecDeque,
    };
    use std::collections::{
        BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque,
    };

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "DuperBinaryHeap")]
        binary_heap: BinaryHeap<i32>,
        #[serde(with = "DuperBTreeSet")]
        btree_set: BTreeSet<String>,
        #[serde(with = "DuperBTreeMap")]
        btree_map: BTreeMap<String, i32>,
        #[serde(with = "DuperLinkedList")]
        linked_list: LinkedList<char>,
        #[serde(with = "DuperVecDeque")]
        vec_deque: VecDeque<bool>,
        #[serde(with = "DuperReverse")]
        reverse: std::cmp::Reverse<usize>,
        #[serde(with = "DuperHashMap")]
        hash_map: HashMap<String, u32>,
        #[serde(with = "DuperHashSet")]
        hash_set: HashSet<u32>,
    }

    let value = Test {
        binary_heap: vec![3, 1, 4, 1, 5].into_iter().collect(),
        btree_set: vec![
            "apple".to_string(),
            "banana".to_string(),
            "cherry".to_string(),
        ]
        .into_iter()
        .collect(),
        hash_set: vec![1, 2, 3, 4, 5].into_iter().collect(),
        linked_list: vec!['a', 'b', 'c'].into_iter().collect(),
        vec_deque: vec![true, false, true].into_iter().collect(),
        btree_map: vec![
            ("one".to_string(), 1),
            ("two".to_string(), 2),
            ("three".to_string(), 3),
        ]
        .into_iter()
        .collect(),
        hash_map: vec![
            ("uno".to_string(), 1),
            ("dos".to_string(), 2),
            ("tres".to_string(), 3),
        ]
        .into_iter()
        .collect(),
        reverse: std::cmp::Reverse(42),
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert!(
        serialized.starts_with(r#"Test({binary_heap: BinaryHeap([5, 3, 4, 1, 1]), btree_set: BTreeSet(["apple", "banana", "cherry"]), btree_map: BTreeMap({one: 1, three: 3, two: 2}), linked_list: LinkedList([Char("a"), Char("b"), Char("c")]), vec_deque: VecDeque([true, false, true]), reverse: Reverse(42), hash_map: HashMap({"#
    ));

    let deserialized: Test = serde_duper::from_string(&serialized).unwrap();
    assert_eq!(
        value.binary_heap.into_sorted_vec(),
        deserialized.binary_heap.into_sorted_vec()
    );
    assert_eq!(value.btree_set, deserialized.btree_set);
    assert_eq!(value.hash_set, deserialized.hash_set);
    assert_eq!(value.linked_list, deserialized.linked_list);
    assert_eq!(value.vec_deque, deserialized.vec_deque);
    assert_eq!(value.btree_map, deserialized.btree_map);
    assert_eq!(value.hash_map, deserialized.hash_map);
    assert_eq!(value.reverse, deserialized.reverse);
}

#[test]
#[cfg(feature = "bytes")]
fn bytes() {
    use bytes::Bytes;
    use serde_duper::types::DuperBytes;

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "DuperBytes")]
        data: Bytes,
    }

    let value = Test {
        data: Bytes::copy_from_slice(b"hello"),
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(serialized, r#"Test({data: Bytes(b"hello")})"#);

    let deserialized: Test = serde_duper::from_string(&serialized).unwrap();
    assert_eq!(value.data, deserialized.data);
}

#[test]
#[cfg(feature = "chrono")]
fn chrono() {
    use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta, Utc};
    use serde_duper::types::chrono::{
        DuperDateTime, DuperNaiveDate, DuperNaiveDateTime, DuperNaiveTime, DuperTimeDelta,
    };

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "DuperDateTime")]
        utc: DateTime<Utc>,
        #[serde(with = "DuperDateTime")]
        fo: DateTime<FixedOffset>,
        #[serde(with = "DuperNaiveDateTime")]
        ndt: NaiveDateTime,
        #[serde(with = "DuperNaiveDate")]
        nd: NaiveDate,
        #[serde(with = "DuperNaiveTime")]
        nt: NaiveTime,
        #[serde(with = "DuperTimeDelta")]
        td: TimeDelta,
    }

    let value = Test {
        utc: "2023-10-05T14:30:00Z".parse().unwrap(),
        fo: "2023-10-05T11:30:00-03:00".parse().unwrap(),
        ndt: "2023-10-05T14:30:00".parse().unwrap(),
        nd: "2023-10-05".parse().unwrap(),
        nt: "14:30:00".parse().unwrap(),
        td: TimeDelta::days(7),
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"Test({utc: DateTime("2023-10-05T14:30:00Z"), fo: DateTime("2023-10-05T11:30:00-03:00"), ndt: NaiveDateTime("2023-10-05T14:30:00"), nd: NaiveDate("2023-10-05"), nt: NaiveTime("14:30:00"), td: TimeDelta((604800, 0))})"#
    );

    let deserialized: Test = serde_duper::from_string(&serialized).unwrap();
    assert_eq!(value.utc, deserialized.utc);
    assert_eq!(value.fo, deserialized.fo);
    assert_eq!(value.ndt, deserialized.ndt);
    assert_eq!(value.nd, deserialized.nd);
    assert_eq!(value.nt, deserialized.nt);
    assert_eq!(value.td, deserialized.td);
}

#[test]
#[cfg(feature = "decimal")]
fn decimal() {
    use rust_decimal::{Decimal, dec};
    use serde_duper::types::DuperDecimal;

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "DuperDecimal")]
        cost: Decimal,
    }

    let value = Test {
        cost: dec!(12345678.90),
    };
    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(serialized, r#"Test({cost: Decimal("12345678.90")})"#);

    let deserialized: Test = serde_duper::from_string(&serialized).unwrap();
    assert_eq!(value.cost, deserialized.cost);
}

#[test]
#[cfg(feature = "ipnet")]
fn ipnet() {
    use ipnet::{IpNet, Ipv4Net, Ipv6Net};
    use serde_duper::types::{DuperIpNet, DuperIpv4Net, DuperIpv6Net};

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "DuperIpNet")]
        generic: IpNet,
        #[serde(with = "DuperIpv4Net")]
        v4: Ipv4Net,
        #[serde(with = "DuperIpv6Net")]
        v6: Ipv6Net,
    }

    let value = Test {
        generic: IpNet::V4("192.168.0.0/24".parse().unwrap()),
        v4: "10.15.0.0/16".parse().unwrap(),
        v6: "2001:db8::/32".parse().unwrap(),
    };
    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"Test({generic: IpNet("192.168.0.0/24"), v4: Ipv4Net("10.15.0.0/16"), v6: Ipv6Net("2001:db8::/32")})"#
    );

    let deserialized: Test = serde_duper::from_string(&serialized).unwrap();
    assert_eq!(value.generic, deserialized.generic);
    assert_eq!(value.v4, deserialized.v4);
    assert_eq!(value.v6, deserialized.v6);
}

#[test]
#[cfg(feature = "regex")]
fn regex() {
    use regex::Regex;
    use serde_duper::types::DuperRegex;

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "DuperRegex")]
        pattern: Regex,
    }

    let value = Test {
        pattern: Regex::new(r"Hello (?<name>\w+)!").unwrap(),
    };
    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"Test({pattern: Regex(r"Hello (?<name>\w+)!")})"#
    );

    let deserialized: Test = serde_duper::from_string(&serialized).unwrap();
    assert_eq!(value.pattern.as_str(), deserialized.pattern.as_str());
}

#[test]
#[cfg(feature = "uuid")]
fn uuid() {
    use serde_duper::types::DuperUuid;
    use uuid::Uuid;

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "DuperUuid")]
        id: Uuid,
    }

    let value = Test {
        id: Uuid::parse_str("f5ea955b-85bf-4643-be05-0675b2c2b61e").unwrap(),
    };
    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"Test({id: Uuid("f5ea955b-85bf-4643-be05-0675b2c2b61e")})"#
    );

    let deserialized: Test = serde_duper::from_string(&serialized).unwrap();
    assert_eq!(value.id, deserialized.id);
}
