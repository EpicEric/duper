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
    let deserialized: Test = serde_duper::from_string(&serialized).unwrap();

    assert_eq!(value.duration, deserialized.duration);
    // SystemTime comparison with small epsilon for potential nanosecond differences
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
    struct Test<'a> {
        #[serde(with = "DuperPathBuf")]
        path_buf: PathBuf,
        #[serde(borrow, with = "DuperPath")]
        path: &'a Path,
    }

    let value = Test {
        path_buf: PathBuf::from("/home/user/file.txt"),
        path: Path::new("/tmp/test"),
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    let deserialized: Test = serde_duper::from_string(&serialized).unwrap();

    assert_eq!(value.path_buf, deserialized.path_buf);
    assert_eq!(value.path, deserialized.path);
}

#[test]
fn ffi() {
    use serde_duper::types::{DuperCString, DuperOsString};
    use std::ffi::{CString, OsString};

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
    let deserialized: Test = serde_duper::from_string(&serialized).unwrap();

    assert_eq!(value.c_string, deserialized.c_string);
    assert_eq!(value.os_string, deserialized.os_string);
}

#[test]
fn smart_pointers() {
    use serde_duper::types::{
        DuperArc, DuperBox, DuperCell, DuperMutex, DuperRc, DuperRefCell, DuperRwLock,
    };
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;
    use std::sync::{Arc, Mutex, RwLock};

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "DuperBox")]
        boxed: Box<i32>,
        #[serde(with = "DuperRc")]
        rc: Rc<String>,
        #[serde(with = "DuperArc")]
        arc: Arc<Vec<i32>>,
        #[serde(with = "DuperCell")]
        cell: Cell<bool>,
        #[serde(with = "DuperRefCell")]
        ref_cell: RefCell<u32>,
        #[serde(with = "DuperMutex")]
        mutex: Mutex<f64>,
        #[serde(with = "DuperRwLock")]
        rw_lock: RwLock<char>,
    }

    let value = Test {
        boxed: Box::new(42),
        rc: Rc::new("hello".to_string()),
        arc: Arc::new(vec![1, 2, 3]),
        cell: Cell::new(true),
        ref_cell: RefCell::new(100),
        mutex: Mutex::new(3.14),
        rw_lock: RwLock::new('x'),
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    let deserialized: Test = serde_duper::from_string(&serialized).unwrap();

    assert_eq!(*value.boxed, *deserialized.boxed);
    assert_eq!(*value.rc, *deserialized.rc);
    assert_eq!(*value.arc, *deserialized.arc);
    assert_eq!(value.cell.get(), deserialized.cell.get());
    assert_eq!(*value.ref_cell.borrow(), *deserialized.ref_cell.borrow());
    assert_eq!(
        *value.mutex.lock().unwrap(),
        *deserialized.mutex.lock().unwrap()
    );
    assert_eq!(
        *value.rw_lock.read().unwrap(),
        *deserialized.rw_lock.read().unwrap()
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
        #[serde(with = "DuperHashSet")]
        hash_set: HashSet<u32>,
        #[serde(with = "DuperLinkedList")]
        linked_list: LinkedList<char>,
        #[serde(with = "DuperVecDeque")]
        vec_deque: VecDeque<bool>,
        #[serde(with = "DuperBTreeMap")]
        btree_map: BTreeMap<String, i32>,
        #[serde(with = "DuperHashMap")]
        hash_map: HashMap<u32, String>,
        #[serde(with = "DuperReverse")]
        reverse: std::cmp::Reverse<usize>,
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
            (1, "uno".to_string()),
            (2, "dos".to_string()),
            (3, "tres".to_string()),
        ]
        .into_iter()
        .collect(),
        reverse: std::cmp::Reverse(42),
    };

    let serialized = serde_duper::to_string(&value).unwrap();
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
fn cow() {
    use serde_duper::types::DuperCow;
    use std::borrow::Cow;

    #[derive(Debug, Serialize, Deserialize)]
    struct Test<'a> {
        #[serde(with = "DuperCow")]
        cow_owned: Cow<'a, str>,
        #[serde(with = "DuperCow")]
        cow_borrowed: Cow<'a, str>,
    }

    let borrowed = "hello";
    let value = Test {
        cow_owned: Cow::Owned("world".to_string()),
        cow_borrowed: Cow::Borrowed(borrowed),
    };

    let serialized = serde_duper::to_string(&value).unwrap();
    let deserialized: Test<'static> = serde_duper::from_string(&serialized).unwrap();

    assert_eq!(value.cow_owned, deserialized.cow_owned);
    assert_eq!(value.cow_borrowed, deserialized.cow_borrowed);
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
    assert_eq!(
        serde_duper::to_string(&value).unwrap(),
        r#"Test({id: Uuid("f5ea955b-85bf-4643-be05-0675b2c2b61e")})"#
    );
}
