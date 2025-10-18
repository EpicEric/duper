use serde::{Deserialize, Serialize};

#[test]
fn some_net() {
    use serde_duper::types::net::{
        DuperOptionIpAddr, DuperOptionIpv4Addr, DuperOptionIpv6Addr, DuperOptionSocketAddr,
        DuperOptionSocketAddrV4, DuperOptionSocketAddrV6,
    };
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "DuperOptionIpAddr")]
        ip_addr: Option<IpAddr>,
        #[serde(with = "DuperOptionIpv4Addr")]
        ipv4_addr: Option<Ipv4Addr>,
        #[serde(with = "DuperOptionIpv6Addr")]
        ipv6_addr: Option<Ipv6Addr>,
        #[serde(with = "DuperOptionSocketAddr")]
        socket_addr: Option<SocketAddr>,
        #[serde(with = "DuperOptionSocketAddrV4")]
        socket_addr_v4: Option<SocketAddrV4>,
        #[serde(with = "DuperOptionSocketAddrV6")]
        socket_addr_v6: Option<SocketAddrV6>,
    }

    let value = Test {
        ip_addr: Some("192.168.1.1".parse().unwrap()),
        ipv4_addr: Some("10.0.0.1".parse().unwrap()),
        ipv6_addr: Some("::1".parse().unwrap()),
        socket_addr: Some("127.0.0.1:8080".parse().unwrap()),
        socket_addr_v4: Some("192.168.1.1:443".parse().unwrap()),
        socket_addr_v6: Some("[2001:db8::1]:80".parse().unwrap()),
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
fn some_num() {
    use serde_duper::types::num::{
        DuperOptionNonZeroI8, DuperOptionNonZeroI16, DuperOptionNonZeroI32, DuperOptionNonZeroI64,
        DuperOptionNonZeroI128, DuperOptionNonZeroIsize, DuperOptionNonZeroU8,
        DuperOptionNonZeroU16, DuperOptionNonZeroU32, DuperOptionNonZeroU64,
        DuperOptionNonZeroU128, DuperOptionNonZeroUsize, DuperOptionSaturating,
        DuperOptionWrapping,
    };
    use std::num::{
        NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize, NonZeroU8,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize, Saturating, Wrapping,
    };

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "DuperOptionNonZeroI8")]
        nz_i8: Option<NonZeroI8>,
        #[serde(with = "DuperOptionNonZeroI16")]
        nz_i16: Option<NonZeroI16>,
        #[serde(with = "DuperOptionNonZeroI32")]
        nz_i32: Option<NonZeroI32>,
        #[serde(with = "DuperOptionNonZeroI64")]
        nz_i64: Option<NonZeroI64>,
        #[serde(with = "DuperOptionNonZeroI128")]
        nz_i128: Option<NonZeroI128>,
        #[serde(with = "DuperOptionNonZeroIsize")]
        nz_isize: Option<NonZeroIsize>,
        #[serde(with = "DuperOptionNonZeroU8")]
        nz_u8: Option<NonZeroU8>,
        #[serde(with = "DuperOptionNonZeroU16")]
        nz_u16: Option<NonZeroU16>,
        #[serde(with = "DuperOptionNonZeroU32")]
        nz_u32: Option<NonZeroU32>,
        #[serde(with = "DuperOptionNonZeroU64")]
        nz_u64: Option<NonZeroU64>,
        #[serde(with = "DuperOptionNonZeroU128")]
        nz_u128: Option<NonZeroU128>,
        #[serde(with = "DuperOptionNonZeroUsize")]
        nz_usize: Option<NonZeroUsize>,
        #[serde(with = "DuperOptionWrapping")]
        wrapping: Option<Wrapping<i32>>,
        #[serde(with = "DuperOptionSaturating")]
        saturating: Option<Saturating<u32>>,
    }

    let value = Test {
        nz_i8: Some(NonZeroI8::new(42).unwrap()),
        nz_i16: Some(NonZeroI16::new(1234).unwrap()),
        nz_i32: Some(NonZeroI32::new(123456).unwrap()),
        nz_i64: Some(NonZeroI64::new(123456789).unwrap()),
        nz_i128: Some(NonZeroI128::new(123456789012).unwrap()),
        nz_isize: Some(NonZeroIsize::new(999).unwrap()),
        nz_u8: Some(NonZeroU8::new(42).unwrap()),
        nz_u16: Some(NonZeroU16::new(1234).unwrap()),
        nz_u32: Some(NonZeroU32::new(123456).unwrap()),
        nz_u64: Some(NonZeroU64::new(123456789).unwrap()),
        nz_u128: Some(NonZeroU128::new(123456789012).unwrap()),
        nz_usize: Some(NonZeroUsize::new(999).unwrap()),
        wrapping: Some(Wrapping(100)),
        saturating: Some(Saturating(200)),
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
fn some_atomic() {
    use serde_duper::types::atomic::{
        DuperOptionAtomicBool, DuperOptionAtomicI8, DuperOptionAtomicI16, DuperOptionAtomicI32,
        DuperOptionAtomicI64, DuperOptionAtomicIsize, DuperOptionAtomicU8, DuperOptionAtomicU16,
        DuperOptionAtomicU32, DuperOptionAtomicU64, DuperOptionAtomicUsize,
    };
    use std::sync::atomic::{
        AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16,
        AtomicU32, AtomicU64, AtomicUsize,
    };

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "DuperOptionAtomicBool")]
        atomic_bool: Option<AtomicBool>,
        #[serde(with = "DuperOptionAtomicI8")]
        atomic_i8: Option<AtomicI8>,
        #[serde(with = "DuperOptionAtomicI16")]
        atomic_i16: Option<AtomicI16>,
        #[serde(with = "DuperOptionAtomicI32")]
        atomic_i32: Option<AtomicI32>,
        #[serde(with = "DuperOptionAtomicI64")]
        atomic_i64: Option<AtomicI64>,
        #[serde(with = "DuperOptionAtomicIsize")]
        atomic_isize: Option<AtomicIsize>,
        #[serde(with = "DuperOptionAtomicU8")]
        atomic_u8: Option<AtomicU8>,
        #[serde(with = "DuperOptionAtomicU16")]
        atomic_u16: Option<AtomicU16>,
        #[serde(with = "DuperOptionAtomicU32")]
        atomic_u32: Option<AtomicU32>,
        #[serde(with = "DuperOptionAtomicU64")]
        atomic_u64: Option<AtomicU64>,
        #[serde(with = "DuperOptionAtomicUsize")]
        atomic_usize: Option<AtomicUsize>,
    }

    let value = Test {
        atomic_bool: Some(AtomicBool::new(true)),
        atomic_i8: Some(AtomicI8::new(-42)),
        atomic_i16: Some(AtomicI16::new(-1234)),
        atomic_i32: Some(AtomicI32::new(-123456)),
        atomic_i64: Some(AtomicI64::new(-123456789)),
        atomic_isize: Some(AtomicIsize::new(-999)),
        atomic_u8: Some(AtomicU8::new(42)),
        atomic_u16: Some(AtomicU16::new(1234)),
        atomic_u32: Some(AtomicU32::new(123456)),
        atomic_u64: Some(AtomicU64::new(123456789)),
        atomic_usize: Some(AtomicUsize::new(999)),
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"Test({atomic_bool: AtomicBool(true), atomic_i8: AtomicI8(-42), atomic_i16: AtomicI16(-1234), atomic_i32: AtomicI32(-123456), atomic_i64: AtomicI64(-123456789), atomic_isize: AtomicIsize(-999), atomic_u8: AtomicU8(42), atomic_u16: AtomicU16(1234), atomic_u32: AtomicU32(123456), atomic_u64: AtomicU64(123456789), atomic_usize: AtomicUsize(999)})"#
    );

    let deserialized: Test = serde_duper::from_string(&serialized).unwrap();
    assert_eq!(
        value
            .atomic_bool
            .as_ref()
            .unwrap()
            .load(std::sync::atomic::Ordering::SeqCst),
        deserialized
            .atomic_bool
            .as_ref()
            .unwrap()
            .load(std::sync::atomic::Ordering::SeqCst)
    );
    assert_eq!(
        value
            .atomic_i8
            .as_ref()
            .unwrap()
            .load(std::sync::atomic::Ordering::SeqCst),
        deserialized
            .atomic_i8
            .as_ref()
            .unwrap()
            .load(std::sync::atomic::Ordering::SeqCst)
    );
    assert_eq!(
        value
            .atomic_i16
            .as_ref()
            .unwrap()
            .load(std::sync::atomic::Ordering::SeqCst),
        deserialized
            .atomic_i16
            .as_ref()
            .unwrap()
            .load(std::sync::atomic::Ordering::SeqCst)
    );
    assert_eq!(
        value
            .atomic_i32
            .as_ref()
            .unwrap()
            .load(std::sync::atomic::Ordering::SeqCst),
        deserialized
            .atomic_i32
            .as_ref()
            .unwrap()
            .load(std::sync::atomic::Ordering::SeqCst)
    );
    assert_eq!(
        value
            .atomic_i64
            .as_ref()
            .unwrap()
            .load(std::sync::atomic::Ordering::SeqCst),
        deserialized
            .atomic_i64
            .as_ref()
            .unwrap()
            .load(std::sync::atomic::Ordering::SeqCst)
    );
    assert_eq!(
        value
            .atomic_isize
            .as_ref()
            .unwrap()
            .load(std::sync::atomic::Ordering::SeqCst),
        deserialized
            .atomic_isize
            .as_ref()
            .unwrap()
            .load(std::sync::atomic::Ordering::SeqCst)
    );
    assert_eq!(
        value
            .atomic_u8
            .as_ref()
            .unwrap()
            .load(std::sync::atomic::Ordering::SeqCst),
        deserialized
            .atomic_u8
            .as_ref()
            .unwrap()
            .load(std::sync::atomic::Ordering::SeqCst)
    );
    assert_eq!(
        value
            .atomic_u16
            .as_ref()
            .unwrap()
            .load(std::sync::atomic::Ordering::SeqCst),
        deserialized
            .atomic_u16
            .as_ref()
            .unwrap()
            .load(std::sync::atomic::Ordering::SeqCst)
    );
    assert_eq!(
        value
            .atomic_u32
            .as_ref()
            .unwrap()
            .load(std::sync::atomic::Ordering::SeqCst),
        deserialized
            .atomic_u32
            .as_ref()
            .unwrap()
            .load(std::sync::atomic::Ordering::SeqCst)
    );
    assert_eq!(
        value
            .atomic_u64
            .as_ref()
            .unwrap()
            .load(std::sync::atomic::Ordering::SeqCst),
        deserialized
            .atomic_u64
            .as_ref()
            .unwrap()
            .load(std::sync::atomic::Ordering::SeqCst)
    );
    assert_eq!(
        value
            .atomic_usize
            .as_ref()
            .unwrap()
            .load(std::sync::atomic::Ordering::SeqCst),
        deserialized
            .atomic_usize
            .as_ref()
            .unwrap()
            .load(std::sync::atomic::Ordering::SeqCst)
    );
}

