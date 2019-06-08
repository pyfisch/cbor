//! CBOR values, keys and serialization routines.

mod order;
pub mod ser;
pub mod value;

use std::collections::BTreeMap;

pub use self::ser::to_value;
pub use self::value::from_value;

/// The `Value` enum, a loosely typed way of representing any valid CBOR value.
///
/// Maps are sorted according to the canonical ordering
/// described in [RFC 7049 bis].
/// Therefore values are unambiguously serialized
/// to a canonical form of CBOR from the same RFC.
///
/// [RFC 7049 bis]: https://tools.ietf.org/html/draft-ietf-cbor-7049bis-04#section-4.10
#[derive(Clone, Debug)]
pub enum Value {
    /// Represents the absence of a value or the value undefined.
    Null,
    /// Represents a boolean value.
    Bool(bool),
    /// Represents an unsigned integer.
    U64(u64),
    /// Represents a signed integer.
    I64(i64),
    /// Represents a byte string.
    Bytes(Vec<u8>),
    /// Represents an UTF-8 string.
    String(String),
    /// Represents a list.
    Array(Vec<Value>),
    /// Represents a map.
    Object(BTreeMap<Value, Value>),
    /// Represents a floating point value.
    F64(f64),
}
