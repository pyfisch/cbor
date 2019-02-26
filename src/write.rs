#[cfg(feature = "std")]
use std::io;

use crate::error;

#[cfg(not(feature = "unsealed_read_write"))]
/// A sink for serialized CBOR.
///
/// This trait is similar to the [`Write`]() trait in the standard library,
/// but has a smaller and more general API.
///
/// Any object implementing `std::io::Write`
/// can be wrapped in an [`StdWriter`](../write/struct.StdWriter.html) that implements
/// this trait for the underlying object.
pub trait Write: private::Sealed {
    /// The type of error returned when a write operation fails.
    #[doc(hidden)]
    type Error: Into<error::Error>;

    /// Attempts to write an entire buffer into this write.
    #[doc(hidden)]
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error>;
}

#[cfg(feature = "unsealed_read_write")]
/// A sink for serialized CBOR.
///
/// This trait is similar to the [`Write`]() trait in the standard library,
/// but has a smaller and more general API.
///
/// Any object implementing `std::io::Write`
/// can be wrapped in an [`StdWriter`](../write/struct.StdWriter.html) that implements
/// this trait for the underlying object.
///
/// This trait is sealed by default, enabling the `unsealed_read_write` feature removes this bound
/// to allow objects outside of this crate to implement this trait.
pub trait Write {
    /// The type of error returned when a write operation fails.
    type Error: Into<error::Error>;

    /// Attempts to write an entire buffer into this write.
    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error>;
}

#[cfg(not(feature = "unsealed_read_write"))]
mod private {
    pub trait Sealed {}
}

impl<W> Write for &mut W
where
    W: Write,
{
    type Error = W::Error;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        (*self).write_all(buf)
    }
}

#[cfg(not(feature = "unsealed_read_write"))]
impl<W> private::Sealed for &mut W where W: Write {}

#[cfg(feature = "std")]
/// A wrapper for types that implement
/// [`std::io::Write`](https://doc.rust-lang.org/std/io/trait.Write.html) to implement the local
/// [`Write`](trait.Write.html) trait.
pub struct StdWriter<'a, W>(&'a mut W);

#[cfg(feature = "std")]
impl<'a, W: io::Write> StdWriter<'a, W> {
    /// Wraps an `io::Write` writer to make it compatible with [`Write`](trait.Write.html)
    pub fn new(w: &'a mut W) -> StdWriter<'a, W> {
        StdWriter(w)
    }
}

#[cfg(feature = "std")]
impl<'a, W: io::Write> Write for StdWriter<'a, W> {
    type Error = io::Error;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        self.0.write_all(buf)
    }
}

#[cfg(all(feature = "std", not(feature = "unsealed_read_write")))]
impl<'a, W> private::Sealed for StdWriter<'a, W> where W: io::Write {}

// TODO this should be possible with just alloc
#[cfg(feature = "std")]
impl Write for Vec<u8> {
    type Error = io::Error;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        io::Write::write_all(self, buf)
    }
}

#[cfg(not(feature = "unsealed_read_write"))]
impl private::Sealed for Vec<u8> {}
