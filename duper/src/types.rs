use crate::ast::DuperIdentifier;

#[non_exhaustive]
pub enum DuperTypes {
    // Integer types
    DecInteger,
    HexInteger,
    OctInteger,
    BinInteger,
    // String types
    Uuid,
    Uuid4,
    Url,
    IsoDatetime,
    IsoDate,
    IsoTime,
    Ip,
    Ipv4,
    Ipv6,
    Cidr,
    Cidrv4,
    Cidrv6,
    Regex,
    Base64,
    Jwt,
}

impl TryFrom<&DuperIdentifier<'_>> for DuperTypes {
    type Error = String;

    fn try_from(identifier: &DuperIdentifier<'_>) -> Result<Self, Self::Error> {
        match identifier.0.as_ref() {
            "Integer" | "DecInteger" => Ok(DuperTypes::DecInteger),
            "HexInteger" => Ok(DuperTypes::HexInteger),
            "OctInteger" => Ok(DuperTypes::OctInteger),
            "BinInteger" => Ok(DuperTypes::BinInteger),
            "Uuid" => Ok(DuperTypes::Uuid),
            "Uuid4" => Ok(DuperTypes::Uuid4),
            "Url" => Ok(DuperTypes::Url),
            "Datetime" | "IsoDatetime" => Ok(DuperTypes::IsoDatetime),
            "Date" | "IsoDate" => Ok(DuperTypes::IsoDate),
            "Time" | "IsoTime" => Ok(DuperTypes::IsoTime),
            "Ip" => Ok(DuperTypes::Ip),
            "Ipv4" => Ok(DuperTypes::Ipv4),
            "Ipv6" => Ok(DuperTypes::Ipv6),
            "Cidr" => Ok(DuperTypes::Cidr),
            "Cidrv4" => Ok(DuperTypes::Cidrv4),
            "Cidrv6" => Ok(DuperTypes::Cidrv6),
            "Regex" => Ok(DuperTypes::Regex),
            "Base64" => Ok(DuperTypes::Base64),
            "Jwt" => Ok(DuperTypes::Jwt),
            _ => Err(format!("Unsupported type {}", identifier.0.as_ref())),
        }
    }
}
