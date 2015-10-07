//! CBOR deserialization.

use std::io::{self, Read};

use byteorder::{BigEndian, ReadBytesExt};
use serde::de::{self, Visitor, Deserialize};
use serde::bytes::ByteBuf;

use super::read::PositionReader;
use super::error::{Error, ErrorCode, Result};

/// A structure that deserializes CBOR into Rust values.
pub struct Deserializer<R: Read> {
    reader: PositionReader<R>,
}

impl <R: Read>Deserializer<R> {
    /// Creates the CBOR parser from an `std::io::Read`.
    #[inline]
    pub fn new(reader: R) -> Deserializer<R> {
        Deserializer { reader: PositionReader::new(reader) }
    }

    /// The `Deserializer::end` method should be called after a value has been fully deserialized.
    /// This allows the `Deserializer` to validate that the input stream is at the end.
    #[inline]
    pub fn end(&mut self) -> Result<()> {
        if try!(self.read(&mut [0; 1])) == 0 {
            Ok(())
        } else {
            Err(self.error(ErrorCode::TrailingBytes))
        }
    }

    #[inline]
    fn error(&mut self, reason: ErrorCode) -> Error {
        Error::SyntaxError(reason, self.reader.position())
    }

    #[inline]
    fn read_bytes(&mut self, n: usize) -> Result<Vec<u8>> {
        self.reader.read_bytes(n)
    }

    #[inline]
    fn parse_value<V: Visitor>(&mut self, visitor: V) -> Result<V::Value> {
        let first = try!(self.read_u8());
        match (first & 0b111_00000) >> 5 {
            0 => self.parse_uint(first, visitor),
            1 => self.parse_int(first, visitor),
            2 => self.parse_byte_buf(first, visitor),
            3 => self.parse_string(first, visitor),
            4 => self.parse_seq(first, visitor),
            5 => self.parse_map(first, visitor),
            6 => self.parse_tag(first, visitor),
            7 => self.parse_simple_value(first, visitor),
            _ => unreachable!(),
        }
    }

    #[inline]
    fn parse_additional_information(&mut self, first: u8) -> Result<Option<usize>> {
        Ok(Some(match first & 0b000_11111 {
            n @ 0...23 => n as usize,
            24 => try!(self.read_u8()) as usize,
            25 => try!(self.read_u16::<BigEndian>()) as usize,
            26 => try!(self.read_u32::<BigEndian>()) as usize,
            27 => try!(self.read_u64::<BigEndian>()) as usize,
            31 => return Ok(None),
            _ => return Err(self.error(ErrorCode::UnknownByte(first))),
        }))
    }

    #[inline]
    fn parse_uint<V: Visitor>(&mut self, first: u8, mut visitor: V) -> Result<V::Value> {
        match first & 0b000_11111 {
            n @ 0...23 => visitor.visit_u8(n),
            24 => visitor.visit_u8(try!(self.read_u8())),
            25 => visitor.visit_u16(try!(self.read_u16::<BigEndian>())),
            26 => visitor.visit_u32(try!(self.read_u32::<BigEndian>())),
            27 => visitor.visit_u64(try!(self.read_u64::<BigEndian>())),
            _ => Err(self.error(ErrorCode::UnknownByte(first))),
        }
    }

    #[inline]
    fn parse_int<V: Visitor>(&mut self, first: u8, mut visitor: V) -> Result<V::Value> {
        match first & 0b000_11111 {
            n @ 0...23 => visitor.visit_i8(-1 - n as i8),
            24 => visitor.visit_i16(-1 - try!(self.read_u8()) as i16),
            25 => visitor.visit_i32(-1 - try!(self.read_u16::<BigEndian>()) as i32),
            26 => visitor.visit_i64(-1 - try!(self.read_u32::<BigEndian>()) as i64),
            27 => visitor.visit_i64(-1 - try!(self.read_u64::<BigEndian>()) as i64),
            _ => Err(self.error(ErrorCode::UnknownByte(first))),
        }
    }

    #[inline]
    fn parse_byte_buf<V: Visitor>(&mut self, first: u8, mut visitor: V) -> Result<V::Value> {
        // Workaround as long as append is unstable.
        #[inline]
        fn append(this: &mut Vec<u8>, other: &[u8]) {
            for v in other {
                this.push(v.clone())
            }
        }
        if let Some(n) = try!(self.parse_additional_information(first)) {
            visitor.visit_byte_buf(try!(self.read_bytes(n)))
        } else {
            let mut bytes = Vec::new();
            loop {
                match ByteBuf::deserialize(self) {
                    Ok(value) => append(&mut bytes, &*value),
                    Err(Error::SyntaxError(ErrorCode::StopCode, _)) => break,
                    Err(e) => return Err(e),
                }
            }
            visitor.visit_byte_buf(bytes)
        }
    }

