extern crate serde_cbor;
extern crate serde_json;

use serde_json::Value;

use std::io::{stdin, stdout, BufReader, BufWriter};


fn main() {
    let data: Value = serde_json::from_reader(BufReader::new(stdin())).unwrap();
    serde_cbor::to_writer(&mut BufWriter::new(stdout()), &data).unwrap();
}
