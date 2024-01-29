// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

#[cfg(not(feature = "std"))]
extern crate alloc;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "std")] {
        #[allow(unused)]
        pub use std::{
            any,
            borrow,
            boxed,
            cmp,
            collections,
            fmt,
            format,
            hash,
            marker,
            mem,
            num,
            ops,
            string,
            sync,
            time,
            vec,
            rc,
            iter,
        };
    } else {
        #[allow(unused)]
        pub use alloc::{
            borrow,
            boxed,
            collections,
            format,
            string,
            sync,
            vec,
            rc
        };
        #[allow(unused)]
        pub use core::{
            any,
            cmp,
            fmt,
            hash,
            marker,
            mem,
            num,
            ops,
            time,
            iter,
        };
    }
}
