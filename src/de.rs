//! CBOR deserialization.

use std::io::{self, Read};

use byteorder::{BigEndian, ReadBytesExt};
use serde::de::{self, Visitor, Deserialize, DeserializeSeed};
use serde::bytes::ByteBuf;

use super::error::{Error, Result};

const MAX_SEQ_LEN: u64 = 524288;

macro_rules! forward_deserialize {
    ($($name:ident($($arg:ident: $ty:ty,)*);)*) => {
        $(#[inline]
        fn $name<V: Visitor>(self, $($arg: $ty,)* visitor: V) -> Result<V::Value> {
            self.deserialize(visitor)
        })*
    }
}

/// A structure that deserializes CBOR into Rust values.
pub struct Deserializer<R: Read> {
    reader: R,
    first: Option<u8>,
    struct_fields: &'static [&'static str],
}

impl<R: Read> Deserializer<R> {
    /// Creates the CBOR parser from an `std::io::Read`.
    #[inline]
    pub fn new(reader: R) -> Deserializer<R> {
        Deserializer {
            reader: reader,
            first: None,
            struct_fields: &[],
        }
    }

    /// The `Deserializer::end` method should be called after a value has been fully deserialized.
    /// This allows the `Deserializer` to validate that the input stream is at the end.
    #[inline]
    pub fn end(&mut self) -> Result<()> {
        if self.read(&mut [0; 1])? == 0 {
            Ok(())
        } else {
            Err(Error::TrailingBytes)
        }
    }

