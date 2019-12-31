#[macro_use]
extern crate serde_derive;

use serde_cbor;
use serde_cbor::de;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct MyStuff {
    #[serde(flatten)]
    data: MyStuffType,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum MyStuffType {
    ver1 {
        x: f64,
    },
    ver2
}

#[test]
/// Test roundtrip operation on a serde data structure.
fn test_roundtrip() {
    let stuff1 = MyStuff {
        data: MyStuffType::ver1 {
            x: 2.5
        }
    };
    let data_bytes = serde_cbor::to_vec(&stuff1).unwrap();
    let stuff2 = serde_cbor::from_slice(&data_bytes).unwrap();
    assert_eq!(stuff1, stuff2);
}
