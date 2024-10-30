// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

macro_rules! impl_from {
    ($module_path:path => $delegate_ty:ident :: $variant:ident) => {
        impl From<$module_path> for $delegate_ty {
            fn from(val: $module_path) -> Self {
                $delegate_ty::$variant(val.into())
            }
        }
    };
}
