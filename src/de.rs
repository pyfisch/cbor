use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};
use serde::de::{self, Visitor, Deserialize};
use serde::bytes::ByteBuf;

use super::read::PositionReader;
use super::error::{Error, ErrorCode, Result};

pub struct Deserializer<R: Read> {
    reader: PositionReader<R>,
}

impl <R: Read>Deserializer<R> {
    pub fn new(reader: R) -> Deserializer<R> {
        Deserializer { reader: PositionReader::new(reader) }
    }

    fn read_bytes(&mut self, n: usize) -> Result<Vec<u8>> {
        self.reader.read_bytes(n)
    }

    fn error(&mut self, reason: ErrorCode) -> Error {
        Error::SyntaxError(reason, self.reader.position())
    }

    pub fn parse_value<V: Visitor>(&mut self, mut visitor: V) -> Result<V::Value> {
        // Workaround to not require the currently unstable `f32::ldexp`:
        mod ffi {
            use libc::c_int;

            extern {
                pub fn ldexpf(x: f32, exp: c_int) -> f32;
            }

            #[inline]
            pub fn c_ldexpf(x: f32, exp: isize) -> f32 {
                unsafe { ldexpf(x, exp as c_int) }
            }
        }
        fn decode_f16(half: u16) -> f32 {
            let exp: u16 = half >> 10 & 0x1f;
            let mant: u16 = half & 0x3ff;
            let val: f32 = if exp == 0 {
                ffi::c_ldexpf(mant as f32, -24)
            } else if exp != 31 {
                ffi::c_ldexpf(mant as f32 + 1024f32, exp as isize - 25)
            } else {
                if mant == 0 {
                    ::std::f32::INFINITY
                } else {
                    ::std::f32::NAN
                }
            };
            if half & 0x8000 != 0 {
                -val
            } else {
                val
            }
        }
        match try!(self.reader.read_u8()) {
            // Unsigned integers
            b @ 0x00...0x17 => visitor.visit_u8(b & 0b00011111),
            0x18 => visitor.visit_u8(try!(self.reader.read_u8())),
            0x19 => visitor.visit_u16(try!(self.reader.read_u16::<BigEndian>())),
            0x1a => visitor.visit_u32(try!(self.reader.read_u32::<BigEndian>())),
            0x1b => visitor.visit_u64(try!(self.reader.read_u64::<BigEndian>())),
            // Signed integers
            b @ 0x20...0x37 => visitor.visit_i8(-1 - (b & 0b00011111) as i8),
            0x38 => visitor.visit_i16(-1 - try!(self.reader.read_u8()) as i16),
            0x39 => visitor.visit_i32(-1 - try!(self.reader.read_u16::<BigEndian>()) as i32),
            0x3a => visitor.visit_i64(-1 - try!(self.reader.read_u32::<BigEndian>()) as i64),
            0x3b => visitor.visit_i64(-1 - try!(self.reader.read_u64::<BigEndian>()) as i64),
            // Byte strings
            b @ 0x40...0x57 => visitor.visit_byte_buf(
                try!(self.read_bytes((b & 0b00011111) as usize))),
            0x58 => {
                let n = try!(self.reader.read_u8()) as usize;
                visitor.visit_byte_buf(try!(self.read_bytes(n)))
            }
            0x59 => {
                let n = try!(self.reader.read_u16::<BigEndian>()) as usize;
                visitor.visit_byte_buf(try!(self.read_bytes(n)))
            }
            0x5a => {
                let n = try!(self.reader.read_u32::<BigEndian>()) as usize;
                visitor.visit_byte_buf(try!(self.read_bytes(n)))
            }
            0x5b => {
                let n = try!(self.reader.read_u64::<BigEndian>()) as usize;
                visitor.visit_byte_buf(try!(self.read_bytes(n)))
            }
            0x5f => {
                let mut bytes = Vec::new();
                loop {
                    match ByteBuf::deserialize(self) {
                        Ok(value) => bytes.append(&mut value.to_vec()),
                        Err(Error::SyntaxError(ErrorCode::StopCode, _)) => break,
                        Err(e) => return Err(e),
                    }
                }
                visitor.visit_byte_buf(bytes)
            }
            // UTF-8 strings
            b @ 0x60...0x77 => visitor.visit_string(
                try!(String::from_utf8(try!(self.read_bytes((b & 0b00011111) as usize))))),
            0x78 => {
                let n = try!(self.reader.read_u8()) as usize;
                visitor.visit_string(try!(String::from_utf8(try!(self.read_bytes(n)))))
            }
            0x79 => {
                let n = try!(self.reader.read_u16::<BigEndian>()) as usize;
                visitor.visit_string(try!(String::from_utf8(try!(self.read_bytes(n)))))
            }
            0x7a => {
                let n = try!(self.reader.read_u32::<BigEndian>()) as usize;
                visitor.visit_string(try!(String::from_utf8(try!(self.read_bytes(n)))))
            }
            0x7b => {
                let n = try!(self.reader.read_u64::<BigEndian>()) as usize;
                visitor.visit_string(try!(String::from_utf8(try!(self.read_bytes(n)))))
            }
            0x7f => {
                let mut string = String::new();
                loop {
                    match String::deserialize(self) {
                        Ok(value) => string.push_str(&value[..]),
                        Err(Error::SyntaxError(ErrorCode::StopCode, _)) => break,
                        Err(e) => return Err(e),
                    }
                }
                visitor.visit_string(string)
            }
            // Arrays
            b @ 0x80...0x97 => visitor.visit_seq(
                SeqVisitor::new(self, Some((b & 0b00011111) as usize))),
            0x98 => {
                let n = try!(self.reader.read_u8()) as usize;
                visitor.visit_seq(SeqVisitor::new(self, Some(n)))
            }
            0x99 => {
                let n = try!(self.reader.read_u16::<BigEndian>()) as usize;
                visitor.visit_seq(SeqVisitor::new(self, Some(n)))
            }
            0x9a => {
                let n = try!(self.reader.read_u32::<BigEndian>()) as usize;
                visitor.visit_seq(SeqVisitor::new(self, Some(n)))
            }
            0x9b => {
                let n = try!(self.reader.read_u64::<BigEndian>()) as usize;
                visitor.visit_seq(SeqVisitor::new(self, Some(n)))
            }
            0x9f => visitor.visit_seq(SeqVisitor::new(self, None)),
            // Maps
            b @ 0xa0...0xb7 => visitor.visit_map(
                MapVisitor::new(self, Some((b & 0b00011111) as usize))),
            0xb8 => {
                let n = try!(self.reader.read_u8()) as usize;
                visitor.visit_map(MapVisitor::new(self, Some(n)))
            }
            0xb9 => {
                let n = try!(self.reader.read_u16::<BigEndian>()) as usize;
                visitor.visit_map(MapVisitor::new(self, Some(n)))
            }
            0xba => {
                let n = try!(self.reader.read_u32::<BigEndian>()) as usize;
                visitor.visit_map(MapVisitor::new(self, Some(n)))
            }
            0xbb => {
                let n = try!(self.reader.read_u64::<BigEndian>()) as usize;
                visitor.visit_map(MapVisitor::new(self, Some(n)))
            }
            0xbf => visitor.visit_map(MapVisitor::new(self, None)),
            // Tagged items (tags get ignored)
            0xc0 ... 0xd7 => self.parse_value(visitor),
            0xd8 => {
                try!(self.reader.read_u8());
                self.parse_value(visitor)
            }
            0xd9 => {
                try!(self.reader.read_u16::<BigEndian>());
                self.parse_value(visitor)
            }
            0xda => {
                try!(self.reader.read_u32::<BigEndian>());
                self.parse_value(visitor)
            }
            0xdb => {
                try!(self.reader.read_u64::<BigEndian>());
                self.parse_value(visitor)
            }
            // 0xe0...0xf3 => unimplemented!(), // (simple value)
            // Boolean, Null, Undefined
            0xf4 => visitor.visit_bool(false),
            0xf5 => visitor.visit_bool(true),
            0xf6 => visitor.visit_unit(),
            0xf7 => visitor.visit_unit(),
            // 0xf8 => unimplemented!(), // (simple value, one byte follows)
            // Floats
            0xf9 => visitor.visit_f32(decode_f16(try!(self.reader.read_u16::<BigEndian>()))),
            0xfa => visitor.visit_f32(try!(self.reader.read_f32::<BigEndian>())),
            0xfb => visitor.visit_f64(try!(self.reader.read_f64::<BigEndian>())),
            0xff => Err(self.error(ErrorCode::StopCode)),
            n => Err(self.error(ErrorCode::UnknownByte(n))),
        }
    }
}

