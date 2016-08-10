//! CBOR errors.
use std::error;
use std::error::Error as StdError;
use std::fmt;
use std::result;
use std::io;
use std::string::FromUtf8Error;

use serde::de;
use serde::ser;

/// Represents all possible errors that can occur when serializing or deserializing a value.
#[derive(Debug)]
pub enum Error {
    /// The CBOR value had a syntactic error.
    Syntax,
    /// Some IO error occured when processing a value.
    Io(io::Error),
    /// Some error occured while converting a string.
    FromUtf8(FromUtf8Error),
    /// A custom error provided by serde occured.
    Custom(String),
    /// The data source contains not enough bytes to parse a value.
    Eof,
    /// Break stop code encountered.
    StopCode,
    /// The data source contains trailing bytes after all values were read.
    TrailingBytes,
    #[doc(hidden)]
    __Nonexhaustive,
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Syntax => "syntax error",
            Error::Io(ref error) => StdError::description(error),
            Error::FromUtf8(ref error) => error.description(),
            Error::Custom(ref s) => s,
            Error::Eof => "unexpected end of file",
            Error::StopCode => "unexpected stop code",
            Error::TrailingBytes => "unexpected trailing bytes",
            Error::__Nonexhaustive => unreachable!(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref error) => Some(error),
            Error::FromUtf8(ref error) => Some(error),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Custom(ref s) => write!(f, "custom error: {}", s),
            _ => f.write_str(self.description()),
        }
    }
}


impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Error {
        Error::FromUtf8(error)
    }
}

impl de::Error for Error {
    fn custom<T: Into<String>>(msg: T) -> Error {
        Error::Custom(msg.into())
    }

    fn end_of_stream() -> Error {
        Error::Eof
    }
}

impl ser::Error for Error {
    fn custom<T: Into<String>>(msg: T) -> Error {
        Error::Custom(msg.into())
    }
}

/// Helper alias for Result objects that return a JSON Error.
pub type Result<T> = result::Result<T, Error>;
