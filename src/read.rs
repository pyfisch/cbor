//! CBOR special reader.

use std::io::{self, Read};
use super::{Error, ErrorCode, Result};

/// Reader that tracks the current position of the undelying readable object in bytes.
pub struct PositionReader<R: Read> {
    inner: R,
    pos: usize,
}

impl <R: Read>PositionReader<R> {
    /// Constructs a new `PositionReader<R>`
    pub fn new(reader: R) -> PositionReader<R> {
        PositionReader {
            inner: reader,
            pos: 0,
        }
    }

    /// Gives the current position in bytes.
    ///
    /// The position is the number of bytes read since the creation of the `PositionReader`.
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Gets a reference to the underlying reader.
    pub fn get_ref(&self) -> &R {
        &self.inner
    }

    /// Gets a mutable reference to the underlying reader.
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.inner
    }

    /// Gets the underlying reader.
    pub fn into_inner(self) -> R {
        self.inner
    }

    /// Reads the given number of bytes from this Reader and returns a `Vec<u8>`.
    ///
    /// This copies code from RFC0980 the read_exact RFC. The code will be removed after
    /// `read_exact()` becomes stable.
    pub fn read_bytes(&mut self, n: usize) -> Result<Vec<u8>> {
        let mut buffer = vec![0; n];
        {
            let mut buf = &mut buffer[..];
            while !buf.is_empty() {
                match self.read(buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        let tmp = buf;
                        buf = &mut tmp[n..];
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
                    Err(e) => return Err(Error::IoError(e)),
                }
            }
            if !buf.is_empty() {
                return Err(Error::SyntaxError(ErrorCode::UnexpectedEOF, self.position()))
            }
        }
        Ok(buffer)
    }
}

impl <R: Read>Read for PositionReader<R> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.inner.read(buf) {
            Ok(n) => {
                self.pos += n;
                Ok(n)
            }
            Err(e) => Err(e),
        }
    }
}
