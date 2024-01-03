// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! Default platform for WASM environments.

#[cfg(feature = "web")]
mod wasm_helpers;
#[cfg(feature = "web")]
mod wasm_platform;
#[cfg(feature = "web")]
mod wasm_socket;

pub use helpers::build_platform;

#[cfg(feature = "native")]
mod helpers {
    use smoldot_light::platform::default::DefaultPlatform as Platform;
    use std::sync::Arc;

    pub type PlatformType = Arc<Platform>;

    pub fn build_platform() -> PlatformType {
        Platform::new(
            "subxt-light-client".into(),
            env!("CARGO_PKG_VERSION").into(),
        )
    }
}

#[cfg(feature = "web")]
mod helpers {
    use super::wasm_platform::SubxtPlatform as Platform;

    pub type PlatformType = Platform;

    pub fn build_platform() -> PlatformType {
        Platform::new()
    }
}
