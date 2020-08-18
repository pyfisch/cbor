#![cfg(all(feature = "std", feature = "half"))]
use crate::deserialize::peek;
use crate::serialize::values;
use crate::test_utils::assert_peek;

#[test]
fn simple() {
    assert_peek(values::r#true(), "f5", peek::r#true);
    assert_peek(values::r#false(), "f4", peek::r#false);
    assert_peek(values::null(), "f6", peek::null);
    assert_peek(values::undefined(), "f7", peek::undefined);
}

#[test]
fn floats() {
    assert_peek(values::half_float_from_f32(1.2), "f93ccd", peek::f16);
    assert_peek(values::f32(4.5), "fa40900000", peek::f32);
    assert_peek(values::f64(6.7), "fb401acccccccccccd", peek::f64);
}
