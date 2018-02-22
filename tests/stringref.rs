extern crate serde_cbor;
#[macro_use] extern crate pretty_assertions;

use serde_cbor::{Value, error, de};

#[test]
fn test_doc_example1() {
    use serde_cbor::ObjectKey::String as key;
    // http://cbor.schmorp.de/stringref#EXAMPLES
    let value: error::Result<Value> = de::from_slice(&[
       0xd9, 0x01, 0x00,            // tag(256)
          0x83,                     // array(3)
             0xa3,                  // map(3)
                0x64,               // text(4)
                   0x72, 0x61,
                   0x6e, 0x6b,      // "rank"
                0x04,               // unsigned(4)
                0x65,               // text(5)
                   0x63, 0x6f,
                   0x75, 0x6e, 0x74,// "count"
                0x19, 0x01, 0xa1,   // unsigned(417)
                0x64,               // text(4)
                   0x6e, 0x61,
                   0x6d, 0x65,      // "name"
                0x68,               // text(8)
                   0x43, 0x6f, 0x63,
                   0x6b, 0x74, 0x61,
                   0x69, 0x6c,      // "Cocktail"
             0xa3,                  // map(3)
                0xd8, 0x19,         // tag(25)
                   0x02,            // unsigned(2)
                0x64,               // text(4)
                   0x42, 0x61,
                   0x74, 0x68,      // "Bath"
                0xd8, 0x19,         // tag(25)
                   0x01,            // unsigned(1)
                0x19, 0x01, 0x38,   // unsigned(312)
                0xd8, 0x19,         // tag(25)
                   0x00,            // unsigned(0)
                0x04,               // unsigned(4)
             0xa3,                  // map(3)
                0xd8, 0x19,         // tag(25)
                   0x02,            // unsigned(2)
                0x64,               // text(4)
                   0x46, 0x6f,
                   0x6f, 0x64,      // "Food"
                0xd8, 0x19,         // tag(25)
                   0x01,            // unsigned(1)
                0x19, 0x02, 0xb3,   // unsigned(691)
                0xd8, 0x19,         // tag(25)
                   0x00,            // unsigned(0)
                0x04,               // unsigned(4)
    ]);
    assert_eq!(value.unwrap(), Value::Array(vec![
        Value::Object(vec![
            (key("rank".into()), Value::U64(4)),
            (key("count".into()), Value::U64(417)),
            (key("name".into()), Value::String("Cocktail".into())),
        ].into_iter().collect()),
        Value::Object(vec![
            (key("rank".into()), Value::U64(4)),
            (key("count".into()), Value::U64(312)),
            (key("name".into()), Value::String("Bath".into())),
        ].into_iter().collect()),
        Value::Object(vec![
            (key("rank".into()), Value::U64(4)),
            (key("count".into()), Value::U64(691)),
            (key("name".into()), Value::String("Food".into())),
        ].into_iter().collect()),
    ]));
}
