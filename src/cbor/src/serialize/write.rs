use crate::error::Error;
use std::result::Result;

/// A simple error kind for [Write].
#[derive(Debug)]
pub enum WriteErrorKind {
    /// An entity was not found, often a file.
    NotFound,

    /// The operation lacked the necessary privileges to complete.
    PermissionDenied,

    /// The connection was refused by the remote server.
    ConnectionRefused,

    /// The connection was reset by the remote server.
    ConnectionReset,

    /// The connection was aborted (terminated) by the remote server.
    ConnectionAborted,

    /// The network operation failed because it was not connected yet.
    NotConnected,

    /// A socket address could not be bound because the address is already in
    /// use elsewhere.
    AddrInUse,

    /// A nonexistent interface was requested or the requested address was not
    /// local.
    AddrNotAvailable,

    /// The operation failed because a pipe was closed.
    BrokenPipe,

    /// An entity already exists, often a file.
    AlreadyExists,

    /// The operation needs to block to complete, but the blocking operation was
    /// requested to not occur.
    WouldBlock,

    /// A parameter was incorrect.
    InvalidInput,

    /// Data not valid for the operation were encountered.
    ///
    /// Unlike [`InvalidInput`], this typically means that the operation
    /// parameters were valid, however the error was caused by malformed
    /// input data.
    ///
    /// For example, a function that reads a file into a string will error with
    /// `InvalidData` if the file's contents are not valid UTF-8.
    ///
    /// [`InvalidInput`]: #variant.InvalidInput
    InvalidData,

    /// The I/O operation's timeout expired, causing it to be canceled.
    TimedOut,

    /// An error returned when an operation could not be completed because a
    /// call to [`write`] returned [`Ok(0)`].
    ///
    /// This typically means that an operation could only succeed if it wrote a
    /// particular number of bytes but only a smaller number of bytes could be
    /// written.
    WriteZero,

    /// This operation was interrupted.
    ///
    /// Interrupted operations can typically be retried.
    Interrupted,

    /// Any I/O error not part of this list.
    Other,

    /// An error returned when an operation could not be completed because an
    /// "end of file" was reached prematurely.
    ///
    /// This typically means that an operation could only succeed if it read a
    /// particular number of bytes but only a smaller number of bytes could be
    /// read.
    UnexpectedEof,
}

/// Replaces std::io::Error. An error happened during a write operation.
#[derive(Debug)]
pub enum WriteError {
    #[cfg(feature = "std")]
    IoError(std::io::Error),

    /// A simple error enumeration.
    Simple(WriteErrorKind),

    /// A custom error happened. This will box the error type from this crate.
    Custom(Box<dyn Error + Send + Sync>),

    /// A value that is reserved is being serialized.
    ReservedCborValue(),
}

#[cfg(feature = "std")]
impl From<std::io::Error> for WriteError {
    fn from(io_error: std::io::Error) -> Self {
        WriteError::IoError(io_error)
    }
}

/// A writer trait for byte oriented sinks, similar to [std::io::Write],
/// but no_std friendly.
///
/// This implements the minimum requirements for this crate, ie. a single
/// function that writes a series of bytes, and might return an error.
pub trait Write {
    /// Takes a series of bytes and output them. The result is the number
    /// of bytes taken, which normally should be the same as [`bytes.len()`].
    fn write(&mut self, bytes: &[u8]) -> Result<usize, WriteError>;
}

/// A serializer trait for types that write themselves to writers.
pub trait WriteTo {
    /// Get the length expected that would be writter to the writer. This is normally the
    /// the amount of bytes that would be returned by write_to().
    fn len(&self) -> usize;

    /// Write this type to a writer. The writer can be any type that implement the
    /// Write type from this crate.
    fn write_to<W: Write>(&self, _writer: &mut W) -> Result<usize, WriteError>;
}

/// A writer that outputs to a slice.
impl Write for &mut [u8] {
    #[inline]
    fn write(&mut self, bytes: &[u8]) -> Result<usize, WriteError> {
        let amt = std::cmp::min(bytes.len(), self.len());
        let (a, b) = std::mem::replace(self, &mut []).split_at_mut(amt);
        a.copy_from_slice(&bytes[..amt]);
        *self = b;
        Ok(amt)
    }
}

#[cfg(feature = "std")]
impl Write for Vec<u8> {
    fn write(&mut self, bytes: &[u8]) -> Result<usize, WriteError> {
        std::io::Write::write(self, bytes).map_err(WriteError::from)
    }
}
