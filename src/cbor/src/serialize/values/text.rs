use crate::serialize::values::Value;

pub fn text(string: &str) -> Value {
    Value::from_text(string)
}

pub fn indefinite_text<'a>(chunks: &'a [&'a str]) -> Value<'a> {
    Value::from_indefinite_text(chunks)
}
