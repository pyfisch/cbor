#![cfg(feature = "std")]
use crate::serialize::owned;
use crate::test_utils::assert_value_owned;

#[test]
fn tag_simple() {
    // Tag with date.
    let v = owned::usmall(0);
    let value = owned::tag(2, v);

    assert_value_owned(value, "c200")
}

#[test]
fn tag_recursive() {
    // Tag with date.
    let v = owned::usmall(0);
    let v1 = owned::tag(2, v);
    let value = owned::tag(55799, v1);

    assert_value_owned(value, "d9d9f7c200")
}
