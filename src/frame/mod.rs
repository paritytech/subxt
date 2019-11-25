//! Implements support for built-in runtime modules.

use parity_scale_codec::Encode;

pub mod balances;
pub mod contracts;
pub mod system;

/// Creates module calls
pub struct Call<T: Encode> {
    pub module: &'static str,
    pub function: &'static str,
    pub args: T,
}

impl<T: Encode> Call<T> {
    pub fn new(module: &'static str, function: &'static str, args: T) -> Self {
        Call { module, function, args }
    }
}
