#![cfg(feature = "std")]
use crate::encoding::major_type::MajorType;
use crate::serialize::{Write, WriteError};

// We re-export everything in this namespace. We only use multiple files for simplification
// of the code.
mod array;
pub use array::*;
#[cfg(test)]
mod array_test;

mod bytes;
pub use bytes::*;
#[cfg(test)]
mod bytes_test;

mod map;
pub use map::*;
#[cfg(test)]
mod map_test;

mod numbers;
pub use numbers::*;
#[cfg(test)]
mod numbers_test;

mod simple;
pub use simple::*;
#[cfg(test)]
mod simple_test;

mod tag;
pub use tag::*;
#[cfg(test)]
mod tag_test;

mod text;
use crate::encoding::minor_type::MinorType;
use crate::serialize::values::{Value, ValueInner};
use crate::serialize::write::WriteTo;
use std::ops::Deref;
pub use text::*;

#[cfg(test)]
mod text_test;

/// Inner Value type. This will contain references to data that is needed to serialize the
/// value. It has no ownership, however.
/// If there are no references needed (e.g. if the whole data is contained in the Major+
/// Minor types), use NoRef().
/// This type is not copy, because of vectors.
#[derive(Clone, Debug, PartialEq)]
enum OwnedValueInner {
    NoRef(),
    ByteString(Vec<u8>),
    Text(String),
    Array(Vec<OwnedValue>),
    Map(Vec<(OwnedValue, OwnedValue)>),
    IndefiniteByteString(Vec<Vec<u8>>),
    IndefiniteText(Vec<String>),
    IndefiniteArray(Vec<OwnedValue>),
    IndefiniteMap(Vec<(OwnedValue, OwnedValue)>),
    Tag(Box<OwnedValue>),
}

impl WriteTo for OwnedValueInner {
    fn len(&self) -> usize {
        match self {
            OwnedValueInner::NoRef() => 0,
            OwnedValueInner::ByteString(s) => s.len(),
            OwnedValueInner::Text(t) => t.len(),
            OwnedValueInner::Array(a) => a.iter().fold(0, |p, i| p + i.len()),
            OwnedValueInner::Map(kv) => kv.iter().fold(0, |p, (k, v)| p + k.len() + v.len()),
            OwnedValueInner::IndefiniteByteString(chunks) => {
                chunks
                    .iter()
                    .fold(0, |p, i| p + Value::from_byte_string(i).len())
                    + MajorType::Break().len()
            }
            OwnedValueInner::IndefiniteText(chunks) => {
                chunks.iter().fold(0, |p, i| p + Value::from_text(i).len())
                    + MajorType::Break().len()
            }
            OwnedValueInner::IndefiniteArray(values) => {
                values.iter().fold(0, |p, i| p + i.len()) + MajorType::Break().len()
            }
            OwnedValueInner::IndefiniteMap(pairs) => {
                pairs.iter().fold(0, |p, (k, v)| p + k.len() + v.len()) + MajorType::Break().len()
            }
            OwnedValueInner::Tag(v) => v.len(),
        }
    }

    fn write_to<W: Write>(&self, w: &mut W) -> Result<usize, WriteError> {
        match self {
            OwnedValueInner::NoRef() => Ok(0),
            OwnedValueInner::ByteString(s) => w.write(s.deref()),
            OwnedValueInner::Text(t) => w.write(t.as_bytes()),
            OwnedValueInner::Array(a) => {
                let mut sz = 0;
                for i in a {
                    sz += i.write_to(w)?;
                }
                Ok(sz)
            }
            OwnedValueInner::Map(kv) => {
                let mut sz = 0;
                for (k, v) in kv {
                    sz += k.write_to(w)?;
                    sz += v.write_to(w)?;
                }
                Ok(sz)
            }
            OwnedValueInner::IndefiniteByteString(chunks) => {
                let mut sz = 0;
                for i in chunks {
                    sz += Value::from_byte_string(&i).write_to(w)?;
                }
                sz += MajorType::Break().write_to(w)?;
                Ok(sz)
            }
            OwnedValueInner::IndefiniteText(chunks) => {
                for i in chunks {
                    Value::from_text(&i).write_to(w)?;
                }
                MajorType::Break().write_to(w)
            }
            OwnedValueInner::IndefiniteArray(values) => {
                for i in values {
                    i.write_to(w)?;
                }
                MajorType::Break().write_to(w)
            }
            OwnedValueInner::IndefiniteMap(pairs) => {
                for (k, v) in pairs {
                    k.write_to(w)?;
                    v.write_to(w)?;
                }
                MajorType::Break().write_to(w)
            }
            OwnedValueInner::Tag(v) => v.write_to(w),
        }
    }
}