#[test]
fn some_time() {
    use serde_duper::types::{DuperOptionDuration, DuperOptionSystemTime};
    use std::time::{Duration, SystemTime};

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "DuperOptionDuration")]
        duration: Option<Duration>,
        #[serde(with = "DuperOptionSystemTime")]
        system_time: Option<SystemTime>,
    }

    let value = Test {
        duration: Some(Duration::from_secs(3600)),
        system_time: Some(SystemTime::now()),
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
            .unwrap()
            .duration_since(deserialized.system_time.unwrap())
            .unwrap()
            .as_secs()
            < 1
    );
}

#[test]
fn some_path() {
    use serde_duper::types::{DuperOptionPath, DuperOptionPathBuf};
    use std::path::{Path, PathBuf};

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "DuperOptionPathBuf")]
        path_buf: Option<PathBuf>,
    }

    let value = Test {
        path_buf: Some(PathBuf::from("/home/user/file.txt")),
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
        #[serde(borrow, with = "DuperOptionPath")]
        path: Option<&'a Path>,
    }

    let value = TestSerializeOnly {
        path: Some(Path::new("/tmp/test")),
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"TestSerializeOnly({path: Path("/tmp/test")})"#
    );
}

#[test]
#[cfg(unix)]
fn some_ffi_unix() {
    use serde_duper::types::ffi::{
        DuperOptionCStr, DuperOptionCString, DuperOptionOsStr, DuperOptionOsString,
    };
    use std::ffi::{CStr, CString, OsStr, OsString};

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "DuperOptionCString")]
        c_string: Option<CString>,
        #[serde(with = "DuperOptionOsString")]
        os_string: Option<OsString>,
    }

    let value = Test {
        c_string: Some(CString::new("hello").unwrap()),
        os_string: Some(OsString::from("test_os_string")),
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
        #[serde(with = "DuperOptionCStr")]
        c_str: Option<&'a CStr>,
        #[serde(with = "DuperOptionOsStr")]
        os_str: Option<&'a OsStr>,
    }

    let value = TestSerializeOnly {
        c_str: Some(&CString::new("goodbye").unwrap()),
        os_str: Some(&OsString::from("test_os_str")),
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"TestSerializeOnly({c_str: CStr(b"goodbye"), os_str: OsStr({Unix: b"test_os_str"})})"#
    );
}

