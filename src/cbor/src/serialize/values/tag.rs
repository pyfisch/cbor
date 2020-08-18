use crate::serialize::values::Value;

pub fn tag(tag: u64, value: Value) -> Value {
    Value::from_tag(tag, value)
}

pub fn self_describe(value: Value) -> Value {
    tag(55799, value)
}
