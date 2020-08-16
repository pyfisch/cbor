use crate::serialize::owned::OwnedValue;

pub fn key_value(key: OwnedValue, value: OwnedValue) -> (OwnedValue, OwnedValue) {
    (key, value)
}

pub fn map(pairs: &[(OwnedValue, OwnedValue)]) -> OwnedValue {
    OwnedValue::from_map(pairs)
}

pub fn indefinite_map<M: AsRef<[(OwnedValue, OwnedValue)]>>(pairs: M) -> OwnedValue {
    OwnedValue::from_indefinite_map(pairs.as_ref())
}
