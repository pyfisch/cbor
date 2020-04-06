pub enum DeserializeErrorKind {
    TooManyBytes { expected: usize },
}

mod numbers;
pub use numbers::*;

// /// Generic function to peek values off a buffer and generate an [Option<T>].
// /// Note that T will be None if there are not enough bytes to generate it.
// pub fn peek_or_default<T>(bytes: &[u8]) -> T {}
