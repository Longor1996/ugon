//! Location within the inputs byte sequence.

use core::num::NonZeroUsize;
use core::fmt::{self, Debug, Display};

/// Location within the inputs byte sequence.
#[derive(Clone, Copy)]
pub struct Span {
    start: usize,
    end: Option<NonZeroUsize>
}

impl Span {
    /// Returns the start offset.
    pub fn start(&self) -> usize {
        self.start
    }
    
    /// Returns the end offset as `Option<NonZeroUsize>`.
    pub fn end(&self) -> Option<NonZeroUsize> {
        self.end
    }
    
    /// Returns the end offset as `usize`.
    pub fn end_or_zero(&self) -> usize {
        self.end.map(|e| e.get()).unwrap_or(0)
    }
}

impl Default for Span {
    fn default() -> Self {
        Self {
            start: 0,
            end: None
        }
    }
}

impl From<usize> for Span {
    fn from(start: usize) -> Self {
        Self {start, end: None}
    }
}

impl From<(usize, usize)> for Span {
    fn from((start, end): (usize, usize)) -> Self {
        assert!(start <= end);
        Self {
            start,
            end: NonZeroUsize::new(end)
        }
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>4X}", self.start)?;
        
        if let Some(end) = self.end {
            write!(f, ":{:0>4X}", end)?;
            let len = self.start - end.get();
            if len > 0 {write!(f, ":{:X}", len)?;}
        }
        
        Ok(())
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {write!(f, "Span(")?;}
        
        write!(f, "start: {:#04X}", self.start)?;
        
        if let Some(end) = self.end {
            write!(f, ", end: {:#04X}", end)?;
            let len = self.start - end.get();
            if len > 0 {write!(f, ", len: {:#04X}", len)?;}
        }
        
        if f.alternate() {write!(f, ")")?;}
        Ok(())
    }
}