/// A CBOR Value. Can represent any definitely-sized CBOR value possible.
///
/// The only values that aren't representable by using this type are those that have unknown
/// sizes; arrays and maps where a break is being used. To serialize those values, use a
/// serializer directly, don't use this Value type.
///
/// This Value owns any data referred to it.
#[derive(Clone, Debug, PartialEq)]
pub struct OwnedValue {
    major: MajorType,
    inner: OwnedValueInner,
}

impl OwnedValue {
    /// We do not expose this method because a user should use the values functions (like
    /// [u8] or [map]) to create values, or deserialize. Otherwise, non-CBOR byte streams
    /// could be created.
    pub(crate) fn simple(major: MajorType) -> Self {
        Self::with_inner(major, OwnedValueInner::NoRef())
    }

    /// We do not expose this method because a user should use the values functions (like
    /// [u8] or [map]) to create values, or deserialize. Otherwise, non-CBOR byte streams
    /// could be created.
    pub(crate) fn from_byte_string(byte_string: &[u8]) -> Self {
        Self::with_inner(
            MajorType::ByteString(MinorType::size(byte_string.len())),
            OwnedValueInner::ByteString(byte_string.to_vec()),
        )
    }

    /// We do not expose this method because a user should use the values functions (like
    /// [u8] or [map]) to create values, or deserialize. Otherwise, non-CBOR byte streams
    /// could be created.
    pub(crate) fn from_text(text: &str) -> Self {
        Self::with_inner(
            MajorType::Text(MinorType::size(text.len())),
            OwnedValueInner::Text(text.to_owned()),
        )
    }

    /// We do not expose this method because a user should use the values functions (like
    /// [u8] or [map]) to create values, or deserialize. Otherwise, non-CBOR byte streams
    /// could be created.
    pub(crate) fn from_array<V: AsRef<[OwnedValue]>>(array: V) -> Self {
        Self::with_inner(
            MajorType::Array(MinorType::size(array.as_ref().len())),
            OwnedValueInner::Array(array.as_ref().to_vec()),
        )
    }

    /// We do not expose this method because a user should use the values functions (like
    /// [u8] or [map]) to create values, or deserialize. Otherwise, non-CBOR byte streams
    /// could be created.
    pub(crate) fn from_map<M: AsRef<[(OwnedValue, OwnedValue)]>>(map: M) -> Self {
        Self::with_inner(
            MajorType::Map(MinorType::size(map.as_ref().len())),
            OwnedValueInner::Map(map.as_ref().to_vec()),
        )
    }

    /// We do not expose this method because a user should use the values functions (like
    /// [u8] or [map]) to create values, or deserialize. Otherwise, non-CBOR byte streams
    /// could be created.
    pub(crate) fn from_indefinite_byte_string<V: AsRef<[u8]>>(
        indefinite_byte_string: &[V],
    ) -> Self {
        Self::with_inner(
            MajorType::ByteString(MinorType::Indefinite()),
            OwnedValueInner::IndefiniteByteString(
                indefinite_byte_string
                    .iter()
                    .map(|x| x.as_ref().to_vec())
                    .collect(),
            ),
        )
    }