    #[inline]
    fn parse_string<V: Visitor>(&mut self, first: u8, mut visitor: V) -> Result<V::Value> {
        if let Some(n) = try!(self.parse_additional_information(first)) {
            visitor.visit_string(try!(String::from_utf8(try!(self.read_bytes(n)))))
        } else {
            let mut string = String::new();
            loop {
                match String::deserialize(self) {
                    Ok(value) => string.push_str(&value[..]),
                    Err(Error::SyntaxError(ErrorCode::StopCode, _)) => break,
                    Err(e) => return Err(e),
                }
            }
            return visitor.visit_string(string)
        }
    }

    #[inline]
    fn parse_seq<V: Visitor>(&mut self, first: u8, mut visitor: V) -> Result<V::Value> {
        let n = try!(self.parse_additional_information(first));
        visitor.visit_seq(SeqVisitor::new(self, n))
    }

    #[inline]
    fn parse_map<V: Visitor>(&mut self, first: u8, mut visitor: V) -> Result<V::Value> {
        let n = try!(self.parse_additional_information(first));
        visitor.visit_map(MapVisitor::new(self, n))
    }

    #[inline]
    fn parse_tag<V: Visitor>(&mut self, first: u8, visitor: V) -> Result<V::Value> {
        try!(self.parse_additional_information(first));
        self.parse_value(visitor)
    }

    #[inline]
    fn parse_simple_value<V: Visitor>(&mut self, first: u8, mut visitor: V) -> Result<V::Value> {
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
        #[inline]
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
        match first & 0b000_11111 {
            20 => visitor.visit_bool(false),
            21 => visitor.visit_bool(true),
            22 => visitor.visit_unit(),
            23 => visitor.visit_unit(),
            25 => visitor.visit_f32(decode_f16(try!(self.read_u16::<BigEndian>()))),
            26 => visitor.visit_f32(try!(self.read_f32::<BigEndian>())),
            27 => visitor.visit_f64(try!(self.read_f64::<BigEndian>())),
            31 => Err(self.error(ErrorCode::StopCode)),
            _ => Err(self.error(ErrorCode::UnknownByte(first))),
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

impl<R: Read> Read for Deserializer<R> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.reader.read(buf)
    }
}


struct SeqVisitor<'a, R: 'a + Read> {
    de: &'a mut Deserializer<R>,
    items: Option<usize>,
}

impl<'a, R: 'a + Read> SeqVisitor<'a, R> {
    #[inline]
    fn new(de: &'a mut Deserializer<R>, items: Option<usize>) -> Self {
        SeqVisitor {
            de: de,
            items: items,
        }
    }
}

impl<'a, R: Read> de::SeqVisitor for SeqVisitor<'a, R> {
    type Error = Error;

    #[inline]
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

    #[inline]
    fn end(&mut self) -> Result<()> {
        if let Some(0) = self.items {
            Ok(())
        } else {
            Err(self.de.error(ErrorCode::TrailingBytes))
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.items.map_or((0, None), |n| (n, Some(n)))
    }
}


struct MapVisitor<'a, R: 'a + Read> {
    de: &'a mut Deserializer<R>,
    items: Option<usize>,
}

impl<'a, R: Read> MapVisitor<'a, R> {
    #[inline]
    fn new(de: &'a mut Deserializer<R>, items: Option<usize>) -> Self {
        MapVisitor {
            de: de,
            items: items,
        }
    }
}

impl<'a, R: Read> de::MapVisitor for MapVisitor<'a, R> {
    type Error = Error;

    #[inline]
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

    #[inline]
    fn visit_value<V: Deserialize>(&mut self) -> Result<V> {
        Deserialize::deserialize(self.de)
    }

    #[inline]
    fn end(&mut self) -> Result<()> {
        if let Some(0) = self.items {
            Ok(())
        } else {
            Err(self.de.error(ErrorCode::TrailingBytes))
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.items.map_or((0, None), |n| (n, Some(n)))
    }
}

/// Decodes a CBOR value from a `std::io::Read`.
#[inline]
pub fn from_reader<T: Deserialize, R: Read>(reader: R) -> Result<T> {
    let mut de = Deserializer::new(reader);
    let value = Deserialize::deserialize(&mut de);
    try!(de.end());
    value
}

/// Decodes a CBOR value from a `&[u8]` slice.
#[inline]
pub fn from_slice<T: Deserialize>(v: &[u8]) -> Result<T> {
    from_reader(v)
}
