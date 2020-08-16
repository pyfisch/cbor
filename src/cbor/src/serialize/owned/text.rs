use crate::serialize::owned::OwnedValue;

pub fn text(string: &str) -> OwnedValue {
    OwnedValue::from_text(string)
}

pub fn indefinite_text<S: ToString>(chunks: &[S]) -> OwnedValue {
    OwnedValue::from_indefinite_text(chunks)
}
