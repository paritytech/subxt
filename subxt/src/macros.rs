// Copyright 2019-2023 Parity Technologies (UK) Ltd.
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

macro_rules! cfg_substrate_compat {
	($($item:item)*) => {
		crate::macros::cfg_feature!("substrate-compat", $($item)*);
	};
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

pub(crate) use cfg_feature;
pub(crate) use cfg_jsonrpsee;
#[allow(unused)]
pub(crate) use cfg_jsonrpsee_native;
pub(crate) use cfg_substrate_compat;
pub(crate) use cfg_unstable_light_client;
