#![cfg(feature = "std")]
use crate::deserialize::peek;
use crate::serialize::values;
use crate::test_utils::assert_peek;

#[test]
fn tag_simple() {
    let v = values::usmall(0);
    let value = values::tag(2, v);

    assert_peek(value, "c200", peek::tag)
}

#[test]
fn self_describe() {
    // Tag with date.
    let v = values::usmall(0);
    let value = values::self_describe(v);

    assert_peek(value, "d9d9f700", peek::tag)
}

#[test]
fn tag_recursive() {
    // Tag with date.
    let v = values::usmall(0);
    let v1 = values::tag(2, v);
    let value = values::tag(55799, v1);

    assert_peek(value, "d9d9f7c200", peek::tag)
}
