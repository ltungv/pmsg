const CHUNK_SIZE: usize = 4;

#[derive(Debug)]
struct ChunkType {
    bit_ancillary: bool,
    bit_private: bool,
    bit_reserved: bool,
    bit_safe_to_copy: bool,
    chunk: [u8; CHUNK_SIZE],
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.chunk
    }

    fn is_valid(&self) -> bool {
        todo!();
    }

    fn is_critical(&self) -> bool {
        !self.bit_ancillary
    }

    fn is_public(&self) -> bool {
        !self.bit_private
    }

    fn is_reserved_bit_valid(&self) -> bool {
        self.bit_reserved
    }

    fn is_safe_to_copy(&self) -> bool {
        self.bit_safe_to_copy
    }
}

impl PartialEq for ChunkType {
    fn eq(&self, other: &ChunkType) -> bool {
        self.chunk == other.chunk
    }
}

impl Eq for ChunkType {}

impl std::convert::TryFrom<[u8; CHUNK_SIZE]> for ChunkType {
    type Error = crate::Error;

    fn try_from(_: [u8; CHUNK_SIZE]) -> crate::Result<ChunkType> {
        todo!()
    }
}

impl std::str::FromStr for ChunkType {
    type Err = crate::Error;

    fn from_str(_: &str) -> crate::Result<ChunkType> {
        todo!()
    }
}

impl std::fmt::Display for ChunkType {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result {
        todo!()
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
