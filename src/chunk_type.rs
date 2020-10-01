use crate::{Error, Result};
use std::convert::{TryFrom, TryInto};

/// A 4-byte array
type RawChunkType = [u8; 4];

/// Parse 4-byte type codes which are described in the specifications of PNG files
/// ([PNG Structure](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html)).
///
/// Only valid chunk type code can be parsed.
///
/// The type code must consist only of upper-/lower-case alphabetic ASCII characters.
/// Besides representing the type of the chunk, a type code also represents four
/// properties through the used of upper-/lower-case ASCII alphabetic characters
/// so that PNG decoder can make decision upon encountering invalid type codes.
///
/// # Examples
///
/// ```rust
/// # use std::error::Error;
/// # use pmsg::ChunkType;
/// # use std::convert::TryFrom;
/// #
/// # fn main() -> Result<(), Box<dyn Error>> {
///     let raw = b"bLOb";
///     let chunk_type = ChunkType::try_from(*raw)?;
///     Ok(())
/// # }
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType(RawChunkType);

impl ChunkType {
    /// Return the 4-byte array that was parsed.
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use pmsg::ChunkType;
    /// # use std::convert::TryFrom;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    ///     let raw = b"bLOb";
    ///     let chunk_type = ChunkType::try_from(*raw)?;
    ///     assert_eq!(*raw, chunk_type.bytes());
    /// #   Ok(())
    /// # }
    /// ```
    pub fn bytes(&self) -> RawChunkType {
        self.0
    }

    /// A chunk is critical if the first byte is an uppercase
    /// ASCII letter, i.e., the fifth bit of the first byte is zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use pmsg::ChunkType;
    /// # use std::convert::TryFrom;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    ///     let raw = b"bLOb";
    ///     let chunk_type = ChunkType::try_from(*raw)?;
    ///     assert!(!chunk_type.is_critical());
    /// #   Ok(())
    /// # }
    /// ```
    pub fn is_critical(&self) -> bool {
        self.0[0] >> 5 & 1 == 0
    }

    /// A chunk is public if the second byte is a lowercase
    /// ASCII letter, i.e., the fifth bit of the second byte is zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use pmsg::ChunkType;
    /// # use std::convert::TryFrom;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    ///     let raw = b"bLOb";
    ///     let chunk_type = ChunkType::try_from(*raw)?;
    ///     assert!(chunk_type.is_public());
    /// #   Ok(())
    /// # }
    /// ```
    pub fn is_public(&self) -> bool {
        self.0[1] >> 5 & 1 == 0
    }

    /// The reserved bit is valid if the third byte is an uppercase
    /// ASCII letter, i.e., the fifth bit of the third byte is zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use pmsg::ChunkType;
    /// # use std::convert::TryFrom;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    ///     let raw = b"bLOb";
    ///     let chunk_type = ChunkType::try_from(*raw)?;
    ///     assert!(chunk_type.is_reserved_bit_valid());
    /// #   Ok(())
    /// # }
    /// ```
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.0[2] >> 5 & 1 == 0
    }

    /// A chunk is safe to copy if the fourth byte is a lowercase
    /// ASCII letter, i.e., the fifth bit of the forth byte is one.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use pmsg::ChunkType;
    /// # use std::convert::TryFrom;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    ///     let raw = b"bLOb";
    ///     let chunk_type = ChunkType::try_from(*raw)?;
    ///     assert!(chunk_type.is_safe_to_copy());
    /// #   Ok(())
    /// # }
    /// ```
    pub fn is_safe_to_copy(&self) -> bool {
        self.0[3] >> 5 & 1 == 1
    }

    /// A chunk is valid if the reserved bit is valid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use std::error::Error;
    /// # use pmsg::ChunkType;
    /// # use std::convert::TryFrom;
    /// #
    /// # fn main() -> Result<(), Box<dyn Error>> {
    ///     let raw = b"bLOb";
    ///     let chunk_type = ChunkType::try_from(*raw)?;
    ///     assert!(chunk_type.is_valid());
    /// #   Ok(())
    /// # }
    /// ```
    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
}

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

impl std::str::FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let raw: RawChunkType = s.as_bytes().try_into()?;
        ChunkType::try_from(raw)
    }
}

impl std::convert::TryFrom<RawChunkType> for ChunkType {
    type Error = Error;

    fn try_from(raw: RawChunkType) -> Result<Self> {
        if !raw
            .iter()
            .all(|&b| b >= 65 && b <= 90 || b >= 97 && b <= 122)
        {
            return Err(Self::Error::InvalidChunkType);
        }

        Ok(Self(raw))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
