//! Implements support for built-in runtime modules.

use crate::metadata::ModuleMetadata;
use std::marker::PhantomData;

pub mod balances;
pub mod contracts;
pub mod system;

/// Creates module calls
pub struct ModuleCalls<T, P> {
    module: ModuleMetadata,
    marker: PhantomData<fn() -> (T, P)>,
}

impl<T, P> ModuleCalls<T, P> {
    /// Create new module calls
    pub fn new(module: &ModuleMetadata) -> Self {
        ModuleCalls::<T, P> {
            module: module.clone(),
            marker: PhantomData,
        }
    }
}
