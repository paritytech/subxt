// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Exports from `std`, `core` and `alloc` crates.
//!
//! Guarantees a stable interface between `std` and `no_std` modes.

#[cfg(not(feature = "std"))]
extern crate alloc;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "std")] {
        pub use std::{
            any,
            borrow,
            boxed::{Box},
            cmp,
            collections,
            fmt,
            format,
            hash,
            marker::{PhantomData},
            mem,
            num,
            ops,
            string,
            sync,
            time,
            vec::{vec, Vec},
            rc,
            iter,
        };
    } else {
        pub use alloc::{
            borrow,
            boxed::{Box},
            collections,
            format,
            string,
            sync,
            vec::{vec, Vec},
            rc
        };

        pub use core::{
            any,
            cmp,
            fmt,
            hash,
            marker::{PhantomData},
            mem,
            num,
            ops,
            time,
            iter,
        };
    }
}
