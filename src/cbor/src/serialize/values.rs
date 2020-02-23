use crate::encoding::major_type::Token;
use crate::serialize::{Write, WriteError};

// We re-export everything in this namespace. We only use multiple files for simplification
// of the code.
mod numbers;
pub use numbers::*;

/// A CBOR Value. Can represent any CBOR value possible.
///
/// The lifetime is used for bytes and string references. This Value does not own any sliced
/// data itself. For this, use an [OwnedValue] (which is incompatible with no_std).
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Value<'a> {
    pub(crate) inner: Token<'a>,
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
}

impl<'a> From<Token<'a>> for Value<'a> {
    fn from(v: Token<'a>) -> Self {
        Value { inner: v }
    }
}
