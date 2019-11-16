use std::error::Error;
use std::fs::File;
use std::io::Cursor;
use serde::ser::{Serialize, Serializer};
use serde_cbor::{Value, serialize_cbor_tagged};

#[derive(Debug, PartialEq)]
struct Cid(Vec<u8>);

impl Serialize for Cid {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // we could get this infix, but not sure if it is worth it...
        serialize_cbor_tagged(s, 42, &self.0)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("examples/test.cbor")?;
    let mut bytes = Vec::new();
    std::io::copy(&mut file, &mut bytes)?;
    println!("{:?}", bytes);
    let ast: serde_cbor::Value = serde_cbor::from_reader(Cursor::new(bytes))?;
    println!("{:?}", ast);
    let bytes1 = serde_cbor::to_vec(&Value::Tag(42, Box::new(Value::Bytes(Vec::new()))))?;
    // let bytes = serde_cbor::to_vec(&Value::Null)?;
    println!("{:?}", bytes1);


    let bytes2 = serde_cbor::to_vec(&Cid(Vec::new()))?;
    println!("{:?}", bytes2);

    let ast1: Value = serde_cbor::from_slice(&bytes1)?;
    println!("{:?}", ast1);

    let ast2: Value = serde_cbor::from_slice(&bytes1)?;
    println!("{:?}", ast2);

    Ok(())
}
