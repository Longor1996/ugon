//! Input for UGON deserializer.
//! 
//! Taken, with slight adjustments, from https://docs.rs/serde_json/latest/src/serde_json/read.rs.html

use crate::span::Span;
use crate::error::{Error, Result};

/// Trait used by the deserializer to consume input.
trait Read<'de>: private::Sealed {
    /// Read the next byte from the input.
    fn next(&mut self) -> Result<Option<u8>>;
    
    /// Peek the next byte from the input.
    fn peek(&mut self) -> Result<Option<u8>>;
    
    /// Discard the last [`Self::peek`]-ed byte.
    fn discard(&mut self);
    
    /// Returns the offset of the next byte to be returned by [`Self::peek`] or [`Self::next`].
    fn offset(&self) -> usize;
    
    /// Position, and optionally the limit, of the most recent call to [`Self::next`]
    fn position(&self) -> Span;
    
    /// Parse a string from the input, scanning until a string terminating tag is read.
    fn parse_terminated_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>>;
    
    /// Parse a string from the input, scanning until the given length is reached.
    fn parse_sized_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>, length: usize) -> Result<Reference<'de, 's, str>>;
    
    /// Read a sequence of bytes with a known length from the input.
    fn parse_sized_bytes<'s>(&'s mut self, scratch: &'s mut Vec<u8>, length: usize) -> Result<Reference<'de, 's, [u8]>>;
}

//////////////////////////////////////////////////////////////////////////////

/// Like [`core::borrow::Cow`], but with complex lifetimes.
pub enum Reference<'b, 'c, T>
where
    T: ?Sized + 'static,
{
    /// Data borrowed directly from the input.
    Borrowed(&'b T),
    
    /// Data copied from input into scratch buffer.
    Copied(&'c T),
}

impl<'b, 'c, T> core::ops::Deref for Reference<'b, 'c, T>
where
    T: ?Sized + 'static,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match *self {
            Reference::Borrowed(b) => b,
            Reference::Copied(c) => c,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////

// Prevent users from implementing the Read trait.
mod private {
    pub trait Sealed {}
}

//////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "std")]
mod ioread {
    //! Input source that reads from a [`io::Read`]-object.
    use crate::io;
    use super::*;
    
    /// Input source that reads from a [`io::Read`]-object.
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    pub struct ReadFromIo<R> where R: io::Read {
        iter: io::Bytes<R>,
        peek: Option<u8>,
        seek: usize
    }
    
    impl<R> ReadFromIo<R>
    where
        R: io::Read,
    {
        /// Create a input source to read from a [`io::Read`]-object.
        pub fn new(reader: R) -> Self {
            ReadFromIo {
                iter: reader.bytes(),
                peek: None,
                seek: 0
            }
        }
    }

    #[cfg(feature = "std")]
    impl<R> super::private::Sealed for ReadFromIo<R> where R: io::Read {}

    #[cfg(feature = "std")]
    impl<R> ReadFromIo<R>
    where
        R: io::Read,
    {
        //
    }


    #[cfg(feature = "std")]
    impl<'de, R> Read<'de> for ReadFromIo<R>
    where
        R: io::Read,
    {
        #[inline]
        fn next(&mut self) -> Result<Option<u8>> {
            if let Some(peek) = self.peek.take() {
                return Ok(Some(peek))
            }
            
            match self.iter.next() {
                Some(Err(err)) => Err(Error::io(err)),
                Some(Ok(next)) => {
                    self.seek += 1;
                    Ok(Some(next))
                },
                None => Ok(None)
            }
        }
        
        #[inline]
        fn peek(&mut self) -> Result<Option<u8>> {
            match self.peek {
                Some(peek) => Ok(Some(peek)),
                None => match self.iter.next() {
                    Some(Err(err)) => Err(Error::io(err)),
                    Some(Ok(next)) => {
                        self.seek += 1;
                        self.peek = Some(next);
                        Ok(self.peek)
                    }
                    None => Ok(None)
                }
            }
        }
        
        fn discard(&mut self) {
            self.peek = None;
        }
        
        fn offset(&self) -> usize {
            self.seek
        }
        
        fn position(&self) -> Span {
            self.seek.into()
        }
        
        fn parse_terminated_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
            todo!()
        }

        fn parse_sized_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>, length: usize) -> Result<Reference<'de, 's, str>> {
            todo!()
        }

        fn parse_sized_bytes<'s>(&'s mut self, scratch: &'s mut Vec<u8>, length: usize) -> Result<Reference<'de, 's, [u8]>> {
            todo!()
        }
        //
    }

}

//////////////////////////////////////////////////////////////////////////////

mod u8read {
    use super::*;
    
    /// Input source that reads from a [`u8`]-slice.
    pub struct ReadFromSlice<'s> {
        slice: &'s [u8],
        index: usize,
    }
    
    impl<'a> ReadFromSlice<'a> {
        /// Create a JSON input source to read from a slice of bytes.
        pub fn new(slice: &'a [u8]) -> Self {
            ReadFromSlice {
                slice,
                index: 0
            }
        }
    }
    
    impl<'a> private::Sealed for ReadFromSlice<'a> {}
    
    impl<'a> Read<'a> for ReadFromSlice<'a> {
        #[inline]
        fn next(&mut self) -> Result<Option<u8>> {
            // `Ok(self.slice.get(self.index).map(|ch| { self.index += 1; *ch }))`
            // is about 10% slower.
            Ok(if self.index < self.slice.len() {
                let by = self.slice[self.index];
                self.index += 1;
                Some(by)
            } else {
                None
            })
        }
        
        #[inline]
        fn peek(&mut self) -> Result<Option<u8>> {
            // `Ok(self.slice.get(self.index).map(|ch| *ch))` is about 10% slower
            // for some reason.
            Ok(if self.index < self.slice.len() {
                Some(self.slice[self.index])
            } else {
                None
            })
        }
        
        #[inline]
        fn discard(&mut self) {
            self.index += 1;
        }
        
        #[inline]
        fn offset(&self) -> usize {
            self.index
        }
        
        fn position(&self) -> Span {
            self.index.into()
        }
        
        fn parse_terminated_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'a, 's, str>> {
            todo!()
        }
        
        fn parse_sized_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>, length: usize) -> Result<Reference<'a, 's, str>> {
            todo!()
        }
        
        fn parse_sized_bytes<'s>(&'s mut self, scratch: &'s mut Vec<u8>, length: usize) -> Result<Reference<'a, 's, [u8]>> {
            todo!()
        }
    }
}
