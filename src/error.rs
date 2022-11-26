use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    /// Generic Error
    #[error("Generic Error: {0}")]
    Generic(String),

    #[error(transparent)]
    IOError(std::io::Error),
}

impl From<std::io::Error> for Error {
    /// Used to allow the ? operator to convert to our own Error kind
    fn from(err: std::io::Error) -> Self {
        Error::IOError(err)
    }
}