// Copyright 2019-2024 Parity Technologies (UK) Ltd.
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

pub(crate) use {cfg_feature, cfg_substrate_compat};
