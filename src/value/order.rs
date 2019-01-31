use std::cmp::Ordering;

use crate::value::Value;
use crate::value::Value::*;

impl Value {
    fn major_type(&self) -> u8 {
        match self {
            Null => 7,
            Bool(_) => 7,
            Integer(v) => if *v >= 0 { 0 } else { 1 },
            Float(_) => 7,
            Bytes(_) => 2,
            Text(_) => 3,
            Array(_) => 4,
            Map(_) => 5,
            __Hidden => unreachable!(),
        }
    }
}

impl Eq for Value {}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Null, Null) => true,
            (Bool(a), Bool(b)) => a == b,
            (Integer(a), Integer(b)) => a == b,
            (Float(a), Float(b)) => a.to_bits() == b.to_bits() || (a.is_nan() && b.is_nan()),
            (Bytes(a), Bytes(b)) => a == b,
            (Text(a), Text(b)) => a == b,
            (Array(a), Array(b)) => a == b,
            (Map(a), Map(b)) => a == b,
            _ => false,
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.major_type() != other.major_type() {
            return self.major_type().cmp(&other.major_type())
        }
        match (self, other) {
            (Bytes(a), Bytes(b)) if a.len() != b.len() => a.len().cmp(&b.len()),
            (Text(a), Text(b)) if a.len() != b.len() => a.len().cmp(&b.len()),
            (Array(a), Array(b)) if a.len() != b.len() => a.len().cmp(&b.len()),
            (Map(a), Map(b)) if a.len() != b.len() => a.len().cmp(&b.len()),
            (Bytes(a), Bytes(b)) => a.cmp(b),
            (Text(a), Text(b)) => a.cmp(b),
            (a, b) => {
                let a = crate::to_vec(a).expect("self is serializable");
                let b = crate::to_vec(b).expect("other is serializable");
                a.cmp(&b)
            }
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}