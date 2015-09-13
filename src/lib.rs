#![feature(append)]
#![feature(read_exact)]
#![feature(float_extras)]

extern crate byteorder;
extern crate serde;

pub use value::{Value, ObjectKey};
pub use ser::{to_vec, to_writer};
pub use de::from_slice;

pub mod ser;
pub mod de;
pub mod error;
pub mod value;
