#[cfg(feature = "std")]
use std::io;

use crate::error;

pub trait Write {
    type Error: Into<error::Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error>;
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

#[cfg(feature = "std")]
pub struct StdWriter<'a, W>(&'a mut W);

#[cfg(feature = "std")]
impl<'a, W: io::Write> StdWriter<'a, W> {
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

// TODO this should be possible with just alloc
#[cfg(feature = "std")]
impl Write for Vec<u8> {
    type Error = io::Error;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Self::Error> {
        io::Write::write_all(self, buf)
    }
}
