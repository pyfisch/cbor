# Serde CBOR Serialization Library
[![Build Status](https://travis-ci.org/pyfisch/cbor.svg?branch=master)](https://travis-ci.org/pyfisch/cbor)
[![Coverage Status](https://coveralls.io/repos/pyfisch/cbor/badge.svg?branch=master&service=github)](https://coveralls.io/github/pyfisch/cbor?branch=master)

[Documentation](https://pyfisch.github.io/cbor/serde_cbor/)

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
