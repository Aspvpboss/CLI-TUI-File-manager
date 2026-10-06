
mod error;
mod ops;

pub use error::{FmError, Result};
pub use ops::*;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
