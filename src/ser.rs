//! CBOR serializisation.

use std::io::Write;

use byteorder::{BigEndian, WriteBytesExt};
use serde::ser::{self, Serialize, Serializer as SerdeSerializer};

use {Error, Result};

/// A structure for serializing Rust values into CBOR.
pub struct Serializer<W: Write> {
    writer: W,
    packed: bool,
}

impl<W: Write> Serializer<W> {
    /// Creates a new CBOR serializer.
    pub fn new(writer: W) -> Serializer<W> {
        Serializer {
            writer: writer,
            packed: false,
        }
    }

    /// Creates a new packed CBOR serializer.
    ///
    /// In packed mode all struct field names and enum variant names
    /// are not serialized instead the index position of the field
    /// is used.
    pub fn packed(writer: W) -> Serializer<W> {
        Serializer {
            writer: writer,
            packed: true,
        }
    }

    /// Writes the CBOR self-describe tag to the stream.
    ///
    /// Tagging allows a decoder to distinguish different file formats
    /// based on their content without other information.
    pub fn self_describe(&mut self) -> Result<()> {
        self.writer.write_u8(6 << 5 | 25)?;
        self.writer.write_u16::<BigEndian>(55799)?;
        Ok(())
    }

    #[inline]
    fn write_type_u8(&mut self, major: u8, additional: u8) -> Result<()> {
        if additional > 23 {
            self.writer.write_u8(major << 5 | 24)?;
            self.writer.write_u8(additional)?;
        } else {
            self.writer.write_u8(major << 5 | additional)?;
        }
        Ok(())
    }

    #[inline]
    fn write_type_u16(&mut self, major: u8, additional: u16) -> Result<()> {
        if additional > ::std::u8::MAX as u16 {
            self.writer.write_u8(major << 5 | 25)?;
            self.writer.write_u16::<BigEndian>(additional)?;
        } else {
            self.write_type_u8(major, additional as u8)?;
        }
        Ok(())
    }

    #[inline]
    fn write_type_u32(&mut self, major: u8, additional: u32) -> Result<()> {
        if additional > ::std::u16::MAX as u32 {
            self.writer.write_u8(major << 5 | 26)?;
            self.writer.write_u32::<BigEndian>(additional)?;
        } else {
            self.write_type_u16(major, additional as u16)?;
        }
        Ok(())
    }

    #[inline]
    fn write_type_u64(&mut self, major: u8, additional: u64) -> Result<()> {
        if additional > ::std::u32::MAX as u64 {
            self.writer.write_u8(major << 5 | 27)?;
            self.writer.write_u64::<BigEndian>(additional)?;
        } else {
            self.write_type_u32(major, additional as u32)?;
        }
        Ok(())
    }

    #[inline]
    fn write_collection_start(&mut self,
                              major: u8,
                              len: Option<usize>)
                              -> Result<Compound<W, CollectionState>> {
        if let Some(len) = len {
            self.write_type_u64(major, len as u64)?;
            Ok(Compound {
                ser: self,
                state: CollectionState::Fixed,
            })
        } else {
            self.writer.write_u8(major << 5 | 31)?;
            Ok(Compound {
                ser: self,
                state: CollectionState::Indefinite,
            })
        }
    }

    #[inline]
    fn write_collection_end(&mut self, state: CollectionState) -> Result<()> {
        match state {
            CollectionState::Fixed => Ok(()),
            CollectionState::Indefinite => self.writer.write_u8(0xff).map_err(From::from),
        }
    }
}

#[doc(hidden)]
pub enum CollectionState {
    Fixed,
    Indefinite,
}

#[doc(hidden)]
pub struct StructState {
    counter: usize,
}

#[doc(hidden)]
pub struct Compound<'a, W: 'a + Write, S> {
    ser: &'a mut Serializer<W>,
    state: S,
}

impl<'a, W: Write> ser::SerializeSeq for Compound<'a, W, CollectionState> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<()> {
        self.ser.write_collection_end(self.state)
    }
}

impl<'a, W: Write> ser::SerializeTuple for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeTupleStruct for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeTupleVariant for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeMap for Compound<'a, W, CollectionState> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut *self.ser)
    }

    fn serialize_value<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<()> {
        self.ser.write_collection_end(self.state)
    }
}

