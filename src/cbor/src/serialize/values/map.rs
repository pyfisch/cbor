use crate::serialize::values::Value;

pub fn key_value<'a>(key: Value<'a>, value: Value<'a>) -> (Value<'a>, Value<'a>) {
    (key, value)
}

pub fn map<'a>(pairs: &'a [(Value<'a>, Value<'a>)]) -> Value<'a> {
    Value::from_map(pairs)
}

pub fn indefinite_map<'a>(pairs: &'a [(Value<'a>, Value<'a>)]) -> Value<'a> {
    Value::from_indefinite_map(pairs)
}
