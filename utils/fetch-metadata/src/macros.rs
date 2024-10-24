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

macro_rules! cfg_fetch_from_url {
	($($item:item)*) => {
		crate::macros::cfg_feature!("url", $($item)*);
	};
}

#[allow(unused)]
pub(crate) use {cfg_feature, cfg_fetch_from_url};
