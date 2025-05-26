#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    SerdeJsonError(serde_json::Error),

    #[error("{0}")]
    IOError(std::io::Error),

    #[error("{0}")]
    InvalidInput(String),

    #[error("Invalid float input: {0}")]
    InvalidFloat(String),

    #[error("Invalid unsigned input: {0}")]
    InvalidUnsigned(String),

    #[error("Parse error {0}")]
    ParseError(#[from] std::num::ParseIntError),

    #[error("Float parse error: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),

}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJsonError(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}




pub type ProjectResult<T> = Result<T, Error>;