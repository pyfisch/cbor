pub enum DeserializeErrorKind {
    TooManyBytes { expected: usize },
}

pub mod peek;

#[cfg(test)]
mod proptest_test;
