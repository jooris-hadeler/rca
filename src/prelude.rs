pub use crate::error::*;
pub use crate::cli::Options;
pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;
