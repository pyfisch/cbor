# Serde CBOR Serialization Library
This crate is a Rust library for parsing and generating the
[CBOR](http://cbor.io/) (Concise Binary Object Representation)
file format. It is built upon [Serde](https://github.com/serde-rs/serde),
a high performance generic serialization framework.

## About CBOR
CBOR is a binary encoding based on a superset of the JSON data model.
It supports all the standard JSON types plus binary data, big numbers, 
non-string keys, time values and custom data types using tagging of values.
CBOR is always shorter than the corresponding JSON representation and easier
and faster to parse.
