use std::collections::BTreeMap;

use serde::ser;

use crate::value::Value;
use crate::value::Value::*;

impl ser::Serialize for Value {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match self {
            Null => serializer.serialize_unit(),
            Bool(v) => serializer.serialize_bool(*v),
            Integer(v) => serializer.serialize_i128(*v),
            Float(v) => serializer.serialize_f64(*v),
            Bytes(v) => serializer.serialize_bytes(v.as_slice()),
            Text(v) => serializer.serialize_str(v.as_str()),
            Array(v) => v.serialize(serializer),
            Map(v) => BTreeMap::serialize(&v, serializer),
            __Hidden => unreachable!(),
        }
    }
}

