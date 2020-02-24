#![allow(clippy::many_single_char_names)]
use crate::serialize::{Write, WriteError};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MinorType {
    /// A small integer that fits in the minor type, 0 to 23.
    SameByte(u8),

    /// An integer that takes up 1 byte after the major-type byte.
    OneByte(u8),

    /// An integer that takes up to 2 bytes after major-type byte.
    TwoBytes(u8, u8),

    /// An integer that takes up to 4 bytes after major-type byte.
    FourBytes(u8, u8, u8, u8),

    /// An integer that takes up to 8 bytes after major-type byte.
    EightBytes(u8, u8, u8, u8, u8, u8, u8, u8),

    /// Indication for certain Major Types that the size is unbounded and
    /// the encoder/decoder should use a "break" token.
    Indefinite(),
}

impl MinorType {
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the minor type of these bytes.
    pub fn minor(&self) -> u8 {
        match self {
            MinorType::SameByte(a) => *a,
            MinorType::OneByte(_) => 24,
            MinorType::TwoBytes(_, _) => 25,
            MinorType::FourBytes(_, _, _, _) => 26,
            MinorType::EightBytes(_, _, _, _, _, _, _, _) => 27,
            MinorType::Indefinite() => 29,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            MinorType::SameByte(_) => 0,
            MinorType::OneByte(_) => 1,
            MinorType::TwoBytes(_, _) => 2,
            MinorType::FourBytes(_, _, _, _) => 4,
            MinorType::EightBytes(_, _, _, _, _, _, _, _) => 8,
            MinorType::Indefinite() => 0,
        }
    }

    pub fn write<W: Write>(&self, w: &mut W) -> Result<(), WriteError> {
        match *self {
            MinorType::SameByte(_) => Ok(()),
            MinorType::OneByte(a) => w.write(&[a]),
            MinorType::TwoBytes(a, b) => w.write(&[a, b]),
            MinorType::FourBytes(a, b, c, d) => w.write(&[a, b, c, d]),
            MinorType::EightBytes(a, b, c, d, e, f, g, h) => w.write(&[a, b, c, d, e, f, g, h]),
            MinorType::Indefinite() => Ok(()),
        }
    }
}
