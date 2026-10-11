
mod error;
mod fs_ops;
mod selection;
mod transfer;

pub use error::{PfadError, Result};
pub use fs_ops::*;
pub use selection::*;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
