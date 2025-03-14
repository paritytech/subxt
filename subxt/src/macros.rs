// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

macro_rules! cfg_feature {
	($feature:literal, $($item:item)*) => {
		$(
			#[cfg(feature = $feature)]
			#[cfg_attr(docsrs, doc(cfg(feature = $feature)))]
			$item
		)*
	}
}

macro_rules! cfg_unstable_light_client {
	($($item:item)*) => {
		crate::macros::cfg_feature!("unstable-light-client", $($item)*);
	};
}

macro_rules! cfg_reconnecting_rpc_client {
	($($item:item)*) => {
		crate::macros::cfg_feature!("reconnecting-rpc-client", $($item)*);
	};
}

macro_rules! cfg_jsonrpsee {
	($($item:item)*) => {
		crate::macros::cfg_feature!("jsonrpsee", $($item)*);
	};
}

#[allow(unused)]
macro_rules! cfg_jsonrpsee_native {
	($($item:item)*) => {
		$(
			#[cfg(all(feature = "jsonrpsee", feature = "native"))]
			#[cfg_attr(docsrs, doc(cfg(all(feature = "jsonrpsee", feature = "native"))))]
			$item
		)*
	}
}

#[allow(unused)]
macro_rules! cfg_jsonrpsee_web {
	($($item:item)*) => {
		$(
			#[cfg(all(feature = "jsonrpsee", feature = "web"))]
			#[cfg_attr(docsrs, doc(cfg(all(feature = "jsonrpsee", feature = "web"))))]
			$item
		)*
	}
}

pub(crate) use {cfg_feature, cfg_jsonrpsee, cfg_unstable_light_client};

// Only used by light-client.
#[allow(unused)]
pub(crate) use {cfg_jsonrpsee_native, cfg_jsonrpsee_web, cfg_reconnecting_rpc_client};
