use crate::serialize::owned::OwnedValue;

pub fn array(values: &[OwnedValue]) -> OwnedValue {
    OwnedValue::from_array(values)
}

pub fn indefinite_array(values: &[OwnedValue]) -> OwnedValue {
    OwnedValue::from_indefinite_array(values)
}
