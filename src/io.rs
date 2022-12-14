//! `no_std` facade.
//! 
//! Taken from https://docs.rs/serde_json/latest/src/serde_json/io/mod.rs.html

pub use self::imp::{Error, ErrorKind, Result, Write};

#[cfg(not(feature = "std"))]
#[path = "core.rs"]
mod imp;

#[cfg(feature = "std")]
use std::io as imp;

#[cfg(feature = "std")]
pub use std::io::{Bytes, Read};
