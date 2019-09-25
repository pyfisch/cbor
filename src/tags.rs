//! Serialize and Deserialize CBOR tags.
use crate::de::{Deserializer, TagType};
use crate::error::{Error, Result};
use crate::read::Read;
use crate::ser::Serializer;
use crate::write::Write;

use serde::de;
use serde::ser::{self, Serialize};

/// This is simply for serializing the tag itself (not the data)
#[derive(Debug)]
pub struct TagSerializer<'a, W> {
    ser: &'a mut Serializer<W>,
}

impl<'a, W> TagSerializer<'a, W>
where
    W: Write,
{
    fn new(ser: &'a mut Serializer<W>) -> Self {
        Self { ser }
    }
}

impl<'a, W> ser::Serializer for &mut TagSerializer<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = ser::Impossible<(), Error>;
    type SerializeTuple = ser::Impossible<(), Error>;
    type SerializeTupleStruct = ser::Impossible<(), Error>;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeMap = ser::Impossible<(), Error>;
    type SerializeStruct = ser::Impossible<(), Error>;
    type SerializeStructVariant = ser::Impossible<(), Error>;

    #[inline]
    fn serialize_bytes(self, _value: &[u8]) -> Result<()> {
        Err(Error::message("expected an u64, received bytes"))
    }

    #[inline]
    fn serialize_bool(self, _value: bool) -> Result<()> {
        Err(Error::message("expected an u64, received bool"))
    }

    #[inline]
    fn serialize_i8(self, _value: i8) -> Result<()> {
        Err(Error::message("expected an u64, received i8"))
    }

    #[inline]
    fn serialize_i16(self, _value: i16) -> Result<()> {
        Err(Error::message("expected an u64, received i16"))
    }

    #[inline]
    fn serialize_i32(self, _value: i32) -> Result<()> {
        Err(Error::message("expected an u64, received i32"))
    }

    #[inline]
    fn serialize_i64(self, _value: i64) -> Result<()> {
        Err(Error::message("expected an u64, received i64"))
    }

    #[inline]
    fn serialize_u8(self, _value: u8) -> Result<()> {
        Err(Error::message("expected an u64, received u8"))
    }

    #[inline]
    fn serialize_u16(self, _value: u16) -> Result<()> {
        Err(Error::message("expected an u64, received u16"))
    }

    #[inline]
    fn serialize_u32(self, _value: u32) -> Result<()> {
        Err(Error::message("expected an u64, received u32"))
    }

    // The `Tag` definition is with a u64, hence always only this case is hit. `write_64` will
    // make sure that the actual value is written with the smallest representation possible
    #[inline]
    fn serialize_u64(self, value: u64) -> Result<()> {
        self.ser.write_tag(value)
    }

    #[inline]
    fn serialize_f32(self, _value: f32) -> Result<()> {
        Err(Error::message("expected an u64, received f32"))
    }

    #[inline]
    fn serialize_f64(self, _value: f64) -> Result<()> {
        Err(Error::message("expected an u64, received f64"))
    }

    #[inline]
    fn serialize_char(self, _value: char) -> Result<()> {
        Err(Error::message("expected an u64, received char"))
    }

    #[inline]
    fn serialize_str(self, _value: &str) -> Result<()> {
        Err(Error::message("expected an u64, received str"))
    }

    #[inline]
    fn serialize_unit(self) -> Result<()> {
        Err(Error::message("expected an u64, received unit"))
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(Error::message("expected an u64, received unit_struct"))
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _idx: u32,
        _variant: &'static str,
    ) -> Result<()> {
        Err(Error::message("expected an u64, received unit_variant"))
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, _value: &T) -> Result<()>
    where
        T: Serialize,
    {
        Err(Error::message("expected an u64, received newtype_struct"))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _idx: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: Serialize,
    {
        Err(Error::message("expected an u64, received newtype_variant"))
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        Err(Error::message("expected an u64, received none"))
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<()>
    where
        T: Serialize,
    {
        Err(Error::message("expected an u64, received some"))
    }

    #[inline]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::message("expected an u64, received seq"))
    }

    #[inline]
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::message("expected an u64, received tuple"))
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::message("expected an u64, received tuple_struct"))
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _idx: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::message("expected an u64, received tuple_variant"))
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::message("expected an u64, received map"))
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(Error::message("expected an u64, received struct"))
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _idx: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::message("expected an u64, received struct_variant"))
    }
}

/// Represents CBOR serialization implementation for tags
#[derive(Debug)]
pub struct TagStructSerializer<'a, W> {
    // True if the tag (the first element of the tuple) was already read
    tag_read: bool,
    // The serializer for the actual tag
    tag_tag_ser: TagSerializer<'a, W>,
}

impl<'a, W> TagStructSerializer<'a, W>
where
    W: Write,
{
    /// Creates a new serializer for CBOR tags.
    pub fn new(ser: &'a mut Serializer<W>) -> Self {
        Self {
            tag_read: false,
            tag_tag_ser: TagSerializer::new(ser),
        }
    }
}

