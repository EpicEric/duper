pub mod bytes;
mod de;
mod error;
mod ser;
pub mod types;

pub use de::{Deserializer, from_string, from_value};
pub use duper::{DuperInner, DuperValue};
pub use error::{Error, ErrorImpl, ErrorKind, Result};
pub use ser::{Serializer, to_duper, to_string, to_string_minified, to_string_pretty};
