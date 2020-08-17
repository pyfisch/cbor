use crate::deserialize::peek;
use crate::serialize::values;
use crate::test_utils::assert_peek_simple;

#[test]
fn text_simple() {
    let s = "Hello World!";
    assert_peek_simple(s, values::text, peek::text);
}