impl<'a, W> ser::Serializer for &mut TagStructSerializer<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = serde::ser::Impossible<(), Error>;
    type SerializeTuple = Self;
    type SerializeTupleStruct = serde::ser::Impossible<(), Error>;
    type SerializeTupleVariant = serde::ser::Impossible<(), Error>;
    type SerializeMap = serde::ser::Impossible<(), Error>;
    type SerializeStruct = serde::ser::Impossible<(), Error>;
    type SerializeStructVariant = serde::ser::Impossible<(), Error>;

    #[inline]
    fn serialize_bytes(self, _val: &[u8]) -> Result<()> {
        Err(Error::message("expected tuple, received bytes"))
    }

    #[inline]
    fn serialize_bool(self, _val: bool) -> Result<()> {
        Err(Error::message("expected tuple, received bool"))
    }

    #[inline]
    fn serialize_i8(self, _value: i8) -> Result<()> {
        Err(Error::message("expected tuple, received i8"))
    }

    #[inline]
    fn serialize_i16(self, _val: i16) -> Result<()> {
        Err(Error::message("expected tuple, received i16"))
    }

    #[inline]
    fn serialize_i32(self, _val: i32) -> Result<()> {
        Err(Error::message("expected tuple, received i32"))
    }

    #[inline]
    fn serialize_i64(self, _val: i64) -> Result<()> {
        Err(Error::message("expected tuple, received i64"))
    }

    #[inline]
    fn serialize_u8(self, _val: u8) -> Result<()> {
        Err(Error::message("expected tuple, received u8"))
    }

    #[inline]
    fn serialize_u16(self, _val: u16) -> Result<()> {
        Err(Error::message("expected tuple, received u16"))
    }

    #[inline]
    fn serialize_u32(self, _val: u32) -> Result<()> {
        Err(Error::message("expected tuple, received u32"))
    }

    #[inline]
    fn serialize_u64(self, _val: u64) -> Result<()> {
        Err(Error::message("expected tuple, received u64"))
    }

    #[inline]
    fn serialize_f32(self, _val: f32) -> Result<()> {
        Err(Error::message("expected tuple, received f32"))
    }

    #[inline]
    fn serialize_f64(self, _val: f64) -> Result<()> {
        Err(Error::message("expected tuple, received f64"))
    }

    #[inline]
    fn serialize_char(self, _val: char) -> Result<()> {
        Err(Error::message("expected tuple, received char"))
    }

    #[inline]
    fn serialize_str(self, _val: &str) -> Result<()> {
        Err(Error::message("expected tuple, received str"))
    }

    #[inline]
    fn serialize_unit(self) -> Result<()> {
        Err(Error::message("expected tuple, received unit"))
    }

    #[inline]
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(Error::message("expected tuple, received unit_struct"))
    }

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _idx: u32,
        _variant: &'static str,
    ) -> Result<()> {
        Err(Error::message("expected tuple, received unit_variant"))
    }

    #[inline]
    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, _value: &T) -> Result<()>
    where
        T: Serialize,
    {
        Err(Error::message("expected tuple, received newtype_struct"))
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _idx: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: Serialize,
    {
        Err(Error::message("expected tuple, received newtype_variant"))
    }

    #[inline]
    fn serialize_none(self) -> Result<()> {
        Err(Error::message("expected tuple, received none"))
    }

    #[inline]
    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<()>
    where
        T: Serialize,
    {
        Err(Error::message("expected tuple, received some"))
    }

    #[inline]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::message("expected tuple, received seq"))
    }

    #[inline]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        if len == 2 {
            Ok(self)
        } else {
            Err(Error::message(format!(
                "expected tuple with two elements, received tuple with {} elements",
                len
            )))
        }
    }

    #[inline]
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::message("expected tuple, received tuple_struct"))
    }

    #[inline]
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _idx: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::message("expected tuple, received tuple_variant"))
    }

    #[inline]
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::message("expected tuple, received map"))
    }

    #[inline]
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(Error::message("expected tuple, received struct"))
    }

    #[inline]
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _idx: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::message("expected tuple, received struct_variant"))
    }
}

impl<'a, W> ser::SerializeTuple for &mut TagStructSerializer<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + ser::Serialize>(&mut self, value: &T) -> Result<()> {
        // Serialize the value with the default serializer
        if self.tag_read {
            value.serialize(&mut *self.tag_tag_ser.ser)
        }
        // Serialize the tag itself
        else {
            self.tag_read = true;
            value.serialize(&mut self.tag_tag_ser)
        }
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
enum TagDeserializerState {
    New,
    ReadTag,
    ReadData,
}

/// Deserialize a CBOR tag and its value
#[derive(Debug)]
pub struct TagDeserializer<'a, R> {
    de: &'a mut Deserializer<R>,
    tag_type: TagType,
    state: TagDeserializerState,
}

impl<'de, 'a, R> TagDeserializer<'a, R>
where
    R: Read<'de> + 'a,
{
    /// Creates a new TagDeserializer.
    pub fn new(de: &'a mut Deserializer<R>, tag_type: TagType) -> Self {
        TagDeserializer {
            de,
            tag_type,
            state: TagDeserializerState::New,
        }
    }
}

impl<'de, 'a, R> de::Deserializer<'de> for TagDeserializer<'a, R>
where
    R: Read<'de> + 'a,
{
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(self)
    }

    serde::forward_to_deserialize_any! {
       bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit option
       seq bytes byte_buf map unit_struct newtype_struct
       struct identifier tuple enum ignored_any tuple_struct
    }
}

impl<'de, 'a, R> de::SeqAccess<'de> for TagDeserializer<'a, R>
where
    R: Read<'de> + 'a,
{
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        Ok(Some(seed.deserialize(self)?))
    }
}

/// Deserializer for Tag SeqAccess
impl<'de, 'a, R> de::Deserializer<'de> for &mut TagDeserializer<'a, R>
where
    R: Read<'de> + 'a,
{
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match self.state {
            TagDeserializerState::New => {
                let tag = self.de.parse_tag_by_type(self.tag_type)?;
                self.state = TagDeserializerState::ReadTag;
                visitor.visit_u64(tag)
            }
            TagDeserializerState::ReadTag => {
                self.state = TagDeserializerState::ReadData;
                self.de.parse_value(visitor)
            }
            TagDeserializerState::ReadData => unreachable!(),
        }
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit option
        seq bytes byte_buf map unit_struct newtype_struct
        tuple_struct struct identifier tuple enum ignored_any
    }
}
