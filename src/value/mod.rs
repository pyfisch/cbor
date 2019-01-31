//! CBOR values, keys and serialization routines.

use std::collections::BTreeMap;

mod de;
mod order;
mod ser;

/// An enum covering all possible CBOR values.
/// 
/// Maps are sorted according to the canonical ordering
/// described in [RFC 7049 bis].
/// Therefore values are unambiguously serialized
/// to a canonical form of CBOR from the same RFC.
/// 
/// [RFC 7049 bis]: https://tools.ietf.org/html/draft-ietf-cbor-7049bis-04#section-2
#[derive(Clone, Debug)]
pub enum Value {
    /// Represents the absence of a value or the value undefined.
    Null,
    /// Represents a boolean value.
    Bool(bool),
    /// Integer CBOR numbers.
    /// 
    /// The biggest value that can be represented is 2^64 - 1.
    /// While the smallest value is -2^64.
    /// Values outside this range can't be serialized
    /// and will cause an error.
    Integer(i128),
    /// Represents a floating point value.
    Float(f64),
    /// Represents a byte string.
    Bytes(Vec<u8>),
    /// Represents an UTF-8 encoded string.
    Text(String),
    /// Represents an array of values.
    Array(Vec<Value>),
    /// Represents a map.
    /// 
    /// Maps are also called tables, dictionaries, hashes, or objects (in JSON).
    /// While any value can be used as a CBOR key
    /// it is better to use only one type of key in a map
    /// to avoid ambiguity.
    /// If floating point values are used as keys they are compared bit-by-bit for equality.
    /// If arrays or maps are used as keys the comparisons
    /// to establish canonical order may be slow and therefore insertion
    /// and retrieval of values will be slow too.
    Map(BTreeMap<Value, Value>),
    // The hidden variant allows the enum to be extended
    // with variants for tags and simple values.
    #[doc(hidden)]
    __Hidden,
}
