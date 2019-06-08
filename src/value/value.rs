//! CBOR values and keys.

use std::collections::BTreeMap;
use std::fmt;

use serde::de;
use serde::ser;

use crate::value::Value;

impl Value {
    /// Returns true if the value is an object.
    pub fn is_object(&self) -> bool {
        self.as_object().is_some()
    }

    /// If the value is an object, returns the associated BTreeMap. Returns None otherwise.
    pub fn as_object(&self) -> Option<&BTreeMap<Value, Value>> {
        if let Value::Map(ref v) = *self {
            Some(v)
        } else {
            None
        }
    }

    /// If the value is an object, returns the associated mutable BTreeMap. Returns None otherwise.
    pub fn as_object_mut(&mut self) -> Option<&mut BTreeMap<Value, Value>> {
        if let Value::Map(ref mut v) = *self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns true if the value is an array.
    pub fn is_array(&self) -> bool {
        self.as_array().is_some()
    }

    /// If the value is an array, returns the associated Vec. Returns None otherwise.
    pub fn as_array(&self) -> Option<&Vec<Value>> {
        if let Value::Array(ref v) = *self {
            Some(v)
        } else {
            None
        }
    }

    /// If the value is an array, returns the associated mutable Vec. Returns None otherwise.
    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        if let Value::Array(ref mut v) = *self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns true if the value is a byte string.
    pub fn is_bytes(&self) -> bool {
        self.as_bytes().is_some()
    }

    /// Returns the associated byte string or `None` if the value has a different type.
    pub fn as_bytes(&self) -> Option<&Vec<u8>> {
        if let Value::Bytes(ref v) = *self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns the associated mutable byte string or `None` if the value has a different type.
    pub fn as_bytes_mut(&mut self) -> Option<&mut Vec<u8>> {
        if let Value::Bytes(ref mut v) = *self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns true if the value is a string.
    pub fn is_string(&self) -> bool {
        self.as_string().is_some()
    }

    /// Returns the associated string or `None` if the value has a different type.
    pub fn as_string(&self) -> Option<&String> {
        if let Value::Text(ref v) = *self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns the associated mutable string or `None` if the value has a different type.
    pub fn as_string_mut(&mut self) -> Option<&mut String> {
        if let Value::Text(ref mut v) = *self {
            Some(v)
        } else {
            None
        }
    }

    /// Retrns true if the value is a number.
    pub fn is_number(&self) -> bool {
        match *self {
            Value::Integer(_) | Value::Float(_) => true,
            _ => false,
        }
    }

    /// Returns true if the value is a boolean.
    pub fn is_boolean(&self) -> bool {
        self.as_boolean().is_some()
    }

    /// If the value is a Boolean, returns the associated bool. Returns None otherwise.
    pub fn as_boolean(&self) -> Option<bool> {
        if let Value::Bool(v) = *self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns true if the value is a Null. Returns false otherwise.
    pub fn is_null(&self) -> bool {
        self.as_null().is_some()
    }

    /// If the value is a Null, returns (). Returns None otherwise.
    pub fn as_null(&self) -> Option<()> {
        if let Value::Null = *self {
            Some(())
        } else {
            None
        }
    }
}

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

macro_rules! impl_from {
    ($for_enum:ident, $variant:ident, $for_type:ty) => {
        impl From<$for_type> for $for_enum {
            fn from(v: $for_type) -> $for_enum {
                $for_enum::$variant(v)
            }
        }
    };
}

// All except &'a str and Cow<'a, str>
impl_from!(Value, Bool, bool);
impl_from!(Value, Integer, i128);
impl_from!(Value, Float, f64);
impl_from!(Value, Bytes, Vec<u8>);
impl_from!(Value, Text, String);
impl_from!(Value, Array, Vec<Value>);
impl_from!(Value, Map, BTreeMap<Value, Value>);

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
