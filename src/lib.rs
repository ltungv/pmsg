mod chunk;
mod chunk_type;
pub use chunk::*;
pub use chunk_type::*;

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
    /// Errors that occured with I/O operations.
    IOError(std::io::Error),
    /// Errors that occured with UTF-8 encoding.
    StringFromUtf8Error(std::string::FromUtf8Error),
    /// Errors that occured with converting slices to arrays.
    ArrayFromSliceError(std::array::TryFromSliceError),
}

impl std::error::Error for PMSGError {}

impl std::fmt::Display for PMSGError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PMSGError::InvalidChunkType => write!(f, "invalid chunk type"),
            PMSGError::InvalidChunkLength => write!(f, "invalid chunk length"),
            PMSGError::InvalidCRC => write!(f, "invalid CRC"),
            PMSGError::IOError(e) => write!(f, "{}", e),
            PMSGError::StringFromUtf8Error(e) => write!(f, "{}", e),
            PMSGError::ArrayFromSliceError(e) => write!(f, "{}", e),
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
