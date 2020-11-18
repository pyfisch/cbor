use std::fmt::{self, Debug};
use std::mem;
use crate::error::Error;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, SerializeStruct, Serializer},
};

pub(crate) const CBOR_RAW_VALUE_NAME: &str = "\0raw_value";

/// Reference to a range of bytes encompassing a single valid CBOR value
///
/// A `RawValue` can be used to defer parsing parts of a payload until later,
/// or to avoid parsing it at all in the case that part of the payload just
/// needs to be transferred verbatim into a different output object.
#[repr(transparent)]
pub struct RawValue {
    cbor: [u8],
}

impl RawValue {
    /// Access the underlying CBOR bytes
    pub fn get(&self) -> &[u8] {
        &self.cbor
    }

    /// Convert a `T: Serialize` into a boxed `RawValue`
    pub fn from_serialize<T>(other: &T) -> Result<Box<Self>, Error>
    where
        T: Serialize,
    {
        let cbor = crate::ser::to_vec(other)?;
        Ok(Box::<RawValue>::from(cbor))
    }
}

impl Clone for Box<RawValue> {
    fn clone(&self) -> Self {
        (**self).to_owned()
    }
}

impl ToOwned for RawValue {
    type Owned = Box<RawValue>;

    fn to_owned(&self) -> Self::Owned {
        (&self.cbor).into()
    }
}

impl<'a> From<&'a [u8]> for &'a RawValue {
    /// Convert a borrowed `&[u8]` of CBOR data to a borrowed `RawValue`
    ///
    /// **Note:** this function does not perform any validity checks on the
    /// provided input.
    fn from(other: &'a [u8]) -> Self {
        unsafe { &*(other as *const [u8] as *const RawValue) }
    }
}

impl<T> From<T> for Box<RawValue>
where
    T: Into<Box<[u8]>>,
{
    /// Convert a `&[u8]`, `Box<[u8]>`, or `Vec<u8>` to an owned `RawValue`
    ///
    /// **Note:** this function does not perform any validity checks on the
    /// provided input.
    fn from(other: T) -> Self {
        let boxed: Box<[u8]> = other.into();
        unsafe { mem::transmute::<Box<[u8]>, Self>(boxed) }
    }
}

impl<'a> From<&'a RawValue> for Box<RawValue> {
    fn from(other: &'a RawValue) -> Self {
        Self::from(&other.cbor)
    }
}

impl Debug for RawValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut hex = String::new();

        for byte in &self.cbor {
            hex.push_str(&format!("{:02X}", byte));
        }

        f
            .debug_tuple("RawValue")
            .field(&format_args!("{}", hex))
            .finish()
    }
}

impl Serialize for RawValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct(CBOR_RAW_VALUE_NAME, 1)?;
        s.serialize_field("", &RawValueNewtype(&self.cbor))?;
        s.end()
    }
}

// This is necessary because serde implements `serialize_seq` handlers on `[T]`,
// and we need it to use `serialize_bytes` instead
struct RawValueNewtype<'a>(&'a [u8]);

impl<'a> Serialize for RawValueNewtype<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(self.0)
    }
}

impl<'de> Deserialize<'de> for Box<RawValue> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // TODO don't roundtrip through Value
        let x = crate::value::Value::deserialize(deserializer)?;
        crate::to_vec(&x).map(|x| x.into()).map_err(|_| panic!())
    }
}

/* TODO
impl<'de: 'a, 'a> Deserialize<'de> for &'a RawValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}
*/
