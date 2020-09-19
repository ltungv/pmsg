mod chunk;
mod chunk_type;
mod png;

pub use chunk::*;
pub use chunk_type::*;
pub use png::*;

/// The error type for operations on PNG files, and associated traits.
pub type Error = PMSGError;

/// The result type for operations on PNG files, and associated traits.
pub type Result<T> = std::result::Result<T, Error>;

/// List of possible errors when working with PNG files
#[derive(Debug)]
pub enum PMSGError {
    /// Value for of the chunk length exceeds 2^31.
    InvalidChunkLength,
    /// Chunk type code contains invalid byte.
    InvalidChunkType,
    /// The given checksum does not match the computed checksum.
    InvalidCRC,
    /// The given data for a PNG file contains an invalid header.
    InvalidPNGFileHeader,
    /// The first chunk of the PNG file is not valid.
    InvalidStartingChunk,
    /// The given chunk type can not be found from the png representation.
    ChunkTypeNotFound,
    /// Errors that occured with I/O operations.
    IOError(std::io::Error),
    /// Errors that occured with UTF-8 encoding.
    StringFromUtf8Error(std::string::FromUtf8Error),
    /// Errors that occured with converting slices to arrays.
    ArrayFromSliceError(std::array::TryFromSliceError),
    /// Errors that occured with converting between integer types.
    NumFromIntError(std::num::TryFromIntError),
}

impl std::error::Error for PMSGError {}

impl std::fmt::Display for PMSGError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PMSGError::InvalidChunkType => write!(f, "invalid chunk type"),
            PMSGError::InvalidChunkLength => write!(f, "invalid chunk length"),
            PMSGError::InvalidCRC => write!(f, "invalid CRC"),
            PMSGError::InvalidPNGFileHeader => write!(f, "invalid PNG file header"),
            PMSGError::InvalidStartingChunk => write!(f, "invalid starting chunk"),
            PMSGError::ChunkTypeNotFound => write!(f, "chunk type not found"),
            PMSGError::IOError(e) => write!(f, "{}", e),
            PMSGError::StringFromUtf8Error(e) => write!(f, "{}", e),
            PMSGError::ArrayFromSliceError(e) => write!(f, "{}", e),
            PMSGError::NumFromIntError(e) => write!(f, "{}", e),
        }
    }
}

impl std::convert::From<std::io::Error> for PMSGError {
    fn from(e: std::io::Error) -> PMSGError {
        PMSGError::IOError(e)
    }
}

impl std::convert::From<std::string::FromUtf8Error> for PMSGError {
    fn from(e: std::string::FromUtf8Error) -> PMSGError {
        PMSGError::StringFromUtf8Error(e)
    }
}

impl std::convert::From<std::array::TryFromSliceError> for PMSGError {
    fn from(e: std::array::TryFromSliceError) -> PMSGError {
        PMSGError::ArrayFromSliceError(e)
    }
}

impl std::convert::From<std::num::TryFromIntError> for PMSGError {
    fn from(e: std::num::TryFromIntError) -> PMSGError {
        PMSGError::NumFromIntError(e)
    }
}
