use crate::deserialize::peek;
use crate::serialize::owned;
use crate::test_utils::assert_peek;

#[test]
fn text_simple() {
    let string = "Hello World";
    assert_peek(owned::text(string), "6b48656c6c6f20576f726c64", peek::text);
}
