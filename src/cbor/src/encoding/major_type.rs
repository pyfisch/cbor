use crate::serialize::{Write, WriteError};

#[derive(Copy, Clone, PartialEq)]
pub enum Bytes {
    SameByte(u8),
    OneByte(u8),
    TwoBytes(u8, u8),
    FourBytes(u8, u8, u8, u8),
    EightBytes(u8, u8, u8, u8, u8, u8, u8, u8),
}

impl Bytes {
    /// Returns the minor type of these bytes.
    pub fn minor(&self) -> u8 {
        match self {
            Bytes::SameByte(a) => *a,
            Bytes::OneByte(_) => 24,
            Bytes::TwoBytes(_, _) => 25,
            Bytes::FourBytes(_, _, _, _) => 26,
            Bytes::EightBytes(_, _, _, _, _, _, _, _) => 27,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Bytes::SameByte(_) => 0,
            Bytes::OneByte(_) => 1,
            Bytes::TwoBytes(_, _) => 2,
            Bytes::FourBytes(_, _, _, _) => 4,
            Bytes::EightBytes(_, _, _, _, _, _, _, _) => 8,
        }
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), WriteError> {
        match *self {
            Bytes::SameByte(_) => Ok(()),
            Bytes::OneByte(a) => w.write(&[a]),
            Bytes::TwoBytes(a, b) => w.write(&[a, b]),
            Bytes::FourBytes(a, b, c, d) => w.write(&[a, b, c, d]),
            Bytes::EightBytes(a, b, c, d, e, f, g, h) => w.write(&[a, b, c, d, e, f, g, h]),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Token<'a> {
    UnsignedInteger(Bytes),
    NegativeInteger(Bytes),
    // Negative(u64, Encoding),
    //    /// Major type 2: a byte string.
    Bytes(&'a [u8]),
    //    /// Major type 3: a text string.
    //    Text(&'a str, Encoding),
    //    Tag(u64, Token<'a>),
}

impl Token<'_> {
    pub fn write<W: Write>(&self, w: &mut W) -> Result<(), WriteError> {
        match self {
            Token::UnsignedInteger(ref bytes) => {
                w.write(&[bytes.minor()])?;
                bytes.write(w)
            }
            _ => Ok(()),
        }
    }
}
