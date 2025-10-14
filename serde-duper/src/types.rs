use serde_core::{Deserialize, Deserializer, Serialize, Serializer};

#[macro_export]
macro_rules! duper_serde_module {
    (
        $mod_name:ident,
        $wrapped_type:ty,
        $type_name:literal
    ) => {
        pub mod $mod_name {
            use super::*;
            use $wrapped_type as WrappedType;

            struct Wrapper(WrappedType);

            impl Serialize for Wrapper
            where
                WrappedType: Serialize,
            {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    serializer.serialize_newtype_struct($type_name, &self.0)
                }
            }

            impl<'de> Deserialize<'de> for Wrapper
            where
                WrappedType: Deserialize<'de>,
            {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: Deserializer<'de>,
                {
                    Ok(Wrapper(WrappedType::deserialize(deserializer)?))
                }
            }

            pub fn serialize<S>(value: &WrappedType, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let duper_type = Wrapper(value.clone());
                duper_type.serialize(serializer)
            }

            pub fn deserialize<'de, D>(deserializer: D) -> Result<WrappedType, D::Error>
            where
                D: Deserializer<'de>,
            {
                Ok(Wrapper::deserialize(deserializer)?.0)
            }
        }
    };
}

pub mod net {
    use super::*;

    duper_serde_module!(duper_ip_addr, ::std::net::IpAddr, "IpAddr");
    duper_serde_module!(duper_ipv4_addr, ::std::net::Ipv4Addr, "Ipv4Addr");
    duper_serde_module!(duper_ipv6_addr, ::std::net::Ipv6Addr, "Ipv6Addr");
    duper_serde_module!(duper_socket_addr, ::std::net::SocketAddr, "SocketAddr");
    duper_serde_module!(
        duper_socket_addr_v4,
        ::std::net::SocketAddrV4,
        "SocketAddrV4"
    );
    duper_serde_module!(
        duper_socket_addr_v6,
        ::std::net::SocketAddrV6,
        "SocketAddrV6"
    );
}

#[cfg(feature = "bytes")]
duper_serde_module!(duper_bytes, ::bytes::Bytes, "Bytes");

#[cfg(feature = "uuid")]
duper_serde_module!(duper_uuid, ::uuid::Uuid, "Uuid");

#[cfg(feature = "chrono")]
pub mod chrono {
    use super::*;

    duper_serde_module!(
        duper_naive_date_time,
        ::chrono::NaiveDateTime,
        "NaiveDateTime"
    );
    duper_serde_module!(duper_naive_date, ::chrono::NaiveDate, "NaiveDate");
    duper_serde_module!(duper_naive_time, ::chrono::NaiveTime, "NaiveTime");
    duper_serde_module!(duper_time_delta, ::chrono::TimeDelta, "TimeDelta");

    pub mod duper_date_time {
        use super::*;
        use ::chrono::{DateTime as WrappedType, TimeZone};

        struct DateTime<T: TimeZone>(WrappedType<T>);

        impl<T> Serialize for DateTime<T>
        where
            T: TimeZone,
            WrappedType<T>: Serialize,
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_newtype_struct("DateTime", &self.0)
            }
        }

        impl<'de, T> Deserialize<'de> for DateTime<T>
        where
            T: TimeZone,
            WrappedType<T>: Deserialize<'de>,
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                Ok(DateTime(WrappedType::<T>::deserialize(deserializer)?))
            }
        }

        pub fn serialize<S, T>(value: &WrappedType<T>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
            T: TimeZone,
            WrappedType<T>: Serialize,
        {
            let duper_type = DateTime(value.clone());
            duper_type.serialize(serializer)
        }

        pub fn deserialize<'de, D, T>(deserializer: D) -> Result<WrappedType<T>, D::Error>
        where
            D: Deserializer<'de>,
            T: TimeZone,
            WrappedType<T>: Deserialize<'de>,
        {
            let wrapper = DateTime::<T>::deserialize(deserializer)?;
            Ok(wrapper.0)
        }
    }
}
