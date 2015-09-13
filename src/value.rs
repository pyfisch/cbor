use std::collections::HashMap;

use serde::de::{self, SeqVisitor};
use serde::ser;

#[derive(Debug, PartialEq)]
pub enum Value {
    U64(u64),
    I64(i64),
    Bytes(Vec<u8>),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<ObjectKey, Value>),
    F64(f64),
    Bool(bool),
    Null,
}

impl de::Deserialize for Value {
    #[inline]
    fn deserialize<D>(deserializer: &mut D) -> Result<Value, D::Error>
        where D: de::Deserializer,
    {
        struct ValueVisitor;

        impl de::Visitor for ValueVisitor {
            type Value = Value;

            #[inline]
            fn visit_str<E>(&mut self, value: &str) -> Result<Value, E>
                    where E: de::Error {
                self.visit_string(String::from(value))
            }

            #[inline]
            fn visit_string<E>(&mut self, value: String) -> Result<Value, E>
                    where E: de::Error {
                Ok(Value::String(value))
            }
            #[inline]
            fn visit_bytes<E>(&mut self, _v: &[u8]) -> Result<Self::Value, E>
                    where E: de::Error {
                self.visit_byte_buf(_v.to_owned())
            }

            #[inline]
            fn visit_byte_buf<E>(&mut self, v: Vec<u8>) -> Result<Self::Value, E>
                    where E: de::Error {
                Ok(Value::Bytes(v))
            }

            #[inline]
            fn visit_u64<E>(&mut self, v: u64) -> Result<Self::Value, E> where E: de::Error {
                Ok(Value::U64(v))
            }

            #[inline]
            fn visit_i64<E>(&mut self, v: i64) -> Result<Self::Value, E> where E: de::Error {
                Ok(Value::I64(v))
            }

            #[inline]
            fn visit_bool<E>(&mut self, _v: bool) -> Result<Self::Value, E> where E: de::Error {
                Ok(Value::Bool(_v))
            }

            #[inline]
            fn visit_none<E>(&mut self) -> Result<Self::Value, E> where E: de::Error {
                self.visit_unit()
            }

            #[inline]
            fn visit_unit<E>(&mut self) -> Result<Self::Value, E> where E: de::Error {
                Ok(Value::Null)
            }

            #[inline]
            fn visit_seq<V>(&mut self, _visitor: V) -> Result<Self::Value, V::Error>
                    where V: SeqVisitor {
                let values = try!(de::impls::VecVisitor::new().visit_seq(_visitor));
                Ok(Value::Array(values))
            }

            #[inline]
            fn visit_map<V>(&mut self, _visitor: V) -> Result<Value, V::Error>
                where V: de::MapVisitor,
            {
                let values = try!(de::impls::HashMapVisitor::new().visit_map(_visitor));
                Ok(Value::Object(values))
            }

            #[inline]
            fn visit_f64<E>(&mut self, v: f64) -> Result<Self::Value, E> where E: de::Error {
                Ok(Value::F64(v))
            }
        }

        deserializer.visit(ValueVisitor)
    }
}


impl ser::Serialize for Value {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: ser::Serializer {
        match *self {
            Value::U64(v) => serializer.visit_u64(v),
            Value::I64(v) => serializer.visit_i64(v),
            Value::Bytes(ref v) => serializer.visit_bytes(&v),
            Value::String(ref v) => serializer.visit_str(&v),
            Value::Array(ref v) => v.serialize(serializer),
            Value::Object(_) => unimplemented!(), //.serialize(serializer),
            Value::F64(v) => serializer.visit_f64(v),
            Value::Bool(v) => serializer.visit_bool(v),
            Value::Null => serializer.visit_unit(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ObjectKey {
    String(String),
    Bytes(Vec<u8>),
    Integer(i64),
    Bool(bool),
    Null,
}

impl de::Deserialize for ObjectKey {
    #[inline]
    fn deserialize<D>(deserializer: &mut D) -> Result<ObjectKey, D::Error>
        where D: de::Deserializer,
    {
        struct ObjectKeyVisitor;

        impl de::Visitor for ObjectKeyVisitor {
            type Value = ObjectKey;

            #[inline]
            fn visit_str<E>(&mut self, value: &str) -> Result<Self::Value, E>
                    where E: de::Error {
                self.visit_string(String::from(value))
            }

            #[inline]
            fn visit_string<E>(&mut self, value: String) -> Result<Self::Value, E>
                    where E: de::Error {
                Ok(ObjectKey::String(value))
            }
            #[inline]
            fn visit_bytes<E>(&mut self, _v: &[u8]) -> Result<Self::Value, E>
                    where E: de::Error {
                self.visit_byte_buf(_v.to_owned())
            }

            #[inline]
            fn visit_byte_buf<E>(&mut self, v: Vec<u8>) -> Result<Self::Value, E>
                    where E: de::Error {
                Ok(ObjectKey::Bytes(v))
            }

            #[inline]
            fn visit_u64<E>(&mut self, v: u64) -> Result<Self::Value, E> where E: de::Error {
                Ok(ObjectKey::Integer(v as i64))
            }

            #[inline]
            fn visit_i64<E>(&mut self, v: i64) -> Result<Self::Value, E> where E: de::Error {
                Ok(ObjectKey::Integer(v))
            }

            #[inline]
            fn visit_bool<E>(&mut self, _v: bool) -> Result<Self::Value, E> where E: de::Error {
                Ok(ObjectKey::Bool(_v))
            }

            #[inline]
            fn visit_none<E>(&mut self) -> Result<Self::Value, E> where E: de::Error {
                self.visit_unit()
            }

            #[inline]
            fn visit_unit<E>(&mut self) -> Result<Self::Value, E> where E: de::Error {
                Ok(ObjectKey::Null)
            }
        }

        deserializer.visit(ObjectKeyVisitor)
    }
}

impl ser::Serialize for ObjectKey {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: ser::Serializer {
        match *self {
            ObjectKey::String(ref v) => serializer.visit_str(&v),
            ObjectKey::Bytes(ref v) => serializer.visit_bytes(&v),
            ObjectKey::Integer(v) => serializer.visit_i64(v),
            ObjectKey::Bool(v) => serializer.visit_bool(v),
            ObjectKey::Null => serializer.visit_unit(),
        }
    }
}
