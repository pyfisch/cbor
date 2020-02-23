#[cfg(not(feature = "std"))]
use std::fmt::{Debug, Display};

/// This type is replaced with [std::error::Error] in std feature environments,
/// but also has a definition in no_std.
/// This provides the core trait for all errors in this crate.
#[cfg(feature = "std")]
pub use std::error::Error;

#[cfg(not(feature = "std"))]
pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
