[![Build Status](https://travis-ci.org/pyfisch/cbor.svg?branch=master)](https://travis-ci.org/pyfisch/cbor)

This repository manages 2 packages; ll_cbor and serde_cbor.

# LL CBOR
[![Crates.io](https://img.shields.io/crates/v/serde_cbor.svg)](https://crates.io/crates/ll_cbor)
[![Documentation](https://docs.rs/serde_cbor/badge.svg)](https://docs.rs/ll_cbor)

LL CBOR is a low level CBOR serialization and deserialization API that allows people to
serialize and deserialize CBOR without any framework, in the Schema they want.

This crate comes from the need for developers to support CBOR values that aren't natively
supported by other serialization platform (like serde). An example of this is large integers
and dates; Serde does not support bit integer types or special object types, but CBOR has
them and an application might want to serialize or deserialize using those (or the full
CBOR spectrum, or custom made tags).

## Usage
LL CBOR supports Rust 1.38 and up. To install it add this to your `Cargo.toml`:
```toml
[dependencies]
ll_cbor = "0.1.0"
```

Then, for serializing values:

```rust
use ll_cbor::serialize::values as cbor;
use ll_cbor::serialize::builders;

fn main() -> Result<(), std::error::Error> {
    // Serialize a single u64.
    let some_value = cbor::u64(0);

    // Serialize a vector of u32.
    let some_vector = cbor::vector(vec![cbor::u32(1), cbor::u32(2), cbor::u32(3)]);

    // Serialize a map of variable values.
    // This is a HashMap<ll_cbor::Value, ll_cbor::Value>.
    let hash = std::collections::HashMap::new();
    hash.insert(cbor::string("hello"), cbor::i8(-100));
    // It is legal in CBOR to have different type of keys in maps, but impossible to represent
    // natively with Serde.
    hash.insert(cbor::u32(1), cbor::string("World"));
    let some_map = cbor::dictionary(&hash);

    // We can also just pass in bytes and get an untrusted ll_cbor::Value from it:
    let value = ll_cbor::Value::from_untrusted_slice(&[1, 2, 3]);

    // When we don't know how many objects in advance, we can use a builder.
    let some_map_builder = builders::dictionary();
    for i in 0..1000 {
        // It is also possible in CBOR to have multiple values with the same key.
        some_map_builder.insert(cbor::string("key"), cbor::u32(i));
    }

    // Adding a CBOR tag to it.
    let some_map2 = cbor::tag(55799, some_map_builder.build());

    // Getting the bytes for the second map.
    println("{}", hex::encode(&some_map2));

    Ok(())
}
```

For deserialization, there are multiple ways. The main way is to use various `try_from`
to check if a byte stream is of the right type. You can also build a schema and validate
the input with it.

```rust
use ll_cbor::deserialize::values as cbor_de;
use ll_cbor::schema;

fn main() -> Result<(), std::error::Error> {
    let bytes: Vec<u8> = vec![1, 2, 3];

    // These will be of type Result<u64, ll_cbor::deserialize::Error>.
    let maybe_u64 = cbor_de::u64::try_from(&bytes);
    let maybe_string = cbor_de::string::try_from(&bytes);

    // A vector can contain any elements.
    let maybe_vec = cbor_de::vec::try_from(&bytes);

    // So we have to map and test all items.
    // TODO: correct the unwrap() calls with results.
    let maybe_vec_of_u32 = maybe_vec.map(|v| v.iter().map(|i| cbor_de::u32::try_from(i)).collect());

    // This will create a schema for a dictionary of string -> tag + i8.
    let s = schema::dictionary(schema::string, schema::tag(schema::i8));
    // Validate bytes match the schema.
    // This returns a `Result<BTreeMap<String, i8>, ll_cbor::schema::Error>`.
    let maybe_v = s.validate(&bytes);

    Ok(())
}
```


# Serde CBOR
[![Crates.io](https://img.shields.io/crates/v/serde_cbor.svg)](https://crates.io/crates/serde_cbor)
[![Documentation](https://docs.rs/serde_cbor/badge.svg)](https://docs.rs/serde_cbor)

This crate implements the Concise Binary Object Representation from [RFC 7049].
It builds on [Serde], the generic serialization framework for Rust.
CBOR provides a binary encoding for a superset
of the JSON data model that is small and very fast to parse.

[RFC 7049]: https://tools.ietf.org/html/rfc7049
[Serde]: https://github.com/serde-rs/serde

## Usage

Serde CBOR supports Rust 1.40 and up. Add this to your `Cargo.toml`:
```toml
[dependencies]
serde_cbor = "0.11.1"
```

Storing and loading Rust types is easy and requires only
minimal modifications to the program code.

```rust
use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;

// Types annotated with `Serialize` can be stored as CBOR.
// To be able to load them again add `Deserialize`.
#[derive(Debug, Serialize, Deserialize)]
struct Mascot {
    name: String,
    species: String,
    year_of_birth: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let ferris = Mascot {
        name: "Ferris".to_owned(),
        species: "crab".to_owned(),
        year_of_birth: 2015,
    };

    let ferris_file = File::create("examples/ferris.cbor")?;
    // Write Ferris to the given file.
    // Instead of a file you can use any type that implements `io::Write`
    // like a HTTP body, database connection etc.
    serde_cbor::to_writer(ferris_file, &ferris)?;

    let tux_file = File::open("examples/tux.cbor")?;
    // Load Tux from a file.
    // Serde CBOR performs roundtrip serialization meaning that
    // the data will not change in any way.
    let tux: Mascot = serde_cbor::from_reader(tux_file)?;

    println!("{:?}", tux);
    // prints: Mascot { name: "Tux", species: "penguin", year_of_birth: 1996 }

    Ok(())
}
```

There are a lot of options available to customize the format.
To operate on untyped CBOR values have a look at the `Value` type.

## License
Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
