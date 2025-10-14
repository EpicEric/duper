use serde::{Deserialize, Serialize};

#[test]
fn net() {
    use serde_duper::types::net::{
        duper_ip_addr, duper_ipv4_addr, duper_ipv6_addr, duper_socket_addr, duper_socket_addr_v4,
        duper_socket_addr_v6,
    };
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "duper_ip_addr")]
        ip_addr: IpAddr,
        #[serde(with = "duper_ipv4_addr")]
        ipv4_addr: Ipv4Addr,
        #[serde(with = "duper_ipv6_addr")]
        ipv6_addr: Ipv6Addr,
        #[serde(with = "duper_socket_addr")]
        socket_addr: SocketAddr,
        #[serde(with = "duper_socket_addr_v4")]
        socket_addr_v4: SocketAddrV4,
        #[serde(with = "duper_socket_addr_v6")]
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
#[cfg(feature = "bytes")]
fn bytes() {
    use bytes::Bytes;
    use serde_duper::types::duper_bytes;

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "duper_bytes")]
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
        duper_date_time, duper_naive_date, duper_naive_date_time, duper_naive_time,
        duper_time_delta,
    };

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "duper_date_time")]
        utc: DateTime<Utc>,
        #[serde(with = "duper_date_time")]
        fo: DateTime<FixedOffset>,
        #[serde(with = "duper_naive_date_time")]
        ndt: NaiveDateTime,
        #[serde(with = "duper_naive_date")]
        nd: NaiveDate,
        #[serde(with = "duper_naive_time")]
        nt: NaiveTime,
        #[serde(with = "duper_time_delta")]
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
    use serde_duper::types::duper_uuid;
    use uuid::Uuid;

    #[derive(Debug, Serialize, Deserialize)]
    struct Test {
        #[serde(with = "duper_uuid")]
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
