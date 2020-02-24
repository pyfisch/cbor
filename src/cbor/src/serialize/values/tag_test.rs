#![cfg(feature = "std")]
use crate::serialize::values;
use crate::test_utils::assert_value;

#[test]
fn tag_simple() {
    // Tag with date.
    let v = values::usmall(0);
    let value = values::tag(2, &v);

    assert_value(value, "c200")
}

#[test]
fn tag_recursive() {
    // Tag with date.
    let v = values::usmall(0);
    let v1 = values::tag(2, &v);
    let value = values::tag(55799, &v1);

    assert_value(value, "d9d9f7c200")
}
