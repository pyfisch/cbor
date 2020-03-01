#![cfg(feature = "std")]
use crate::serialize::values;
use crate::test_utils::assert_value;

#[test]
fn text_small() {
    let string = "Hello World";

    assert_value(values::text(string), "6b48656c6c6f20576f726c64");
}

#[test]
fn text_445() {
    let string = concat!(
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do ",
        "eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, ",
        "quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis ",
        "aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla ",
        "pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia ",
        "deserunt mollit anim id est laborum.",
    );

    assert_value(
        values::text(string),
        concat!(
        "7901bd4c6f72656d20697073756d20646f6c6f722073697420616d65742c20636f6e736563746574757220",
        "61646970697363696e6720656c69742c2073656420646f20656975736d6f642074656d706f7220696e6369",
        "646964756e74207574206c61626f726520657420646f6c6f7265206d61676e6120616c697175612e205574",
        "20656e696d206164206d696e696d2076656e69616d2c2071756973206e6f73747275642065786572636974",
        "6174696f6e20756c6c616d636f206c61626f726973206e69736920757420616c6971756970206578206561",
        "20636f6d6d6f646f20636f6e7365717561742e2044756973206175746520697275726520646f6c6f722069",
        "6e20726570726568656e646572697420696e20766f6c7570746174652076656c697420657373652063696c",
        "6c756d20646f6c6f726520657520667567696174206e756c6c612070617269617475722e20457863657074",
        "6575722073696e74206f6363616563617420637570696461746174206e6f6e2070726f6964656e742c2073",
        "756e7420696e2063756c706120717569206f666669636961206465736572756e74206d6f6c6c697420616e",
        "696d20696420657374206c61626f72756d2e"),
    );
}
