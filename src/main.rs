mod chunk_type;

pub type Error = PNGMeError;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum PNGMeError {
    InvalidChunkTypeCode,
}

impl std::error::Error for PNGMeError {}

impl std::fmt::Display for PNGMeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PNGMeError::InvalidChunkTypeCode => write!(f, "invalid chunk type code"),
        }
    }
}

fn main() {
    todo!();
}
