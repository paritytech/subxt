#[cfg(not(feature = "std"))]
extern crate alloc;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "std")] {



        pub use std::{
            borrow,
            borrow::ToOwned,
            boxed::{Box},
            collections,
            format,
            format::format,
            string::{String},
            str,
            sync,
            vec,
            rc,
        };
        pub use std::vec::Vec;
    } else {
        pub use alloc::{
            borrow,
            borrow::ToOwned,
            boxed::{Box},
            collections,
            format,
            string::{String},
            str,
            sync,
            vec,
            rc,
        };
        pub use alloc::vec::Vec;

    }
}
