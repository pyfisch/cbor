//! CBOR errors.
use std::error;
use std::fmt;
use std::result;
use std::io;
use std::string::FromUtf8Error;

use serde::de;
use byteorder::Error as ByteorderError;

/// The errors that can arise while parsing a CBOR stream.
#[derive(Clone, PartialEq)]
pub enum ErrorCode {
    /// The data source contains trailing bytes after all values were read.
    TrailingBytes,
    /// The data source contains not enough bytes to parse a value.
    UnexpectedEOF,
    /// Break stop code encountered.
    StopCode,
    /// Invalid Byte at the beginning of a new value detected.
    UnknownByte(u8),
    /// Unknown field in struct.
    UnknownField(String),
    /// Struct is missing a field.
    MissingField(&'static str),
    /// General error required by serde.
    InvalidSyntax(String),
}

impl fmt::Debug for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::fmt::Debug;

        match *self {
            ErrorCode::UnexpectedEOF => "EOF while parsing a value".fmt(f),
            ErrorCode::StopCode => "break stop code outside indefinite length item".fmt(f),
            ErrorCode::UnknownField(ref field) => write!(f, "unknown field \"{}\"", field),
            ErrorCode::MissingField(ref field) => write!(f, "missing field \"{}\"", field),
            ErrorCode::TrailingBytes => "trailing bytes".fmt(f),
            ErrorCode::UnknownByte(byte) => write!(f, "unknown start byte b'\\x{0:x}'", byte),
            ErrorCode::InvalidSyntax(ref msg) => write!(f, "invalid syntax: \"{}\"", msg),
        }
    }
}

/// Represents all possible errors that can occur when serializing or deserializing a value.
#[derive(Debug)]
pub enum Error {
    /// The CBOR value had a syntactic error.
    SyntaxError(ErrorCode, usize),
    /// Some IO error occured when processing a value.
    IoError(io::Error),
    /// Some error occured while converting a string.
    FromUtf8Error(FromUtf8Error),
    #[doc(hidden)]
    __Nonexhaustive,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::SyntaxError(..) => "syntax error",
            Error::IoError(ref error) => error::Error::description(error),
            Error::FromUtf8Error(ref error) => error.description(),
            _ => "unknown error"
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IoError(ref error) => Some(error),
            Error::FromUtf8Error(ref error) => Some(error),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::SyntaxError(ref code, pos) => {
                write!(fmt, "{:?} at byte position {}", code, pos)
            }
            Error::IoError(ref error) => fmt::Display::fmt(error, fmt),
            Error::FromUtf8Error(ref error) => fmt::Display::fmt(error, fmt),
            _ => fmt.write_str("unknown error")
        }
    }
}


impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::IoError(error)
    }
}

impl From<FromUtf8Error> for Error {
    fn from(error: FromUtf8Error) -> Error {
        Error::FromUtf8Error(error)
    }
}

impl From<ByteorderError> for Error {
    fn from(error: ByteorderError) -> Error {
        match error {
            ByteorderError::UnexpectedEOF => Error::SyntaxError(ErrorCode::UnexpectedEOF, 0),
            ByteorderError::Io(e) => Error::IoError(e),
        }
    }
}

impl de::Error for Error {
    fn syntax(s: &str) -> Error {
        Error::SyntaxError(ErrorCode::InvalidSyntax(s.to_owned()), 0)
    }

    fn end_of_stream() -> Error {
        Error::SyntaxError(ErrorCode::UnexpectedEOF, 0)
    }

    fn unknown_field(field: &str) -> Error {
        Error::SyntaxError(ErrorCode::UnknownField(String::from(field)), 0)
    }

    fn missing_field(field: &'static str) -> Error {
        Error::SyntaxError(ErrorCode::MissingField(field), 0)
    }
}

/// Helper alias for Result objects that return a JSON Error.
pub type Result<T> = result::Result<T, Error>;
