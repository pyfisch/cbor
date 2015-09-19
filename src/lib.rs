//! CBOR and serialization.
//!
//! # What is CBOR?
//! [CBOR](http://cbor.io) is a way to enode data in a space-efficient and fast binary format.
//! CBORs data-model is a superset of the JSONs.
//!
//! A simple object describing a person in diagnostic notation (it is actually JSON plus some
//! annotations) looks like
//!
//! ```json
//! {
//!     "FirstName": "John",
//!     "LastName": "Doe",
//!     "Age": 43,
//!     "Address": {
//!         "Street": "Downing Street 10",
//!         "City": "London",
//!         "Country": "Great Britain"
//!     },
//!     "PhoneNumbers": [
//!         "+44 1234567",
//!         "+44 2345678"
//!     ]
//! }
//! ```
//!
//! The CBOR encoded object with comments in hexadecimal notation looks like
//!
//! ```cbor
//! a5                                      # map(5)
//!    69                                   # text(9)
//!       46697273744e616d65                # "FirstName"
//!    64                                   # text(4)
//!       4a6f686e                          # "John"
//!    68                                   # text(8)
//!       4c6173744e616d65                  # "LastName"
//!    63                                   # text(3)
//!       446f65                            # "Doe"
//!    63                                   # text(3)
//!       416765                            # "Age"
//!    18 2b                                # unsigned(43)
//!    67                                   # text(7)
//!       41646472657373                    # "Address"
//!    a3                                   # map(3)
//!       66                                # text(6)
//!          537472656574                   # "Street"
//!       71                                # text(17)
//!          446f776e696e6720537472656574203130 # "Downing Street 10"
//!       64                                # text(4)
//!          43697479                       # "City"
//!       66                                # text(6)
//!          4c6f6e646f6e                   # "London"
//!       67                                # text(7)
//!          436f756e747279                 # "Country"
//!       6d                                # text(13)
//!          4772656174204272697461696e     # "Great Britain"
//!    6c                                   # text(12)
//!       50686f6e654e756d62657273          # "PhoneNumbers"
//!    82                                   # array(2)
//!       6b                                # text(11)
//!          2b34342031323334353637         # "+44 1234567"
//!       6b                                # text(11)
//!          2b34342032333435363738         # "+44 2345678"
//! ```
//! While the JSON encoding is 174 bytes long the CBOR representation is only 141 bytes long.
//! This is 19% shorter! Sometimes compression will even better, but never CBOR will be longer
//! than the corresponding JSON. More importantly CBOR supports binary data, custom data tyes,
//! annotations for dates, times and expected encoding and is faster to serialize and deserialize.
//! It can even be used on embedded devices.
//!
//! # Type-based Serialization and Deserialization
//! Serde provides a mechanism for low boilerplate serialization & deserialization of values to and
//! from CBOR via the serialization API. To be able to serialize a piece of data, it must implement
//! the serde::Serialize trait. To be able to deserialize a piece of data, it must implement the
//! serde::Deserialize trait. Serde provides provides an annotation to automatically generate the
//! code for these traits: #[derive(Serialize, Deserialize)].
//!
//! The CBOR API also provides an enum serde_cbor::Value
//!
//! # Examples
//! Read a CBOR value that is known to be a map of string keys to string values and print it.
//!
//! ```rust
//! use std::collections::HashMap;
//! use serde_cbor::from_slice;
//!
//! let slice = b"\xa5aaaAabaBacaCadaDaeaE";
//! let value: HashMap<String, String> = from_slice(slice).unwrap();
//! println!("{:?}", value); // {"e": "E", "d": "D", "a": "A", "c": "C", "b": "B"}
//! ```
//!
//! Read a general CBOR value with an unknown content.
//!
//! ```rust
//! use serde_cbor::{from_slice, Value};
//!
//! let slice = b"\x82\x01\xa1aaab";
//! let value: Value = from_slice(slice).unwrap();
//! println!("{:?}", value); // Array([U64(1), Object({String("a"): String("b")})])
//! ```
//!
//! Serialize an object.
//!
//! ```rust
//! use std::collections::HashMap;
//! use serde_cbor::to_vec;
//!
//! let mut programming_languages = HashMap::new();
//! programming_languages.insert("rust", vec!["safe", "concurrent", "fast"]);
//! programming_languages.insert("python", vec!["powerful", "friendly", "open"]);
//! programming_languages.insert("js", vec!["lightweight", "interpreted", "object-oriented"]);
//! let encoded = to_vec(&programming_languages);
//! ```

extern crate byteorder;
extern crate libc;
extern crate serde;

pub use de::{from_slice, from_reader};
pub use error::{Error, Result};
pub use ser::{to_vec, to_writer, to_vec_sd, to_writer_sd};
pub use value::{Value, ObjectKey};

pub mod de;
pub mod error;
pub mod read;
pub mod ser;
pub mod value;
