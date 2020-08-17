use crate::serialize::values::Value;

pub fn tag<'a>(tag: u64, value: &'a Value<'a>) -> Value<'a> {
    Value::from_tag(tag, value)
}

pub fn self_describe<'a>(value: &'a Value<'a>) -> Value<'a> {
    tag(55799, value)
}