impl<R: Read> de::Deserializer for Deserializer<R> {
    type Error = Error;

    #[inline]
    fn visit<V: Visitor>(&mut self, visitor: V) -> Result<V::Value> {
        self.parse_value(visitor)
    }

    #[inline]
    fn format() -> &'static str {
        "cbor"
    }
}


struct SeqVisitor<'a, R: 'a + Read> {
    de: &'a mut Deserializer<R>,
    items: Option<usize>,
}

impl<'a, R: 'a + Read> SeqVisitor<'a, R> {
    fn new(de: &'a mut Deserializer<R>, items: Option<usize>) -> Self {
        SeqVisitor { de: de, items: items }
    }
}

impl<'a, R: Read> de::SeqVisitor for SeqVisitor<'a, R> {
    type Error = Error;

    fn visit<T: Deserialize>(&mut self) -> Result<Option<T>> {
        match self.items {
            Some(0) => return Ok(None),
            Some(ref mut n) => *n -= 1,
            _ => {}
        };
        match Deserialize::deserialize(self.de) {
            Ok(value) => Ok(Some(value)),
            Err(Error::SyntaxError(ErrorCode::StopCode, _)) => {
                self.items = Some(0);
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }

    fn end(&mut self) -> Result<()> {
        if let Some(0) = self.items {
            Ok(())
        } else {
            Err(self.de.error(ErrorCode::TrailingBytes))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.items.map_or((0, None), |n| (n, Some(n)))
    }
}


struct MapVisitor<'a, R: 'a + Read> {
    de: &'a mut Deserializer<R>,
    items: Option<usize>,
}

impl<'a, R: Read> MapVisitor<'a, R> {
    fn new(de: &'a mut Deserializer<R>, items: Option<usize>) -> Self {
        MapVisitor { de: de, items: items }
    }
}

impl<'a, R: Read> de::MapVisitor for MapVisitor<'a, R> {
    type Error = Error;

    fn visit_key<K: Deserialize>(&mut self) -> Result<Option<K>> {
        match self.items {
            Some(0) => return Ok(None),
            Some(ref mut n) => *n -= 1,
            _ => {}
        };
        match Deserialize::deserialize(self.de) {
            Ok(value) => Ok(Some(value)),
            Err(Error::SyntaxError(ErrorCode::StopCode, _)) => {
                self.items = Some(0);
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }

    fn visit_value<V: Deserialize>(&mut self) -> Result<V> {
        Deserialize::deserialize(self.de)
    }

    fn end(&mut self) -> Result<()> {
        if let Some(0) = self.items {
            Ok(())
        } else {
            Err(self.de.error(ErrorCode::TrailingBytes))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.items.map_or((0, None), |n| (n, Some(n)))
    }
}

pub fn from_reader<T: Deserialize, R: Read>(reader: R) -> Result<T> {
    Deserialize::deserialize(&mut Deserializer::new(reader))
}

pub fn from_slice<T: Deserialize>(v: &[u8]) -> Result<T> {
    Deserialize::deserialize(&mut Deserializer::new(v))
}
