use crate::{Error, Result};
use std::convert::{TryFrom, TryInto};

type RawChunkType = [u8; 4];

#[derive(Debug)]
pub struct ChunkType(RawChunkType);

impl ChunkType {
    pub fn bytes(&self) -> RawChunkType {
        self.0
    }

    pub fn is_critical(&self) -> bool {
        self.0[0] >> 5 & 1 == 0
    }

    pub fn is_public(&self) -> bool {
        self.0[1] >> 5 & 1 == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.0[2] >> 5 & 1 == 0
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.0[3] >> 5 & 1 == 1
    }

    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
}

impl Eq for ChunkType {}

impl PartialEq for ChunkType {
    fn eq(&self, other: &ChunkType) -> bool {
        self.0 == other.0
    }
}

impl std::convert::TryFrom<RawChunkType> for ChunkType {
    type Error = Error;

    fn try_from(chunk: RawChunkType) -> Result<ChunkType> {
        if chunk
            .iter()
            .any(|&b| !b.is_ascii_lowercase() && !b.is_ascii_uppercase())
        {
            return Err(Error::InvalidChunkTypeCode);
        }

        Ok(ChunkType(chunk))
    }
}

impl std::str::FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<ChunkType> {
        let chunk: std::result::Result<RawChunkType, _> = s.as_bytes().try_into();
        match chunk {
            Err(_) => Err(Error::InvalidChunkTypeCode),
            Ok(chunk) => ChunkType::try_from(chunk),
        }
    }
}

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", unsafe {
            // only valid chunk type code can be created
            std::str::from_utf8_unchecked(&self.0)
        })
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
