use crate::serialize::owned::OwnedValue;

pub fn tag(tag: u64, value: OwnedValue) -> OwnedValue {
    OwnedValue::from_tag(tag, value)
}
