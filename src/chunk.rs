use crate::chunk_type::ChunkType;
use crate::{Error, Result};
use crc::{crc32, Hasher32};
use std::io::Read;

/// Parse a chunk from bytes as described by the specifications of PNG files
/// ([PNG Structure](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html)).
///
/// A chunk consists of four pieces of data which are the length of the data in the
/// chunk, the type code, the chunk data, and the checksum of the chunk.
/// The chunk length is a four-byte unsigned integer in big-endian order, although
/// it is an unsigned 32-bit integer, the largest possible value is only 2^31.
/// The CRC checksum is computed from the chunk type bytes and chunk data bytes
/// with the IEEE CRC32 polynomial using the methods described in ISO-3309.
#[derive(Debug)]
pub struct Chunk {
    length: u32, // NOTE: this must not exceed 2^31
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    /// Get the length of the data contained in the chunk
    pub fn length(&self) -> u32 {
        self.length
    }

    /// Get the parsed type code of the chunk
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    /// Get the data of the chunk in raw bytes
    pub fn data(&self) -> &[u8] {
        &self.chunk_data
    }

    /// Get the CRC checksum value of the chunk
    pub fn crc(&self) -> u32 {
        self.crc
    }

    /// Get the data of the chunk encoded as an UTF-8 string
    pub fn data_as_string(&self) -> Result<String> {
        Ok(String::from_utf8(self.chunk_data.clone())?)
    }

    /// Get the whole chunk in bytes
    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.chunk_data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
}

impl std::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}\"{}\"",
            self.chunk_type,
            String::from_utf8_lossy(&self.chunk_data)
        )
    }
}

impl std::convert::TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(raw: &[u8]) -> Result<Chunk> {
        let mut r = std::io::BufReader::new(raw);
        let mut buf = [0u8; 4];

        // parse chunk length
        r.read_exact(&mut buf)?;
        let length = u32::from_be_bytes(buf);
        if length > 1 << 31 {
            return Err(Error::InvalidChunkLength);
        }

        // parse chunk type
        r.read_exact(&mut buf)?;
        let chunk_type = ChunkType::try_from(buf)?;

        // parse chunk data
        let mut chunk_data = vec![0; length as usize];
        r.read_exact(&mut chunk_data)?;

        // creating checksum from received data
        let mut digest = crc32::Digest::new(crc32::IEEE);
        digest.write(&buf);
        digest.write(&chunk_data);

        // parse and check chunk checksum
        r.read_exact(&mut buf)?;
        let crc = u32::from_be_bytes(buf);
        if digest.sum32() != crc {
            return Err(Error::InvalidCRC);
        }

        Ok(Chunk {
            length,
            chunk_type,
            chunk_data,
            crc,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = b"RuSt";
        let message_bytes = b"This is where your secret message will be!";
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = b"RuSt";
        let message_bytes = b"This is where your secret message will be!";
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = b"RuSt";
        let message_bytes = b"This is where your secret message will be!";
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = b"RuSt";
        let message_bytes = b"This is where your secret message will be!";
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
