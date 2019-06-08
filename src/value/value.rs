//! CBOR values and keys.

use std::collections::BTreeMap;
use std::fmt;

use serde::de;
use serde::ser;

use crate::value::Value;

impl<'de> de::Deserialize<'de> for Value {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> de::Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.write_str("any valid CBOR value")
            }

            #[inline]
            fn visit_str<E>(self, value: &str) -> Result<Value, E>
            where
                E: de::Error,
            {
                self.visit_string(String::from(value))
            }

            #[inline]
            fn visit_string<E>(self, value: String) -> Result<Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Text(value))
            }
            #[inline]
            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_byte_buf(v.to_owned())
            }

            #[inline]
            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Bytes(v))
            }

            #[inline]
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Integer(v.into()))
            }

            #[inline]
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Integer(v.into()))
            }

            #[inline]
            fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Integer(v))
            }

            #[inline]
            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Bool(v))
            }

            #[inline]
            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_unit()
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Null)
            }

            #[inline]
            fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: de::SeqAccess<'de>,
            {
                let mut vec = Vec::new();

                while let Some(elem) = visitor.next_element()? {
                    vec.push(elem);
                }

                Ok(Value::Array(vec))
            }

            #[inline]
            fn visit_map<V>(self, mut visitor: V) -> Result<Value, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let mut values = BTreeMap::new();

                while let Some((key, value)) = visitor.next_entry()? {
                    values.insert(key, value);
                }

                Ok(Value::Map(values))
            }

            #[inline]
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Value::Float(v))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

impl ser::Serialize for Value {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match *self {
            Value::Integer(v) => serializer.serialize_i128(v),
            Value::Bytes(ref v) => serializer.serialize_bytes(&v),
            Value::Text(ref v) => serializer.serialize_str(&v),
            Value::Array(ref v) => v.serialize(serializer),
            Value::Map(ref v) => v.serialize(serializer),
            Value::Float(v) => serializer.serialize_f64(v),
            Value::Bool(v) => serializer.serialize_bool(v),
            Value::Null => serializer.serialize_unit(),
            Value::__Hidden => unreachable!(),
        }
    }
}

/// Convert a `serde_cbor::Value` into a type `T`
#[allow(clippy::needless_pass_by_value)]
pub fn from_value<T>(value: Value) -> Result<T, crate::error::Error>
where
    T: de::DeserializeOwned,
{
    // TODO implement in a way that doesn't require
    // roundtrip through buffer (i.e. by implementing
    // `serde::de::Deserializer` for `Value` and then doing
    // `T::deserialize(value)`).
    let buf = crate::to_vec(&value)?;
    crate::from_slice(buf.as_slice())
}
