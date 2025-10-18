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
        ip_addr: None,
        ipv4_addr: None,
        ipv6_addr: None,
        socket_addr: None,
        socket_addr_v4: None,
        socket_addr_v6: None,
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"Test({ip_addr: IpAddr(null), ipv4_addr: Ipv4Addr(null), ipv6_addr: Ipv6Addr(null), socket_addr: SocketAddr(null), socket_addr_v4: SocketAddrV4(null), socket_addr_v6: SocketAddrV6(null)})"#
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
        nz_i8: None,
        nz_i16: None,
        nz_i32: None,
        nz_i64: None,
        nz_i128: None,
        nz_isize: None,
        nz_u8: None,
        nz_u16: None,
        nz_u32: None,
        nz_u64: None,
        nz_u128: None,
        nz_usize: None,
        wrapping: None,
        saturating: None,
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"Test({nz_i8: NonZeroI8(null), nz_i16: NonZeroI16(null), nz_i32: NonZeroI32(null), nz_i64: NonZeroI64(null), nz_i128: NonZeroI128(null), nz_isize: NonZeroIsize(null), nz_u8: NonZeroU8(null), nz_u16: NonZeroU16(null), nz_u32: NonZeroU32(null), nz_u64: NonZeroU64(null), nz_u128: NonZeroU128(null), nz_usize: NonZeroUsize(null), wrapping: Wrapping(null), saturating: Saturating(null)})"#
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
        atomic_bool: None,
        atomic_i8: None,
        atomic_i16: None,
        atomic_i32: None,
        atomic_i64: None,
        atomic_isize: None,
        atomic_u8: None,
        atomic_u16: None,
        atomic_u32: None,
        atomic_u64: None,
        atomic_usize: None,
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"Test({atomic_bool: AtomicBool(null), atomic_i8: AtomicI8(null), atomic_i16: AtomicI16(null), atomic_i32: AtomicI32(null), atomic_i64: AtomicI64(null), atomic_isize: AtomicIsize(null), atomic_u8: AtomicU8(null), atomic_u16: AtomicU16(null), atomic_u32: AtomicU32(null), atomic_u64: AtomicU64(null), atomic_usize: AtomicUsize(null)})"#
    );

    let deserialized: Test = serde_duper::from_string(&serialized).unwrap();
    assert!(deserialized.atomic_bool.as_ref().is_none());
    assert!(deserialized.atomic_i8.is_none());
    assert!(deserialized.atomic_i16.is_none());
    assert!(deserialized.atomic_i32.is_none());
    assert!(deserialized.atomic_i64.is_none());
    assert!(deserialized.atomic_isize.is_none());
    assert!(deserialized.atomic_u8.is_none());
    assert!(deserialized.atomic_u16.is_none());
    assert!(deserialized.atomic_u32.is_none());
    assert!(deserialized.atomic_u64.is_none());
    assert!(deserialized.atomic_usize.is_none());
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
        duration: None,
        system_time: None,
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"Test({duration: Duration(null), system_time: SystemTime(null)})"#
    );

    let deserialized: Test = serde_duper::from_string(&serialized).unwrap();
    assert_eq!(value.duration, deserialized.duration);
    assert!(value.system_time.is_none());
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

    let value = Test { path_buf: None };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(serialized, r#"Test({path_buf: PathBuf(null)})"#);

    let deserialized: Test = serde_duper::from_string(&serialized).unwrap();
    assert_eq!(value.path_buf, deserialized.path_buf);

    #[derive(Debug, Serialize)]
    struct TestSerializeOnly<'a> {
        #[serde(borrow, with = "DuperOptionPath")]
        path: Option<&'a Path>,
    }

    let value = TestSerializeOnly { path: None };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(serialized, r#"TestSerializeOnly({path: Path(null)})"#);
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
        c_string: None,
        os_string: None,
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"Test({c_string: CString(null), os_string: OsString(null)})"#
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
        c_str: None,
        os_str: None,
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"TestSerializeOnly({c_str: CStr(null), os_str: OsStr(null)})"#
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
        c_string: None,
        os_string: None,
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"Test({c_string: CString(null), os_string: OsString(null)})"#
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
        c_str: None,
        os_str: None,
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"TestSerializeOnly({c_str: CStr(null), os_str: OsStr(null)})"#
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
        binary_heap: None,
        btree_set: None,
        hash_set: None,
        linked_list: None,
        vec_deque: None,
        btree_map: None,
        hash_map: None,
        reverse: None,
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"Test({binary_heap: BinaryHeap(null), btree_set: BTreeSet(null), btree_map: BTreeMap(null), linked_list: LinkedList(null), vec_deque: VecDeque(null), reverse: Reverse(null), hash_map: HashMap(null), hash_set: HashSet(null)})"#
    );

    let deserialized: Test = serde_duper::from_string(&serialized).unwrap();
    assert!(deserialized.binary_heap.is_none());
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

    let value = Test { data: None };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(serialized, r#"Test({data: Bytes(null)})"#);

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
        utc: None,
        fo: None,
        ndt: None,
        nd: None,
        nt: None,
        td: None,
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(
        serialized,
        r#"Test({utc: DateTime(null), fo: DateTime(null), ndt: NaiveDateTime(null), nd: NaiveDate(null), nt: NaiveTime(null), td: TimeDelta(null)})"#
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

    let value = Test { id: None };
    let serialized = serde_duper::to_string(&value).unwrap();
    assert_eq!(serialized, r#"Test({id: Uuid(null)})"#);

    let deserialized: Test = serde_duper::from_string(&serialized).unwrap();
    assert_eq!(value.id, deserialized.id);
}