impl<'a, W: Write> ser::SerializeStruct for Compound<'a, W, StructState> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ser::Serialize>(&mut self,
                                                   key: &'static str,
                                                   value: &T)
                                                   -> Result<()> {
        if !self.ser.packed {
            self.ser.serialize_str(key)?;
        } else {
            self.ser.serialize_u64(self.state.counter as u64)?;
        }
        value.serialize(&mut *self.ser)?;
        self.state.counter += 1;
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W: Write> ser::SerializeStructVariant for Compound<'a, W, StructState> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + ser::Serialize>(&mut self,
                                                   key: &'static str,
                                                   value: &T)
                                                   -> Result<()> {
        if !self.ser.packed {
            self.ser.serialize_str(key)?;
        } else {
            self.ser.serialize_u64(self.state.counter as u64)?;
        }
        value.serialize(&mut *self.ser)?;
        self.state.counter += 1;
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, W: Write> ser::Serializer for &'a mut Serializer<W> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Compound<'a, W, CollectionState>;
    type SerializeTuple = &'a mut Serializer<W>;
    type SerializeTupleStruct = &'a mut Serializer<W>;
    type SerializeTupleVariant = &'a mut Serializer<W>;
    type SerializeMap = Compound<'a, W, CollectionState>;
    type SerializeStruct = Compound<'a, W, StructState>;
    type SerializeStructVariant = Compound<'a, W, StructState>;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<()> {
        self.writer
            .write_u8(match v {
                false => 7 << 5 | 20,
                true => 7 << 5 | 21,
            })
            .map_err(From::from)
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<()> {
        if v < 0 {
            self.write_type_u8(1, -(v + 1) as u8)
        } else {
            self.serialize_u8(v as u8)
        }
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<()> {
        if v < 0 {
            self.write_type_u16(1, -(v + 1) as u16)
        } else {
            self.serialize_u16(v as u16)
        }
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<()> {
        if v < 0 {
            self.write_type_u32(1, -(v + 1) as u32)
        } else {
            self.serialize_u32(v as u32)
        }
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<()> {
        if v < 0 {
            self.write_type_u64(1, -(v + 1) as u64)
        } else {
            self.serialize_u64(v as u64)
        }
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<()> {
        self.write_type_u8(0, v)
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<()> {
        self.write_type_u16(0, v)
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<()> {
        self.write_type_u32(0, v)
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<()> {
        self.write_type_u64(0, v)
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<()> {
        // TODO: Encode to f16
        if v.is_infinite() && v.is_sign_positive() {
                self.writer.write_all(&[0xf9, 0x7c, 0x00])
            } else if v.is_infinite() && v.is_sign_negative() {
                self.writer.write_all(&[0xf9, 0xfc, 0x00])
            } else if v.is_nan() {
                self.writer.write_all(&[0xf9, 0x7e, 0x00])
            } else {
                self.writer
                    .write_u8(7 << 5 | 26)
                    .and_then(|()| self.writer.write_f32::<BigEndian>(v))
            }
            .map_err(From::from)
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<()> {
        // TODO: Encode to f16
        if v.is_infinite() && v.is_sign_positive() {
                self.writer.write_all(&[0xf9, 0x7c, 0x00])
            } else if v.is_infinite() && v.is_sign_negative() {
                self.writer.write_all(&[0xf9, 0xfc, 0x00])
            } else if v.is_nan() {
                self.writer.write_all(&[0xf9, 0x7e, 0x00])
            } else if v as f32 as f64 == v {
                self.writer
                    .write_u8(7 << 5 | 26)
                    .and_then(|()| self.writer.write_f32::<BigEndian>(v as f32))
            } else {
                self.writer
                    .write_u8(7 << 5 | 27)
                    .and_then(|()| self.writer.write_f64::<BigEndian>(v))
            }
            .map_err(From::from)
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<()> {
        // TODO: Avoid allocation. rust-lang/rust#27784
        let mut s = String::new();
        s.push(v);
        self.serialize_str(s.as_str())
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<()> {
        self.write_type_u64(3, value.len() as u64)?;
        self.writer.write_all(value.as_bytes()).map_err(From::from)
    }

    #[inline]
    fn serialize_bytes(self, value: &[u8]) -> Result<()> {
        self.write_type_u64(2, value.len() as u64)?;
        self.writer.write_all(value).map_err(From::from)
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<()> {
        value.serialize(self)
    }

    #[inline]
    fn serialize_unit(self) -> Result<()> {
        self.writer.write_u8(7 << 5 | 22).map_err(From::from)
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_unit_variant(self,
                              _name: &'static str,
                              variant_index: usize,
                              variant: &'static str)
                              -> Result<()> {
        if !self.packed {
            self.serialize_str(variant)
        } else {
            self.serialize_u64(variant_index as u64)
        }
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized + Serialize>(self,
                                                       _name: &'static str,
                                                       value: &T)
                                                       -> Result<()> {
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<T: ?Sized + Serialize>(self,
                                                        name: &'static str,
                                                        variant_index: usize,
                                                        variant: &'static str,
                                                        value: &T)
                                                        -> Result<()> {
        self.writer.write_u8(4 << 5 | 2)?;
        self.serialize_unit_variant(name, variant_index, variant)?;
        value.serialize(self)
    }

    #[inline]
    fn serialize_seq(self, len: Option<usize>) -> Result<Compound<'a, W, CollectionState>> {
        self.write_collection_start(4, len)
    }

    #[inline]
    fn serialize_seq_fixed_size(self, size: usize) -> Result<Compound<'a, W, CollectionState>> {
        self.serialize_seq(Some(size))
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<&'a mut Serializer<W>> {
        self.write_type_u64(4, len as u64)?;
        Ok(self)
    }

    #[inline]
    fn serialize_tuple_struct(self,
                              _name: &'static str,
                              len: usize)
                              -> Result<&'a mut Serializer<W>> {
        self.serialize_tuple(len)
    }

    #[inline]
    fn serialize_tuple_variant(self,
                               name: &'static str,
                               variant_index: usize,
                               variant: &'static str,
                               len: usize)
                               -> Result<&'a mut Serializer<W>> {
        self.write_type_u64(4, (len + 1) as u64)?;
        self.serialize_unit_variant(name, variant_index, variant)?;
        Ok(self)
    }

    #[inline]
    fn serialize_map(self, len: Option<usize>) -> Result<Compound<'a, W, CollectionState>> {
        self.write_collection_start(5, len)
    }

    #[inline]
    fn serialize_struct(self,
                        _name: &'static str,
                        len: usize)
                        -> Result<Compound<'a, W, StructState>> {
        self.write_type_u64(5, len as u64)?;
        Ok(Compound {
            ser: self,
            state: StructState { counter: 0 },
        })
    }

    #[inline]
    fn serialize_struct_variant(self,
                                name: &'static str,
                                variant_index: usize,
                                variant: &'static str,
                                len: usize)
                                -> Result<Compound<'a, W, StructState>> {
        self.writer.write_u8(4 << 5 | 2)?;
        self.serialize_unit_variant(name, variant_index, variant)?;
        self.write_type_u64(5, len as u64)?;
        Ok(Compound {
            ser: self,
            state: StructState { counter: 0 },
        })
    }
}

/// Serializes a value to a writer.
pub fn to_writer<W: Write, T: Serialize>(mut writer: &mut W, value: &T) -> Result<()> {
    value.serialize(&mut Serializer::new(&mut writer))
}

/// Serializes a value to a writer and add a CBOR self-describe tag.
pub fn to_writer_sd<W: Write, T: Serialize>(mut writer: &mut W, value: &T) -> Result<()> {
    let mut ser = Serializer::new(&mut writer);
    ser.self_describe()?;
    value.serialize(&mut ser)
}

/// Serializes a value without names to a writer.
pub fn to_writer_packed<W: Write, T: Serialize>(mut writer: &mut W, value: &T) -> Result<()> {
    value.serialize(&mut Serializer::packed(&mut writer))
}

/// Serializes a value without names to a writer and add a CBOR self-describe tag.
pub fn to_writer_packed_sd<W: Write, T: Serialize>(mut writer: &mut W, value: &T) -> Result<()> {
    let mut ser = Serializer::packed(&mut writer);
    ser.self_describe()?;
    value.serialize(&mut ser)
}

/// Serializes a value to a vector.
pub fn to_vec<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let mut vec = Vec::new();
    to_writer(&mut vec, value)?;
    Ok(vec)
}

/// Serializes a value to a vector and add a CBOR self-describe tag.
pub fn to_vec_sd<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let mut vec = Vec::new();
    to_writer_sd(&mut vec, value)?;
    Ok(vec)
}

/// Serializes a value without names to a vector.
pub fn to_vec_packed<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let mut vec = Vec::new();
    to_writer_packed(&mut vec, value)?;
    Ok(vec)
}
/// Serializes a value without names to a vector and add a CBOR self-describe tag.
pub fn to_vec_packed_sd<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let mut vec = Vec::new();
    to_writer_packed_sd(&mut vec, value)?;
    Ok(vec)
}
