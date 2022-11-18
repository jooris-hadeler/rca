use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    /// Generic Error
    #[error("Generic Error: {0}")]
    Generic(String),

    #[error(transparent)]
    IOError(std::io::Error),
}
