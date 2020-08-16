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
use crate::serialize::write::WriteTo;
pub use text::*;

#[cfg(test)]
mod text_test;

/// Inner Value type. This will contain references to data that is needed to serialize the
/// value. It has no ownership, however.
/// If there are no references needed (e.g. if the whole data is contained in the Major+
/// Minor types), use NoRef().
#[derive(Copy, Clone, Debug, PartialEq)]
enum ValueInner<'a> {
    NoRef(),
    ByteString(&'a [u8]),
    Text(&'a str),
    Array(&'a [Value<'a>]),
    Map(&'a [(Value<'a>, Value<'a>)]),
    IndefiniteByteString(&'a [&'a [u8]]),
    IndefiniteText(&'a [&'a str]),
    IndefiniteArray(&'a [Value<'a>]),
    IndefiniteMap(&'a [(Value<'a>, Value<'a>)]),
    Tag(&'a Value<'a>),
}

impl<'a> WriteTo for ValueInner<'a> {
    fn len(&self) -> usize {
        match self {
            ValueInner::NoRef() => 0,
            ValueInner::ByteString(s) => s.len(),
            ValueInner::Text(t) => t.len(),
            ValueInner::Array(a) => {
                let mut total: usize = 0;
                for i in 0..a.len() {
                    total += a[i].len();
                }
                total
            }
            ValueInner::Map(kv) => {
                let mut total: usize = 0;
                for i in 0..kv.len() {
                    total += kv[i].0.len() + kv[i].1.len();
                }
                total
            }
            ValueInner::IndefiniteByteString(chunks) => {
                let mut total: usize = 0;
                for i in 0..chunks.len() {
                    total += Value::from_byte_string(chunks[i]).len();
                }
                total + MajorType::Break().len()
            }
            ValueInner::IndefiniteText(chunks) => {
                let mut total: usize = 0;
                for i in 0..chunks.len() {
                    total += Value::from_text(chunks[i]).len();
                }
                total + MajorType::Break().len()
            }
            ValueInner::IndefiniteArray(values) => {
                let mut total: usize = 0;
                for i in 0..values.len() {
                    total += values[i].len();
                }
                total + MajorType::Break().len()
            }
            ValueInner::IndefiniteMap(pairs) => {
                let mut total: usize = 0;
                for i in 0..pairs.len() {
                    total += pairs[i].0.len() + pairs[i].1.len();
                }
                total + MajorType::Break().len()
            }
            ValueInner::Tag(v) => v.len(),
        }
    }

    fn write_to<W: Write>(&self, w: &mut W) -> Result<usize, WriteError> {
        match self {
            ValueInner::NoRef() => Ok(0),
            ValueInner::ByteString(s) => w.write(*s),
            ValueInner::Text(t) => w.write(t.as_bytes()),
            ValueInner::Array(a) => {
                let mut sz = 0;
                for i in 0..a.len() {
                    sz += a[i].write_to(w)?;
                }
                Ok(sz)
            }
            ValueInner::Map(kv) => {
                let mut sz = 0;
                for i in 0..kv.len() {
                    sz += kv[i].0.write_to(w)?;
                    sz += kv[i].1.write_to(w)?;
                }
                Ok(sz)
            }
            ValueInner::IndefiniteByteString(chunks) => {
                let mut sz = 0;
                for i in 0..chunks.len() {
                    sz += Value::from_byte_string(chunks[i]).write_to(w)?;
                }
                sz += MajorType::Break().write_to(w)?;
                Ok(sz)
            }
            ValueInner::IndefiniteText(chunks) => {
                for i in 0..chunks.len() {
                    Value::from_text(chunks[i]).write_to(w)?;
                }
                MajorType::Break().write_to(w)
            }
            ValueInner::IndefiniteArray(values) => {
                for i in 0..values.len() {
                    values[i].write_to(w)?;
                }
                MajorType::Break().write_to(w)
            }
            ValueInner::IndefiniteMap(pairs) => {
                for i in 0..pairs.len() {
                    pairs[i].0.write_to(w)?;
                    pairs[i].1.write_to(w)?;
                }
                MajorType::Break().write_to(w)
            }
            ValueInner::Tag(v) => v.write_to(w),
        }
    }
}

/// A CBOR Value. Can represent any definitely-sized CBOR value possible.
///
/// The only values that aren't representable by using this type are those that have unknown
/// sizes; arrays and maps where a break is being used. To serialize those values, use a
/// serializer directly, don't use this Value type.
///
/// The lifetime is used for bytes and string references. This Value does not own any sliced
/// data itself. For this, use an [OwnedValue] (which is incompatible with no_std).
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Value<'a> {
    major: MajorType,
    inner: ValueInner<'a>,
}

