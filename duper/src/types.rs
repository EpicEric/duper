#[non_exhaustive]
pub enum DuperTypes {
    DecInteger,
    HexInteger,
    OctInteger,
    BinInteger,
}

impl TryFrom<&str> for DuperTypes {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Integer" | "DecInteger" => Ok(DuperTypes::DecInteger),
            "HexInteger" => Ok(DuperTypes::HexInteger),
            "OctInteger" => Ok(DuperTypes::OctInteger),
            "BinInteger" => Ok(DuperTypes::BinInteger),
            _ => Err(()),
        }
    }
}
