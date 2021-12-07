use crc32fast::Hasher;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Hash40(u64);

impl Hash40 {
    fn into_hasher(self) -> Hasher {
        Hasher::new_with_initial_len(self.crc32(), self.len() as u64)
    }

    fn new_from_parts(crc32: u32, len: u64) -> Self {
        let raw_u64 = (len << 32) | (crc32 as u64);
        Self(raw_u64)
    }

    pub fn new<S: AsRef<str>>(s: S) -> Self {
        let s = s.as_ref();
        let mut hasher = Hasher::new();
        hasher.update(s.as_bytes());
        let checksum = hasher.finalize();
        
        Self::new_from_parts(checksum, s.len() as u64)
    }

    pub fn new_raw(val: u64) -> Self {
        Self(val)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }

    pub fn crc32(self) -> u32 {
        (self.0 & 0xFFFF_FFFF) as u32
    }

    pub fn len(self) -> usize {
        ((self.0 & 0xFF_0000_0000) >> 32) as usize
    }

    pub fn concat<H: Into<Hash40>>(self, h: H) -> Self {
        let h: Hash40 = h.into();
        let mut result = self.into_hasher();
        let other = h.into_hasher();
        result.combine(&other);
        Self::new_from_parts(result.finalize(), (self.len() + h.len()) as u64)
    }
}

impl<S: AsRef<str>> From<S> for Hash40 {
    fn from(other: S) -> Self {
        Hash40::new(other)
    }
}

impl fmt::Display for Hash40 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}