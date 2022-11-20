pub use crate::cli::Options;
pub use crate::error::Error;
pub use crate::error::*;

pub type Result<T> = core::result::Result<T, Error>;
