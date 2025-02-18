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

macro_rules! cfg_jsonrpsee {
	($($item:item)*) => {
		crate::macros::cfg_feature!("jsonrpsee", $($item)*);
	};
}

macro_rules! cfg_reconnecting_rpc_client {
	($($item:item)*) => {
		$(
			#[cfg(all(feature = "reconnecting-rpc-client", any(feature = "native", feature = "web")))]
			#[cfg_attr(docsrs, doc(cfg(feature = "reconnecting-rpc-client")))]
			$item
		)*
	}
}

macro_rules! cfg_mock_rpc_client {
	($($item:item)*) => {
		crate::macros::cfg_feature!("mock-rpc-client", $($item)*);
	};
}

pub(crate) use {
    cfg_feature, cfg_jsonrpsee, cfg_mock_rpc_client, cfg_reconnecting_rpc_client,
    cfg_unstable_light_client,
};
