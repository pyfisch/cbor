use std::error;
use std::fmt;
use std::result;
use std::io;
use std::string::FromUtf8Error;

use serde::de;
use byteorder::Error as ByteorderError;


#[derive(Clone, PartialEq)]
pub enum ErrorCode {
    TrailingBytes,
    ExpectedSomeValue,
    EOFWhileParsingValue,
    // Break stop code outside indefinite length item
    StopCode,
    UnknownByte(u8),
    /// Unknown field in struct.
    UnknownField(String),
    /// Struct is missing a field.
    MissingField(&'static str),
    InvalidSyntax(String),
}

impl fmt::Debug for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::fmt::Debug;

        match *self {
            ErrorCode::EOFWhileParsingValue => "EOF while parsing a value".fmt(f),
            ErrorCode::ExpectedSomeValue => "expected value".fmt(f),
            ErrorCode::StopCode => "break stop code outside indefinite length item".fmt(f),
            ErrorCode::UnknownField(ref field) => write!(f, "unknown field \"{}\"", field),
            ErrorCode::MissingField(ref field) => write!(f, "missing field \"{}\"", field),
            ErrorCode::TrailingBytes => "trailing bytes".fmt(f),
            ErrorCode::UnknownByte(byte) => write!(f, "unknown start byte b'\\x{0:x}'", byte),
            ErrorCode::InvalidSyntax(ref msg) => write!(f, "invalid syntax: \"{}\"", msg),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    SyntaxError(ErrorCode, usize),
    IoError(io::Error),
    FromUtf8Error(FromUtf8Error),
    UnexpectedEOF,
    #[doc(hidden)]
    __Nonexhaustive,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::SyntaxError(..) => "syntax error",
            Error::IoError(ref error) => error::Error::description(error),
            Error::FromUtf8Error(ref error) => error.description(),
            Error::UnexpectedEOF => "failed to fill whole buffer",
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
            ByteorderError::UnexpectedEOF => Error::SyntaxError(ErrorCode::EOFWhileParsingValue, 0),
            ByteorderError::Io(e) => Error::IoError(e),
        }
    }
}

impl de::Error for Error {
    fn syntax(s: &str) -> Error {
        Error::SyntaxError(ErrorCode::InvalidSyntax(s.to_owned()), 0)
    }

    fn end_of_stream() -> Error {
        Error::SyntaxError(ErrorCode::EOFWhileParsingValue, 0)
    }

    fn unknown_field(field: &str) -> Error {
        Error::SyntaxError(ErrorCode::UnknownField(String::from(field)), 0)
    }

    fn missing_field(field: &'static str) -> Error {
        Error::SyntaxError(ErrorCode::MissingField(field), 0)
    }
}

pub type Result<T> = result::Result<T, Error>;
