use serde_cbor::Value;
use std::error::Error;
use std::fs::File;
use std::io::Cursor;

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("examples/test.cbor")?;
    let mut bytes = Vec::new();
    std::io::copy(&mut file, &mut bytes);
    println!("{:?}", bytes);
    let ast: serde_cbor::Value = serde_cbor::from_reader(Cursor::new(bytes))?;
    println!("{:?}", ast);
    let bytes = serde_cbor::to_vec(&Value::Tag(42, Box::new(Value::Null)))?;
    // let bytes = serde_cbor::to_vec(&Value::Null)?;
    println!("{:?}", bytes);

    Ok(())
}