    #[inline]
    fn parse_value<V: Visitor>(&mut self, visitor: V) -> Result<V::Value> {
        let first = self.first.unwrap();
        self.first = None;
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
    fn parse_additional_information(&mut self, first: u8) -> Result<Option<u64>> {
        Ok(Some(match first & 0b000_11111 {
            n @ 0...23 => n as u64,
            24 => self.read_u8()? as u64,
            25 => self.read_u16::<BigEndian>()? as u64,
            26 => self.read_u32::<BigEndian>()? as u64,
            27 => self.read_u64::<BigEndian>()? as u64,
            31 => return Ok(None),
            _ => return Err(Error::Syntax),
        }))
    }

    fn parse_size_information(&mut self, first: u8) -> Result<Option<usize>> {
        let n = self.parse_additional_information(first)?;
        match n {
            Some(n) if n > MAX_SEQ_LEN => return Err(Error::Syntax),
            _ => (),
        }
        Ok(n.map(|x| x as usize))
    }

    #[inline]
    fn parse_uint<V: Visitor>(&mut self, first: u8, visitor: V) -> Result<V::Value> {
        match first & 0b000_11111 {
            n @ 0...23 => visitor.visit_u8(n),
            24 => visitor.visit_u8(self.read_u8()?),
            25 => visitor.visit_u16(self.read_u16::<BigEndian>()?),
            26 => visitor.visit_u32(self.read_u32::<BigEndian>()?),
            27 => visitor.visit_u64(self.read_u64::<BigEndian>()?),
            _ => Err(Error::Syntax),
        }
    }

    #[inline]
    fn parse_int<V: Visitor>(&mut self, first: u8, visitor: V) -> Result<V::Value> {
        match first & 0b000_11111 {
            n @ 0...23 => visitor.visit_i8(-1 - n as i8),
            24 => visitor.visit_i16(-1 - self.read_u8()? as i16),
            25 => visitor.visit_i32(-1 - self.read_u16::<BigEndian>()? as i32),
            26 => visitor.visit_i64(-1 - self.read_u32::<BigEndian>()? as i64),
            27 => visitor.visit_i64(-1 - self.read_u64::<BigEndian>()? as i64),
            _ => Err(Error::Syntax),
        }
    }

    #[inline]
    fn parse_byte_buf<V: Visitor>(&mut self, first: u8, visitor: V) -> Result<V::Value> {
        if let Some(n) = self.parse_size_information(first)? {
            let mut buf = vec![0; n];
            self.reader.read_exact(&mut buf)?;
            visitor.visit_byte_buf(buf)
        } else {
            let mut bytes = Vec::new();
            loop {
                match ByteBuf::deserialize(&mut *self) {
                    Ok(value) => bytes.append(&mut value.into()),
                    Err(Error::StopCode) => break,
                    Err(e) => return Err(e),
                }
            }
            visitor.visit_byte_buf(bytes)
        }
    }

    #[inline]
    fn parse_string<V: Visitor>(&mut self, first: u8, visitor: V) -> Result<V::Value> {
        if let Some(n) = self.parse_size_information(first)? {
            let mut buf = vec![0; n];
            self.reader.read_exact(&mut buf)?;
            visitor.visit_string(String::from_utf8(buf)?)
        } else {
            let mut string = String::new();
            loop {
                match String::deserialize(&mut *self) {
                    Ok(value) => string.push_str(&value[..]),
                    Err(Error::StopCode) => break,
                    Err(e) => return Err(e),
                }
            }
            return visitor.visit_string(string);
        }
    }

    #[inline]
    fn parse_seq<V: Visitor>(&mut self, first: u8, visitor: V) -> Result<V::Value> {
        let n = self.parse_size_information(first)?;
        visitor.visit_seq(CompositeVisitor::new(self, n.map(|x| x as usize)))
    }

    #[inline]
    fn parse_map<V: Visitor>(&mut self, first: u8, visitor: V) -> Result<V::Value> {
        let n = self.parse_size_information(first)?;
        visitor.visit_map(CompositeVisitor::new(self, n.map(|x| x as usize)))
    }

    #[inline]
    fn parse_tag<V: Visitor>(&mut self, first: u8, visitor: V) -> Result<V::Value> {
        self.parse_additional_information(first)?;
        self.first = Some(self.read_u8()?);
        self.parse_value(visitor)
    }

    #[inline]
    fn parse_simple_value<V: Visitor>(&mut self, first: u8, visitor: V) -> Result<V::Value> {
        #[inline]
        fn decode_f16(half: u16) -> f32 {
            let exp: u16 = half >> 10 & 0x1f;
            let mant: u16 = half & 0x3ff;
            let val: f32 = if exp == 0 {
                (mant as f32) * (2.0f32).powi(-24)
            } else if exp != 31 {
                (mant as f32 + 1024f32) * (2.0f32).powi(exp as i32 - 25)
            } else if mant == 0 {
                ::std::f32::INFINITY
            } else {
                ::std::f32::NAN
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
            25 => visitor.visit_f32(decode_f16(self.read_u16::<BigEndian>()?)),
            26 => visitor.visit_f32(self.read_f32::<BigEndian>()?),
            27 => visitor.visit_f64(self.read_f64::<BigEndian>()?),
            31 => Err(Error::StopCode),
            _ => Err(Error::Syntax),
        }
    }
}

impl<'a, R: Read> de::Deserializer for &'a mut Deserializer<R> {
    type Error = Error;
    forward_deserialize!(
        deserialize_bool();
        deserialize_i8();
        deserialize_i16();
        deserialize_i32();
        deserialize_i64();
        deserialize_u8();
        deserialize_u16();
        deserialize_u32();
        deserialize_u64();
        deserialize_f32();
        deserialize_f64();
        deserialize_char();
        deserialize_str();
        deserialize_string();
        deserialize_unit();
        deserialize_seq();
        deserialize_seq_fixed_size(_len: usize,);
        deserialize_bytes();
        deserialize_byte_buf();
        deserialize_map();
        deserialize_unit_struct(_name: &'static str,);
        deserialize_tuple_struct(_name: &'static str, _len: usize,);
        deserialize_tuple(_len: usize,);
        deserialize_ignored_any();
    );
    #[inline]
    fn deserialize<V: Visitor>(self, visitor: V) -> Result<V::Value> {
        if self.first.is_none() {
            self.first = Some(self.read_u8()?);
        }
        let result = self.parse_value(visitor);
        self.first = None;
        result
    }

    #[inline]
    fn deserialize_option<V: Visitor>(self, visitor: V) -> Result<V::Value> {
        self.first = Some(self.read_u8()?);
        if self.first == Some(0b111_10110) {
            self.first = None;
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }
    #[inline]
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
        where V: de::Visitor
    {
        visitor.visit_newtype_struct(self)
    }

    #[inline]
    fn deserialize_struct<V>(self,
                             _name: &'static str,
                             fields: &'static [&'static str],
                             visitor: V)
                             -> Result<V::Value>
        where V: de::Visitor
    {
        self.struct_fields = fields;
        self.deserialize(visitor)
    }

    #[inline]
    fn deserialize_struct_field<V>(self, visitor: V) -> Result<V::Value>
        where V: de::Visitor
    {
        // Reads a struct field name. A name is either a string
        // or an index to a field. Indices are converted to strings.
        if self.first.is_none() {
            self.first = Some(self.read_u8()?);
        }
        match (self.first.unwrap() & 0b111_00000) >> 5 {
            // integer index for field
            0 => {
                let first = self.first.unwrap();
                self.first = None;
                let index = self.parse_additional_information(first)?
                    .ok_or(Error::Syntax)? as usize;
                visitor.visit_string(self.struct_fields
                    .get(index)
                    .ok_or(Error::Syntax)?
                    .to_string())
            }
            // string field name
            3 => self.deserialize_string(visitor),
            _ => Err(Error::Custom(((self.first.unwrap() & 0b111_00000) >> 5).to_string())),
        }
    }

    #[inline]
    fn deserialize_enum<V: Visitor>(self,
                                    _enum: &'static str,
                                    variants: &'static [&'static str],
                                    visitor: V)
                                    -> Result<V::Value> {
        let first = self.read_u8()?;
        self.struct_fields = variants;
        let items = match (first & 0b111_00000) >> 5 {
            // simple enums packed repr is an integer
            0 => {
                self.first = Some(first);
                Some(0)
            }
            // simple enums are serialized as string
            3 => {
                self.first = Some(first);
                Some(0)
            }
            // variants with associated data as a sequence
            4 => self.parse_size_information(first)?,
            _ => return Err(Error::Syntax),
        };
        visitor.visit_enum(CompositeVisitor::new(self, items))
    }
    
    #[inline]
    fn deserialize_unit<V: Visitor>(&mut self, mut visitor: V) -> Result<V::Value> {
        // CBOR values, `null`, `undefined` and the empty map are accepted.
        let first = try!(self.read_u8());
        match first {
            0b111_10110 => visitor.visit_unit(),
            0b111_10111 => visitor.visit_unit(),
            _ if ((first & 0b111_00000) >> 5) == 5 => {
                let len = try!(self.parse_additional_information(first));
                if len == Some(0)
                        || (len == None && try!(self.read_u8()) == 0b111_11111) {
                    visitor.visit_unit()
                } else {
                    Err(Error::Syntax)
                }
            },
            0b111_11111 => Err(Error::StopCode),
            _ => Err(Error::Syntax),
        }
    }
}

impl<R: Read> Read for Deserializer<R> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.reader.read(buf)
    }
}

struct CompositeVisitor<'a, R: 'a + Read> {
    de: &'a mut Deserializer<R>,
    items: Option<usize>,
}

impl<'a, R: 'a + Read> CompositeVisitor<'a, R> {
    #[inline]
    fn new(de: &'a mut Deserializer<R>, items: Option<usize>) -> Self {
        CompositeVisitor {
            de: de,
            items: items,
        }
    }

