// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

mod v14;
mod v15;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum TryFromError {
    #[error("Expected an sp_runtime::DispatchError type to exist in the type registry, but there was none")]
    DispatchErrorTypeNotFound,
    /// Type missing from type registry
    #[error("Type {0} is expected but not found in the type registry")]
    TypeNotFound(u32),
    /// Type was not a variant/enum type
    #[error("Type {0} was not a variant/enum type, but is expected to be one")]
    VariantExpected(u32),
}
