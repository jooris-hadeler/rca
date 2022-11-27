pub use crate::cli::Options;
pub use crate::error::Error;
pub use crate::error::*;
pub use crate::log::*;
pub use colored::Colorize;

pub type Result<T> = core::result::Result<T, Error>;

