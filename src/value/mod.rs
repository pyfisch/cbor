//! CBOR values, keys and serialization routines.

pub mod ser;
pub mod value;

pub use self::ser::to_value;
pub use self::value::{from_value, ObjectKey, Value};
