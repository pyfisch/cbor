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
pub use text::*;
#[cfg(test)]
mod text_test;

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
    pub(crate) inner: MajorType<'a>,
}

impl Value<'_> {
    /// Write the
    pub fn write<W: Write>(&self, w: &mut W) -> Result<(), WriteError> {
        self.inner.write(w)
    }

    #[cfg(feature = "std")]
    pub fn to_bytes(&self) -> Vec<u8> {
        // This skips the Write trait and just implement its own vector iterator.
        // The Write trait has error handling, and we really don't need that here,
        // so this is simpler.
        struct Writer<'a> {
            vector: &'a mut Vec<u8>,
        }
        impl Write for Writer<'_> {
            fn write(&mut self, bytes: &[u8]) -> Result<(), WriteError> {
                self.vector.extend_from_slice(bytes);
                Ok(())
            }
        }

        let mut vector = Vec::with_capacity(self.len());
        self.inner
            .write(&mut Writer {
                vector: &mut vector,
            })
            .expect("Unexpected error.");
        vector
    }

    /// The length in bytes of this value.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        // This is never empty (because MajorType will always have at least 1 byte.
        false
    }
}

impl<'a> From<MajorType<'a>> for Value<'a> {
    fn from(inner: MajorType<'a>) -> Self {
        Value { inner }
    }
}