impl<'a> Value<'a> {
    /// We do not expose this method because a user should use the values functions (like
    /// [u8] or [map]) to create values, or deserialize. Otherwise, non-CBOR byte streams
    /// could be created.
    pub(crate) fn simple(major: MajorType) -> Self {
        Self::with_inner(major, ValueInner::NoRef())
    }

    /// We do not expose this method because a user should use the values functions (like
    /// [u8] or [map]) to create values, or deserialize. Otherwise, non-CBOR byte streams
    /// could be created.
    pub(crate) fn from_byte_string(byte_string: &'a [u8]) -> Self {
        Self::with_inner(
            MajorType::ByteString(MinorType::size(byte_string.len())),
            ValueInner::ByteString(byte_string),
        )
    }

    /// We do not expose this method because a user should use the values functions (like
    /// [u8] or [map]) to create values, or deserialize. Otherwise, non-CBOR byte streams
    /// could be created.
    pub(crate) fn from_text(text: &'a str) -> Self {
        Self::with_inner(
            MajorType::Text(MinorType::size(text.len())),
            ValueInner::Text(text),
        )
    }

    /// We do not expose this method because a user should use the values functions (like
    /// [u8] or [map]) to create values, or deserialize. Otherwise, non-CBOR byte streams
    /// could be created.
    pub(crate) fn from_array(array: &'a [Value<'a>]) -> Self {
        Self::with_inner(
            MajorType::Array(MinorType::size(array.len())),
            ValueInner::Array(array),
        )
    }

    /// We do not expose this method because a user should use the values functions (like
    /// [u8] or [map]) to create values, or deserialize. Otherwise, non-CBOR byte streams
    /// could be created.
    pub(crate) fn from_map(map: &'a [(Value<'a>, Value<'a>)]) -> Self {
        Self::with_inner(
            MajorType::Map(MinorType::size(map.len())),
            ValueInner::Map(map),
        )
    }

    /// We do not expose this method because a user should use the values functions (like
    /// [u8] or [map]) to create values, or deserialize. Otherwise, non-CBOR byte streams
    /// could be created.
    pub(crate) fn from_indefinite_byte_string(indefinite_byte_string: &'a [&'a [u8]]) -> Self {
        Self::with_inner(
            MajorType::ByteString(MinorType::Indefinite()),
            ValueInner::IndefiniteByteString(indefinite_byte_string),
        )
    }

    /// We do not expose this method because a user should use the values functions (like
    /// [u8] or [map]) to create values, or deserialize. Otherwise, non-CBOR byte streams
    /// could be created.
    pub(crate) fn from_indefinite_text(indefinite_text: &'a [&'a str]) -> Self {
        Self::with_inner(
            MajorType::Text(MinorType::Indefinite()),
            ValueInner::IndefiniteText(indefinite_text),
        )
    }

    /// We do not expose this method because a user should use the values functions (like
    /// [u8] or [map]) to create values, or deserialize. Otherwise, non-CBOR byte streams
    /// could be created.
    pub(crate) fn from_indefinite_array(indefinite_array: &'a [Value<'a>]) -> Self {
        Self::with_inner(
            MajorType::Array(MinorType::Indefinite()),
            ValueInner::IndefiniteArray(indefinite_array),
        )
    }

    /// We do not expose this method because a user should use the values functions (like
    /// [u8] or [map]) to create values, or deserialize. Otherwise, non-CBOR byte streams
    /// could be created.
    pub(crate) fn from_indefinite_map(indefinite_map: &'a [(Value<'a>, Value<'a>)]) -> Self {
        Self::with_inner(
            MajorType::Map(MinorType::Indefinite()),
            ValueInner::IndefiniteMap(indefinite_map),
        )
    }

    /// We do not expose this method because a user should use the values functions (like
    /// [u8] or [map]) to create values, or deserialize. Otherwise, non-CBOR byte streams
    /// could be created.
    pub(crate) fn from_tag(tag: u64, inner: &'a Value<'a>) -> Self {
        Self::with_inner(
            MajorType::Tag(MinorType::size(tag as usize)),
            ValueInner::Tag(inner),
        )
    }

    fn with_inner(major: MajorType, inner: ValueInner<'a>) -> Self {
        Self { major, inner }
    }

    #[cfg(feature = "std")]
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
    pub fn is_empty(&self) -> bool { self.len() == 0 }
}

impl<'a> WriteTo for Value<'a> {
    fn len(&self) -> usize {
        self.major.len() + self.inner.len()
    }

    fn write_to<W: Write>(&self, w: &mut W) -> Result<usize, WriteError> {
        Ok(self.major.write_to(w)? + self.inner.write_to(w)?)
    }
}
