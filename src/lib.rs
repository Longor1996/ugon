#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]

// References:
// - https://docs.rs/serde_json/latest
// - https://docs.rs/serde_json/latest/src/serde_json/lib.rs.html
// - https://docs.rs/postcard/latest
// - https://docs.rs/postcard/latest/src/postcard/lib.rs.html

// Core library:
pub mod io;
pub mod span;
pub mod error;
pub mod value;
pub mod consts;

// Input/Output 'streams':
pub mod input;
pub mod output;

/// UGON Implementation:
pub mod read;
pub mod write;

/// Macros for either building a [`crate::value`]-structure, filling a [`Vec<u8>`] or directly writing to a [`std::io::Write`].
#[cfg(feature = "std")] // re-evaluate once implemented
pub mod macros {
    pub mod build;
    pub mod fill;
    pub mod write;
}

// Re-exports
pub use error::{Error, Result};

// Serde interop, if enabled:

#[cfg(feature = "serde")]
mod ser;

#[cfg(feature = "serde")]
mod de;

#[cfg(feature = "serde")]
pub use ser::{to_slice, Serializer};

#[cfg(all(feature = "serde", any(feature = "std", feature = "alloc")))]
pub use ser::{to_vec, to_writer};

#[cfg(feature = "serde")]
pub use de::{from_slice, from_reader, Deserializer};

#[cfg(test)]
mod tests {
    //! Tests
}
