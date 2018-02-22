extern crate serde;
extern crate serde_cbor;
extern crate serde_json;

use serde::Serialize;
use serde_json::Value;
use serde_cbor::Serializer;

use std::io::{stdin, stdout, BufReader, BufWriter};


fn main() {
    let data: Value = serde_json::from_reader(BufReader::new(stdin())).unwrap();
    data.serialize(&mut Serializer::with_stringref(
        &mut BufWriter::new(stdout()))).unwrap();
}