    /// We do not expose this method because a user should use the values functions (like
    /// [u8] or [map]) to create values, or deserialize. Otherwise, non-CBOR byte streams
    /// could be created.
    pub(crate) fn from_indefinite_text<S: ToString>(indefinite_text: &[S]) -> Self {
        Self::with_inner(
            MajorType::Text(MinorType::Indefinite()),
            OwnedValueInner::IndefiniteText(
                indefinite_text.iter().map(ToString::to_string).collect(),
            ),
        )
    }

    /// We do not expose this method because a user should use the values functions (like
    /// [u8] or [map]) to create values, or deserialize. Otherwise, non-CBOR byte streams
    /// could be created.
    pub(crate) fn from_indefinite_array<A: AsRef<[OwnedValue]>>(indefinite_array: A) -> Self {
        Self::with_inner(
            MajorType::Array(MinorType::Indefinite()),
            OwnedValueInner::IndefiniteArray(indefinite_array.as_ref().to_vec()),
        )
    }

    /// We do not expose this method because a user should use the values functions (like
    /// [u8] or [map]) to create values, or deserialize. Otherwise, non-CBOR byte streams
    /// could be created.
    pub(crate) fn from_indefinite_map<M: AsRef<[(OwnedValue, OwnedValue)]>>(
        indefinite_map: M,
    ) -> Self {
        Self::with_inner(
            MajorType::Map(MinorType::Indefinite()),
            OwnedValueInner::IndefiniteMap(indefinite_map.as_ref().to_vec()),
        )
    }

    /// We do not expose this method because a user should use the values functions (like
    /// [u8] or [map]) to create values, or deserialize. Otherwise, non-CBOR byte streams
    /// could be created.
    pub(crate) fn from_tag(tag: u64, inner: OwnedValue) -> Self {
        Self::with_inner(
            MajorType::Tag(MinorType::size(tag as usize)),
            OwnedValueInner::Tag(Box::new(inner)),
        )
    }

    fn with_inner(major: MajorType, inner: OwnedValueInner) -> Self {
        Self { major, inner }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        // This skips the Write trait and just implement its own vector iterator.
        // The Write trait has error handling, and we really don't need that here,
        // so this is simpler.
        struct Writer<'a> {
            vector: &'a mut Vec<u8>,
        }
        impl Write for Writer<'_> {
            fn write(&mut self, bytes: &[u8]) -> Result<usize, WriteError> {
                self.vector.extend_from_slice(bytes);
                Ok(bytes.len())
            }
        }

        let mut vector = Vec::with_capacity(self.len());
        self.write_to(&mut Writer {
            vector: &mut vector,
        })
        .expect("Unexpected error.");
        vector
    }

    pub fn len(&self) -> usize {
        WriteTo::len(self)
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl WriteTo for OwnedValue {
    fn len(&self) -> usize {
        self.major.len() + self.inner.len()
    }

    fn write_to<W: Write>(&self, w: &mut W) -> Result<usize, WriteError> {
        Ok(self.major.write_to(w)? + self.inner.write_to(w)?)
    }
}

impl From<ValueInner<'_>> for OwnedValueInner {
    fn from(inner: ValueInner<'_>) -> Self {
        match inner {
            ValueInner::NoRef() => OwnedValueInner::NoRef(),
            ValueInner::ByteString(b) => OwnedValueInner::ByteString(b.to_vec()),
            ValueInner::Text(s) => OwnedValueInner::Text(s.to_string()),
            ValueInner::Array(a) => OwnedValueInner::Array(a.iter().map(Into::into).collect()),
            ValueInner::Map(m) => {
                OwnedValueInner::Map(m.iter().map(|(k, v)| (k.into(), v.into())).collect())
            }
            _ => unreachable!(),
        }
    }
}

impl From<Value<'_>> for OwnedValue {
    fn from(v: Value<'_>) -> Self {
        Self {
            major: v.major,
            inner: v.inner.into(),
        }
    }
}

impl From<&Value<'_>> for OwnedValue {
    fn from(v: &Value<'_>) -> Self {
        Self {
            major: v.major,
            inner: v.inner.into(),
        }
    }
}