#[test]
#[cfg(windows)]
fn some_ffi_windows() {
    use serde_duper::types::ffi::{
        DuperOptionCStr, DuperOptionCString, DuperOptionOsStr, DuperOptionOsString,
    };
    use std::ffi::{CStr, CString, OsStr, OsString};

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "DuperOptionCString")]
        c_string: Option<CString>,
        #[serde(with = "DuperOptionOsString")]
        os_string: Option<OsString>,
    }

    let value = Test {
        c_string: Some(CString::new("hello").unwrap()),
        os_string: Some(OsString::from("test_os_string")),
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
        #[serde(with = "DuperOptionCStr")]
        c_str: Option<&'a CStr>,
        #[serde(with = "DuperOptionOsStr")]
        os_str: Option<&'a OsStr>,
    }

    let value = TestSerializeOnly {
        c_str: Some(&CString::new("goodbye").unwrap()),
        os_str: Some(&OsString::from("test_os_str")),
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"TestSerializeOnly({c_str: CStr(b"goodbye"), os_str: OsStr({Windows: [0]})})"#
    );
}

#[test]
fn some_collections() {
    use serde_duper::types::DuperOptionReverse;
    use serde_duper::types::collections::{
        DuperOptionBTreeMap, DuperOptionBTreeSet, DuperOptionBinaryHeap, DuperOptionHashMap,
        DuperOptionHashSet, DuperOptionLinkedList, DuperOptionVecDeque,
    };
    use std::collections::{
        BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque,
    };

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "DuperOptionBinaryHeap")]
        binary_heap: Option<BinaryHeap<i32>>,
        #[serde(with = "DuperOptionBTreeSet")]
        btree_set: Option<BTreeSet<String>>,
        #[serde(with = "DuperOptionBTreeMap")]
        btree_map: Option<BTreeMap<String, i32>>,
        #[serde(with = "DuperOptionLinkedList")]
        linked_list: Option<LinkedList<char>>,
        #[serde(with = "DuperOptionVecDeque")]
        vec_deque: Option<VecDeque<bool>>,
        #[serde(with = "DuperOptionReverse")]
        reverse: Option<std::cmp::Reverse<usize>>,
        #[serde(with = "DuperOptionHashMap")]
        hash_map: Option<HashMap<String, u32>>,
        #[serde(with = "DuperOptionHashSet")]
        hash_set: Option<HashSet<u32>>,
    }

    let value = Test {
        binary_heap: Some(vec![3, 1, 4, 1, 5].into_iter().collect()),
        btree_set: Some(
            vec![
                "apple".to_string(),
                "banana".to_string(),
                "cherry".to_string(),
            ]
            .into_iter()
            .collect(),
        ),
        hash_set: Some(vec![1, 2, 3, 4, 5].into_iter().collect()),
        linked_list: Some(vec!['a', 'b', 'c'].into_iter().collect()),
        vec_deque: Some(vec![true, false, true].into_iter().collect()),
        btree_map: Some(
            vec![
                ("one".to_string(), 1),
                ("two".to_string(), 2),
                ("three".to_string(), 3),
            ]
            .into_iter()
            .collect(),
        ),
        hash_map: Some(
            vec![
                ("uno".to_string(), 1),
                ("dos".to_string(), 2),
                ("tres".to_string(), 3),
            ]
            .into_iter()
            .collect(),
        ),
        reverse: Some(std::cmp::Reverse(42)),
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert!(
        serialized.starts_with(r#"Test({binary_heap: BinaryHeap([5, 3, 4, 1, 1]), btree_set: BTreeSet(["apple", "banana", "cherry"]), btree_map: BTreeMap({one: 1, three: 3, two: 2}), linked_list: LinkedList([Char("a"), Char("b"), Char("c")]), vec_deque: VecDeque([true, false, true]), reverse: Reverse(42), hash_map: HashMap({"#
    ));

    let deserialized: Test = serde_duper::from_string(&serialized).unwrap();
    assert_eq!(
        value.binary_heap.unwrap().into_sorted_vec(),
        deserialized.binary_heap.unwrap().into_sorted_vec()
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
fn some_bytes() {
    use bytes::Bytes;
    use serde_duper::types::DuperOptionBytes;

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "DuperOptionBytes")]
        data: Option<Bytes>,
    }

    let value = Test {
        data: Some(Bytes::copy_from_slice(b"hello")),
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(serialized, r#"Test({data: Bytes(b"hello")})"#);

    let deserialized: Test = serde_duper::from_string(&serialized).unwrap();
    assert_eq!(value.data, deserialized.data);
}

#[test]
#[cfg(feature = "chrono")]
fn some_chrono() {
    use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta, Utc};
    use serde_duper::types::chrono::{
        DuperOptionDateTime, DuperOptionNaiveDate, DuperOptionNaiveDateTime, DuperOptionNaiveTime,
        DuperOptionTimeDelta,
    };

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "DuperOptionDateTime")]
        utc: Option<DateTime<Utc>>,
        #[serde(with = "DuperOptionDateTime")]
        fo: Option<DateTime<FixedOffset>>,
        #[serde(with = "DuperOptionNaiveDateTime")]
        ndt: Option<NaiveDateTime>,
        #[serde(with = "DuperOptionNaiveDate")]
        nd: Option<NaiveDate>,
        #[serde(with = "DuperOptionNaiveTime")]
        nt: Option<NaiveTime>,
        #[serde(with = "DuperOptionTimeDelta")]
        td: Option<TimeDelta>,
    }

    let value = Test {
        utc: Some("2023-10-05T14:30:00Z".parse().unwrap()),
        fo: Some("2023-10-05T11:30:00-03:00".parse().unwrap()),
        ndt: Some("2023-10-05T14:30:00".parse().unwrap()),
        nd: Some("2023-10-05".parse().unwrap()),
        nt: Some("14:30:00".parse().unwrap()),
        td: Some(TimeDelta::days(7)),
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
#[cfg(feature = "uuid")]
fn some_uuid() {
    use serde_duper::types::DuperOptionUuid;
    use uuid::Uuid;

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "DuperOptionUuid")]
        id: Option<Uuid>,
    }

    let value = Test {
        id: Some(Uuid::parse_str("f5ea955b-85bf-4643-be05-0675b2c2b61e").unwrap()),
    };
    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"Test({id: Uuid("f5ea955b-85bf-4643-be05-0675b2c2b61e")})"#
    );

    let deserialized: Test = serde_duper::from_string(&serialized).unwrap();
    assert_eq!(value.id, deserialized.id);
}
