use crate::serialize::values::Value;

pub fn array<'a>(values: &'a [Value<'a>]) -> Value<'a> {
    Value::from_array(values)
}

pub fn indefinite_array<'a>(values: &'a [Value<'a>]) -> Value<'a> {
    Value::from_indefinite_array(values)
}
