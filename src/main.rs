mod chunk;
mod chunk_type;

fn main() {
    todo!()
}

pub type Error = PNGMeError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum PNGMeError {
    InvalidChunkLength,
    InvalidChunkType,
    InvalidCRC,
    IOError(std::io::Error),
    StringFromUtf8Error(std::string::FromUtf8Error),
    ArrayFromSliceError(std::array::TryFromSliceError),
}

impl std::error::Error for PNGMeError {}

impl std::fmt::Display for PNGMeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PNGMeError::InvalidChunkType => write!(f, "invalid chunk type"),
            PNGMeError::InvalidChunkLength => write!(f, "invalid chunk length"),
            PNGMeError::InvalidCRC => write!(f, "invalid CRC"),
            PNGMeError::IOError(e) => write!(f, "{}", e),
            PNGMeError::StringFromUtf8Error(e) => write!(f, "{}", e),
            PNGMeError::ArrayFromSliceError(e) => write!(f, "{}", e),
        }
    }
}

impl std::convert::From<std::io::Error> for PNGMeError {
    fn from(e: std::io::Error) -> PNGMeError {
        PNGMeError::IOError(e)
    }
}

impl std::convert::From<std::string::FromUtf8Error> for PNGMeError {
    fn from(e: std::string::FromUtf8Error) -> PNGMeError {
        PNGMeError::StringFromUtf8Error(e)
    }
}

impl std::convert::From<std::array::TryFromSliceError> for PNGMeError {
    fn from(e: std::array::TryFromSliceError) -> PNGMeError {
        PNGMeError::ArrayFromSliceError(e)
    }
}
