use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    /// Key too short error
    #[error("Key of length {0} is too short")]
    KeyTooShort(usize),

    #[error(transparent)]
    IOError(std::io::Error),
}

impl From<std::io::Error> for Error {
    /// Used to allow the ? operator to convert to our own Error kind
    fn from(err: std::io::Error) -> Self {
        Error::IOError(err)
    }
}