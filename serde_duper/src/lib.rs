mod de;
mod error;
mod ser;

pub use de::{Deserializer, from_str, from_value};
pub use duper::{DuperInner, DuperValue};
pub use error::{Error, ErrorImpl, ErrorKind, Result};
pub use ser::{Serializer, to_duper, to_string, to_string_pretty};
