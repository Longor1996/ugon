//! Error type used by both serializer and deserializer.

use core::result;
use core::fmt::{self, Debug, Display};

#[cfg(feature = "std")]
use std::io;

/// This type represents all possible errors that could occur while de/ser-ializing UGON data.
pub struct Error {
    err: Box<BoxedError>
}

/// Alias for a `Result` with the error type `serde_json::Error`.
pub type Result<T> = result::Result<T, Error>;

struct BoxedError {
    code: ErrorCode,
    span: crate::span::Span,
}

/// Error code.
#[non_exhaustive]
pub(crate) enum ErrorCode {
    /// Catchall
    Any(Box<str>),
    
    Io(std::io::Error)
}

impl Error {
    // Creates an [`Error`] from an [`io::Error`].
    #[cold]
    pub(crate) fn io(error: io::Error) -> Self {
        Error {
            err: Box::new(BoxedError {
                code: ErrorCode::Io(error),
                span: Default::default()
            }),
        }
    }
    
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} at {}",
            (),
            self.err.span
        )
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error({:?}, {:?}",
            (),
            self.err.span
        )
    }
}