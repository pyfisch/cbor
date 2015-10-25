//! CBOR values and keys.

use std::collections::HashMap;

use serde::de::{self, SeqVisitor};
use serde::ser;

/// An enum over all possible CBOR types.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// Represents an unsigned integer.
    U64(u64),
    /// Represents a signed integer.
    I64(i64),
    /// Represents a byte string.
    Bytes(Vec<u8>),
    /// Represents an UTF-8 string.
    String(String),
    /// Represents a list.
    Array(Vec<Value>),
    /// Represents a map.
    Object(HashMap<ObjectKey, Value>),
    /// Represents a floating point value.
    F64(f64),
    /// Represents a boolean value.
    Bool(bool),
    /// Represents the absence of a value or the value undefined.
    Null,
}

impl Value {
    /// Returns true if the value is an object.
    pub fn is_object(&self) -> bool {
        self.as_object().is_some()
    }

    /// If the value is an object, returns the associated BTreeMap. Returns None otherwise.
    pub fn as_object(&self) -> Option<&HashMap<ObjectKey, Value>> {
        if let Value::Object(ref v) = *self {
            Some(v)
        } else {
            None
        }
    }


    /// If the value is an object, returns the associated mutable BTreeMap. Returns None otherwise.
    pub fn as_object_mut(&mut self) -> Option<&mut HashMap<ObjectKey, Value>> {
        if let Value::Object(ref mut v) = *self {
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
        if let Value::String(ref v) = *self {
            Some(v)
        } else {
            None
        }
    }

    /// Returns the associated mutable string or `None` if the value has a different type.
    pub fn as_string_mut(&mut self) -> Option<&mut String> {
        if let Value::String(ref mut v) = *self {
            Some(v)
        } else {
            None
        }
    }

    /// Retrns true if the value is a number.
    pub fn is_number(&self) -> bool {
        match *self {
            Value::U64(_) | Value::I64(_) | Value::F64(_) => true,
            _ => false,
        }
    }

    /// Returns true if the `Value` is a i64. Returns false otherwise.
    pub fn is_i64(&self) -> bool {
        match *self {
            Value::I64(_) => true,
            _ => false,
        }
    }

    /// Returns true if the `Value` is a u64. Returns false otherwise.
    pub fn is_u64(&self) -> bool {
        match *self {
            Value::U64(_) => true,
            _ => false,
        }
    }

    /// Returns true if the `Value` is a f64. Returns false otherwise.
    pub fn is_f64(&self) -> bool {
        match *self {
            Value::F64(_) => true,
            _ => false,
        }
    }

    /// If the `Value` is a number, return or cast it to a i64. Returns None otherwise.
    pub fn as_i64(&self) -> Option<i64> {
        match *self {
            Value::I64(n) => Some(n),
            Value::U64(n) => Some(n as i64),
            _ => None,
        }
    }

    /// If the `Value` is a number, return or cast it to a u64. Returns None otherwise.
    pub fn as_u64(&self) -> Option<u64> {
        match *self {
            Value::I64(n) => Some(n as u64),
            Value::U64(n) => Some(n),
            _ => None,
        }
    }

    /// If the `Value` is a number, return or cast it to a f64. Returns None otherwise.
    pub fn as_f64(&self) -> Option<f64> {
        match *self {
            Value::I64(n) => Some(n as f64),
            Value::U64(n) => Some(n as f64),
            Value::F64(n) => Some(n),
            _ => None,
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

impl de::Deserialize for Value {
    #[inline]
    fn deserialize<D>(deserializer: &mut D) -> Result<Value, D::Error>
        where D: de::Deserializer
    {
        struct ValueVisitor;

        impl de::Visitor for ValueVisitor {
            type Value = Value;

            #[inline]
            fn visit_str<E>(&mut self, value: &str) -> Result<Value, E>
                where E: de::Error
            {
                self.visit_string(String::from(value))
            }

            #[inline]
            fn visit_string<E>(&mut self, value: String) -> Result<Value, E>
                where E: de::Error
            {
                Ok(Value::String(value))
            }
            #[inline]
            fn visit_bytes<E>(&mut self, _v: &[u8]) -> Result<Self::Value, E>
                where E: de::Error
            {
                self.visit_byte_buf(_v.to_owned())
            }

            #[inline]
            fn visit_byte_buf<E>(&mut self, v: Vec<u8>) -> Result<Self::Value, E>
                where E: de::Error
            {
                Ok(Value::Bytes(v))
            }

            #[inline]
            fn visit_u64<E>(&mut self, v: u64) -> Result<Self::Value, E>
                where E: de::Error
            {
                Ok(Value::U64(v))
            }

            #[inline]
            fn visit_i64<E>(&mut self, v: i64) -> Result<Self::Value, E>
                where E: de::Error
            {
                Ok(Value::I64(v))
            }

            #[inline]
            fn visit_bool<E>(&mut self, _v: bool) -> Result<Self::Value, E>
                where E: de::Error
            {
                Ok(Value::Bool(_v))
            }

            #[inline]
            fn visit_none<E>(&mut self) -> Result<Self::Value, E>
                where E: de::Error
            {
                self.visit_unit()
            }

            #[inline]
            fn visit_unit<E>(&mut self) -> Result<Self::Value, E>
                where E: de::Error
            {
                Ok(Value::Null)
            }

            #[inline]
            fn visit_seq<V>(&mut self, _visitor: V) -> Result<Self::Value, V::Error>
                where V: SeqVisitor
            {
                let values = try!(de::impls::VecVisitor::new().visit_seq(_visitor));
                Ok(Value::Array(values))
            }

            #[inline]
            fn visit_map<V>(&mut self, _visitor: V) -> Result<Value, V::Error>
                where V: de::MapVisitor
            {
                let values = try!(de::impls::HashMapVisitor::new().visit_map(_visitor));
                Ok(Value::Object(values))
            }

            #[inline]
            fn visit_f64<E>(&mut self, v: f64) -> Result<Self::Value, E>
                where E: de::Error
            {
                Ok(Value::F64(v))
            }
        }

        deserializer.visit(ValueVisitor)
    }
}


impl ser::Serialize for Value {
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: ser::Serializer
    {
        match *self {
            Value::U64(v) => serializer.visit_u64(v),
            Value::I64(v) => serializer.visit_i64(v),
            Value::Bytes(ref v) => serializer.visit_bytes(&v),
            Value::String(ref v) => serializer.visit_str(&v),
            Value::Array(ref v) => v.serialize(serializer),
            Value::Object(ref v) => v.serialize(serializer),
            Value::F64(v) => serializer.visit_f64(v),
            Value::Bool(v) => serializer.visit_bool(v),
            Value::Null => serializer.visit_unit(),
        }
    }
}

/// A simplified CBOR value containing only types useful for keys.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ObjectKey {
    /// An integer.
    Integer(i64),
    /// A byte string.
    Bytes(Vec<u8>),
    /// An UTF-8 string.
    String(String),
    /// A boolean value.
    Bool(bool),
    /// No value.
    Null,
}

impl de::Deserialize for ObjectKey {
    #[inline]
    fn deserialize<D>(deserializer: &mut D) -> Result<ObjectKey, D::Error>
        where D: de::Deserializer
    {
        struct ObjectKeyVisitor;

        impl de::Visitor for ObjectKeyVisitor {
            type Value = ObjectKey;

            #[inline]
            fn visit_str<E>(&mut self, value: &str) -> Result<Self::Value, E>
                where E: de::Error
            {
                self.visit_string(String::from(value))
            }

            #[inline]
            fn visit_string<E>(&mut self, value: String) -> Result<Self::Value, E>
                where E: de::Error
            {
                Ok(ObjectKey::String(value))
            }
            #[inline]
            fn visit_bytes<E>(&mut self, _v: &[u8]) -> Result<Self::Value, E>
                where E: de::Error
            {
                self.visit_byte_buf(_v.to_owned())
            }

            #[inline]
            fn visit_byte_buf<E>(&mut self, v: Vec<u8>) -> Result<Self::Value, E>
                where E: de::Error
            {
                Ok(ObjectKey::Bytes(v))
            }

            #[inline]
            fn visit_u64<E>(&mut self, v: u64) -> Result<Self::Value, E>
                where E: de::Error
            {
                Ok(ObjectKey::Integer(v as i64))
            }

            #[inline]
            fn visit_i64<E>(&mut self, v: i64) -> Result<Self::Value, E>
                where E: de::Error
            {
                Ok(ObjectKey::Integer(v))
            }

            #[inline]
            fn visit_bool<E>(&mut self, _v: bool) -> Result<Self::Value, E>
                where E: de::Error
            {
                Ok(ObjectKey::Bool(_v))
            }

            #[inline]
            fn visit_none<E>(&mut self) -> Result<Self::Value, E>
                where E: de::Error
            {
                self.visit_unit()
            }

            #[inline]
            fn visit_unit<E>(&mut self) -> Result<Self::Value, E>
                where E: de::Error
            {
                Ok(ObjectKey::Null)
            }
        }

        deserializer.visit(ObjectKeyVisitor)
    }
}

impl ser::Serialize for ObjectKey {
    #[inline]
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error>
        where S: ser::Serializer
    {
        match *self {
            ObjectKey::Integer(v) => serializer.visit_i64(v),
            ObjectKey::Bytes(ref v) => serializer.visit_bytes(&v),
            ObjectKey::String(ref v) => serializer.visit_str(&v),
            ObjectKey::Bool(v) => serializer.visit_bool(v),
            ObjectKey::Null => serializer.visit_unit(),
        }
    }
}

impl From<ObjectKey> for Value {
    fn from(key: ObjectKey) -> Value {
        match key {
            ObjectKey::Integer(v) => Value::I64(v),
            ObjectKey::Bytes(v) => Value::Bytes(v),
            ObjectKey::String(v) => Value::String(v),
            ObjectKey::Bool(v) => Value::Bool(v),
            ObjectKey::Null => Value::Null,
        }
    }
}

impl From<Value> for ObjectKey {
    fn from(value: Value) -> ObjectKey {
        match value {
            Value::U64(v) => ObjectKey::Integer(v as i64),
            Value::I64(v) => ObjectKey::Integer(v),
            Value::Bytes(v) => ObjectKey::Bytes(v),
            Value::String(v) => ObjectKey::String(v),
            Value::Bool(v) => ObjectKey::Bool(v),
            Value::Null => ObjectKey::Null,
            _ => panic!("invalid value type for key"),
        }
    }
}