    fn _visit<T: DeserializeSeed>(&mut self, seed: T) -> Result<Option<T::Value>> {
        match self.items {
            Some(0) => return Ok(None),
            Some(ref mut n) => *n -= 1,
            _ => {}
        };
        match seed.deserialize(&mut *self.de) {
            Ok(value) => Ok(Some(value)),
            Err(Error::StopCode) if self.items.is_none() => {
                self.items = Some(0);
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }

    fn _end(&mut self) -> Result<()> {
        if let Some(0) = self.items {
            Ok(())
        } else {
            Err(Error::TrailingBytes)
        }
    }

    fn _size_hint(&self) -> (usize, Option<usize>) {
        self.items.map_or((0, None), |n| (n, Some(n)))
    }
}

impl<'a, R: Read> de::SeqVisitor for CompositeVisitor<'a, R> {
    type Error = Error;
    fn visit_seed<T: DeserializeSeed>(&mut self, seed: T) -> Result<Option<T::Value>> {
        self._visit(seed)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self._size_hint()
    }
}

impl<'a, R: Read> de::MapVisitor for CompositeVisitor<'a, R> {
    type Error = Error;
    fn visit_key_seed<K: DeserializeSeed>(&mut self, seed: K) -> Result<Option<K::Value>> {
        self._visit(seed)
    }
    fn visit_value_seed<V: DeserializeSeed>(&mut self, seed: V) -> Result<V::Value> {
        seed.deserialize(&mut *self.de)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self._size_hint()
    }
}

impl<'a, R: Read> de::EnumVisitor for CompositeVisitor<'a, R> {
    type Error = Error;
    type Variant = Self;

    fn visit_variant_seed<V>(self, seed: V) -> Result<(V::Value, Self)>
        where V: de::DeserializeSeed
    {
        let val = seed.deserialize(&mut *self.de)?;
        Ok((val, self))
    }
}

impl<'a, R: Read> de::VariantVisitor for CompositeVisitor<'a, R> {
    type Error = Error;

    fn visit_unit(self) -> Result<()> {
        if self.items == Some(0) {
            Ok(())
        } else {
            Err(Error::Syntax)
        }
    }

    fn visit_newtype_seed<T: DeserializeSeed>(self, seed: T) -> Result<T::Value> {
        seed.deserialize(self.de)
    }

    fn visit_tuple<V: Visitor>(self, len: usize, visitor: V) -> Result<V::Value> {
        if self.items.is_some() && self.items != Some(len + 1) {
            return Err(Error::Syntax);
        }
        let seq = CompositeVisitor::new(self.de, Some(len));
        visitor.visit_seq(seq)
    }

    fn visit_struct<V: Visitor>(self,
                                _fields: &'static [&'static str],
                                visitor: V)
                                -> Result<V::Value> {
        de::Deserializer::deserialize(self.de, visitor)
    }
}

/// Decodes a CBOR value from a `std::io::Read`.
#[inline]
pub fn from_reader<T: Deserialize, R: Read>(reader: R) -> Result<T> {
    let mut de = Deserializer::new(reader);
    let value = Deserialize::deserialize(&mut de)?;
    de.end()?;
    Ok(value)
}

/// Decodes a CBOR value from a `&[u8]` slice.
#[inline]
pub fn from_slice<T: Deserialize>(v: &[u8]) -> Result<T> {
    from_reader(v)
}
