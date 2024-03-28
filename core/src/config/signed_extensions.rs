// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module contains implementations for common signed extensions, each
//! of which implements [`SignedExtension`], and can be used in conjunction with
//! [`AnyOf`] to configure the set of signed extensions which are known about
//! when interacting with a chain.

use super::extrinsic_params::ExtrinsicParams;
use super::refine_params::RefineParamsData;
use super::RefineParams;
use crate::client::ClientState;
use crate::config::ExtrinsicParamsEncoder;
use crate::error::ExtrinsicParamsError;
use crate::utils::Era;
use crate::Config;
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::vec::Vec;
use codec::{Compact, Encode};
use core::fmt::Debug;
use derive_where::derive_where;
use hashbrown::HashMap;
use scale_decode::DecodeAsType;
use scale_info::PortableRegistry;

/// A single [`SignedExtension`] has a unique name, but is otherwise the
/// same as [`ExtrinsicParams`] in describing how to encode the extra and
/// additional data.
pub trait SignedExtension<T: Config>: ExtrinsicParams<T> {
    /// The type representing the `extra` bytes of a signed extension.
    /// Decoding from this type should be symmetrical to the respective
    /// `ExtrinsicParamsEncoder::encode_extra_to()` implementation of this signed extension.
    type Decoded: DecodeAsType;

    /// This should return true if the signed extension matches the details given.
    /// Often, this will involve just checking that the identifier given matches that of the
    /// extension in question.
    fn matches(identifier: &str, _type_id: u32, _types: &PortableRegistry) -> bool;
}

/// The [`CheckSpecVersion`] signed extension.
pub struct CheckSpecVersion(u32);

impl<T: Config> ExtrinsicParams<T> for CheckSpecVersion {
    type Params = ();

    fn new(client: &ClientState<T>, _params: Self::Params) -> Result<Self, ExtrinsicParamsError> {
        Ok(CheckSpecVersion(client.runtime_version.spec_version))
    }
}

impl ExtrinsicParamsEncoder for CheckSpecVersion {
    fn encode_additional_to(&self, v: &mut Vec<u8>) {
        self.0.encode_to(v);
    }
}

impl<T: Config> SignedExtension<T> for CheckSpecVersion {
    type Decoded = ();
    fn matches(identifier: &str, _type_id: u32, _types: &PortableRegistry) -> bool {
        identifier == "CheckSpecVersion"
    }
}

/// The [`CheckNonce`] signed extension.
pub struct CheckNonce(Compact<u64>);

impl<T: Config> ExtrinsicParams<T> for CheckNonce {
    type Params = CheckNonceParams;

    fn new(_client: &ClientState<T>, params: Self::Params) -> Result<Self, ExtrinsicParamsError> {
        // If no nonce is set (nor by user nor refinement), use a nonce of 0.
        let nonce = params.0.unwrap_or(0);
        Ok(CheckNonce(Compact(nonce)))
    }
}

impl ExtrinsicParamsEncoder for CheckNonce {
    fn encode_extra_to(&self, v: &mut Vec<u8>) {
        self.0.encode_to(v);
    }
}

impl<T: Config> SignedExtension<T> for CheckNonce {
    type Decoded = u64;
    fn matches(identifier: &str, _type_id: u32, _types: &PortableRegistry) -> bool {
        identifier == "CheckNonce"
    }
}

/// Params for [`CheckNonce`]
#[derive(Debug, Clone, Default)]
pub struct CheckNonceParams(pub Option<u64>);

impl<T: Config> RefineParams<T> for CheckNonceParams {
    fn refine(&mut self, data: &RefineParamsData<T>) {
        if self.0.is_none() {
            self.0 = Some(data.account_nonce());
        }
    }
}

/// The [`CheckTxVersion`] signed extension.
pub struct CheckTxVersion(u32);

impl<T: Config> ExtrinsicParams<T> for CheckTxVersion {
    type Params = ();

    fn new(client: &ClientState<T>, _params: Self::Params) -> Result<Self, ExtrinsicParamsError> {
        Ok(CheckTxVersion(
            client.runtime_version.transaction_version,
        ))
    }
}

impl ExtrinsicParamsEncoder for CheckTxVersion {
    fn encode_additional_to(&self, v: &mut Vec<u8>) {
        self.0.encode_to(v);
    }
}

impl<T: Config> SignedExtension<T> for CheckTxVersion {
    type Decoded = ();
    fn matches(identifier: &str, _type_id: u32, _types: &PortableRegistry) -> bool {
        identifier == "CheckTxVersion"
    }
}

/// The [`CheckGenesis`] signed extension.
pub struct CheckGenesis<T: Config>(T::Hash);

impl<T: Config> ExtrinsicParams<T> for CheckGenesis<T> {
    type Params = ();

    fn new(client: &ClientState<T>, _params: Self::Params) -> Result<Self, ExtrinsicParamsError> {
        Ok(CheckGenesis(client.genesis_hash))
    }
}

impl<T: Config> ExtrinsicParamsEncoder for CheckGenesis<T> {
    fn encode_additional_to(&self, v: &mut Vec<u8>) {
        self.0.encode_to(v);
    }
}

impl<T: Config> SignedExtension<T> for CheckGenesis<T> {
    type Decoded = ();
    fn matches(identifier: &str, _type_id: u32, _types: &PortableRegistry) -> bool {
        identifier == "CheckGenesis"
    }
}

/// The [`CheckMortality`] signed extension.
pub struct CheckMortality<T: Config> {
    era: Era,
    checkpoint: T::Hash,
}

/// Parameters to configure the [`CheckMortality`] signed extension.
pub struct CheckMortalityParams<T: Config>(Option<CheckMortalityParamsInner<T>>);
struct CheckMortalityParamsInner<T: Config> {
    era: Era,
    checkpoint: Option<T::Hash>,
}

impl<T: Config> Default for CheckMortalityParams<T> {
    fn default() -> Self {
        CheckMortalityParams(None)
    }
}

impl<T: Config> RefineParams<T> for CheckMortalityParams<T> {
    fn refine(&mut self, data: &RefineParamsData<T>) {
        if self.0.is_none() {
            // By default we refine the params to have a mortal transaction valid for 32 blocks.
            const TX_VALID_FOR: u64 = 32;
            *self =
                CheckMortalityParams::mortal(TX_VALID_FOR, data.block_number(), data.block_hash());
        }
    }
}

impl<T: Config> CheckMortalityParams<T> {
    /// Configure a mortal transaction. The `period` is (roughly) how many
    /// blocks the transaction will be valid for. The `block_number` and
    /// `block_hash` should both point to the same block, and are the block that
    /// the transaction is mortal from.
    pub fn mortal(period: u64, block_number: u64, block_hash: T::Hash) -> Self {
        Self(Some(CheckMortalityParamsInner {
            era: Era::mortal(period, block_number),
            checkpoint: Some(block_hash),
        }))
    }
    /// An immortal transaction.
    pub fn immortal() -> Self {
        Self(Some(CheckMortalityParamsInner {
            era: Era::Immortal,
            checkpoint: None,
        }))
    }
}

impl<T: Config> ExtrinsicParams<T> for CheckMortality<T> {
    type Params = CheckMortalityParams<T>;

    fn new(client: &ClientState<T>, params: Self::Params) -> Result<Self, ExtrinsicParamsError> {
        let check_mortality = if let Some(params) = params.0 {
            CheckMortality {
                era: params.era,
                checkpoint: params.checkpoint.unwrap_or(client.genesis_hash),
            }
        } else {
            CheckMortality {
                era: Era::Immortal,
                checkpoint: client.genesis_hash,
            }
        };
        Ok(check_mortality)
    }
}

impl<T: Config> ExtrinsicParamsEncoder for CheckMortality<T> {
    fn encode_extra_to(&self, v: &mut Vec<u8>) {
        self.era.encode_to(v);
    }
    fn encode_additional_to(&self, v: &mut Vec<u8>) {
        self.checkpoint.encode_to(v);
    }
}

impl<T: Config> SignedExtension<T> for CheckMortality<T> {
    type Decoded = Era;
    fn matches(identifier: &str, _type_id: u32, _types: &PortableRegistry) -> bool {
        identifier == "CheckMortality"
    }
}

/// The [`ChargeAssetTxPayment`] signed extension.
#[derive(DecodeAsType)]
#[derive_where(Clone, Debug; T::AssetId)]
#[decode_as_type(trait_bounds = "T::AssetId: DecodeAsType")]
pub struct ChargeAssetTxPayment<T: Config> {
    tip: Compact<u128>,
    asset_id: Option<T::AssetId>,
}

impl<T: Config> ChargeAssetTxPayment<T> {
    /// Tip to the extrinsic author in the native chain token.
    pub fn tip(&self) -> u128 {
        self.tip.0
    }

    /// Tip to the extrinsic author using the asset ID given.
    pub fn asset_id(&self) -> Option<&T::AssetId> {
        self.asset_id.as_ref()
    }
}

/// Parameters to configure the [`ChargeAssetTxPayment`] signed extension.
pub struct ChargeAssetTxPaymentParams<T: Config> {
    tip: u128,
    asset_id: Option<T::AssetId>,
}

impl<T: Config> Default for ChargeAssetTxPaymentParams<T> {
    fn default() -> Self {
        ChargeAssetTxPaymentParams {
            tip: Default::default(),
            asset_id: Default::default(),
        }
    }
}

impl<T: Config> ChargeAssetTxPaymentParams<T> {
    /// Don't provide a tip to the extrinsic author.
    pub fn no_tip() -> Self {
        ChargeAssetTxPaymentParams {
            tip: 0,
            asset_id: None,
        }
    }
    /// Tip the extrinsic author in the native chain token.
    pub fn tip(tip: u128) -> Self {
        ChargeAssetTxPaymentParams {
            tip,
            asset_id: None,
        }
    }
    /// Tip the extrinsic author using the asset ID given.
    pub fn tip_of(tip: u128, asset_id: T::AssetId) -> Self {
        ChargeAssetTxPaymentParams {
            tip,
            asset_id: Some(asset_id),
        }
    }
}

impl<T: Config> ExtrinsicParams<T> for ChargeAssetTxPayment<T> {
    type Params = ChargeAssetTxPaymentParams<T>;

    fn new(_client: &ClientState<T>, params: Self::Params) -> Result<Self, ExtrinsicParamsError> {
        Ok(ChargeAssetTxPayment {
            tip: Compact(params.tip),
            asset_id: params.asset_id,
        })
    }
}

impl<T: Config> RefineParams<T> for ChargeAssetTxPaymentParams<T> {}

impl<T: Config> ExtrinsicParamsEncoder for ChargeAssetTxPayment<T> {
    fn encode_extra_to(&self, v: &mut Vec<u8>) {
        (self.tip, &self.asset_id).encode_to(v);
    }
}

impl<T: Config> SignedExtension<T> for ChargeAssetTxPayment<T> {
    type Decoded = Self;
    fn matches(identifier: &str, _type_id: u32, _types: &PortableRegistry) -> bool {
        identifier == "ChargeAssetTxPayment"
    }
}

/// The [`ChargeTransactionPayment`] signed extension.
#[derive(Clone, Debug, DecodeAsType)]
pub struct ChargeTransactionPayment {
    tip: Compact<u128>,
}

impl ChargeTransactionPayment {
    /// Tip to the extrinsic author in the native chain token.
    pub fn tip(&self) -> u128 {
        self.tip.0
    }
}

/// Parameters to configure the [`ChargeTransactionPayment`] signed extension.
#[derive(Default)]
pub struct ChargeTransactionPaymentParams {
    tip: u128,
}

impl ChargeTransactionPaymentParams {
    /// Don't provide a tip to the extrinsic author.
    pub fn no_tip() -> Self {
        ChargeTransactionPaymentParams { tip: 0 }
    }
    /// Tip the extrinsic author in the native chain token.
    pub fn tip(tip: u128) -> Self {
        ChargeTransactionPaymentParams { tip }
    }
}

impl<T: Config> ExtrinsicParams<T> for ChargeTransactionPayment {
    type Params = ChargeTransactionPaymentParams;

    fn new(_client: &ClientState<T>, params: Self::Params) -> Result<Self, ExtrinsicParamsError> {
        Ok(ChargeTransactionPayment {
            tip: Compact(params.tip),
        })
    }
}

impl<T: Config> RefineParams<T> for ChargeTransactionPaymentParams {}

impl ExtrinsicParamsEncoder for ChargeTransactionPayment {
    fn encode_extra_to(&self, v: &mut Vec<u8>) {
        self.tip.encode_to(v);
    }
}

impl<T: Config> SignedExtension<T> for ChargeTransactionPayment {
    type Decoded = Self;
    fn matches(identifier: &str, _type_id: u32, _types: &PortableRegistry) -> bool {
        identifier == "ChargeTransactionPayment"
    }
}

/// This accepts a tuple of [`SignedExtension`]s, and will dynamically make use of whichever
/// ones are actually required for the chain in the correct order, ignoring the rest. This
/// is a sensible default, and allows for a single configuration to work across multiple chains.
pub struct AnyOf<T, Params> {
    params: Vec<Box<dyn ExtrinsicParamsEncoder>>,
    _marker: core::marker::PhantomData<(T, Params)>,
}

macro_rules! impl_tuples {
    ($($ident:ident $index:tt),+) => {
        // We do some magic when the tuple is wrapped in AnyOf. We
        // look at the metadata, and use this to select and make use of only the extensions
        // that we actually need for the chain we're dealing with.
        impl <T, $($ident),+> ExtrinsicParams<T> for AnyOf<T, ($($ident,)+)>
        where
            T: Config,
            $($ident: SignedExtension<T>,)+
        {
            type Params = ($($ident::Params,)+);

            fn new(
                client: &ClientState<T>,
                params: Self::Params,
            ) -> Result<Self, ExtrinsicParamsError> {
                let metadata = &client.metadata;
                let types = metadata.types();

                // For each signed extension in the tuple, find the matching index in the metadata, if
                // there is one, and add it to a map with that index as the key.
                let mut exts_by_index = HashMap::new();
                $({
                    for (idx, e) in metadata.extrinsic().signed_extensions().iter().enumerate() {
                        // Skip over any exts that have a match already:
                        if exts_by_index.contains_key(&idx) {
                            continue
                        }
                        // Break and record as soon as we find a match:
                        if $ident::matches(e.identifier(), e.extra_ty(), types) {
                            let ext = $ident::new(client, params.$index)?;
                            let boxed_ext: Box<dyn ExtrinsicParamsEncoder> = Box::new(ext);
                            exts_by_index.insert(idx, boxed_ext);
                            break
                        }
                    }
                })+

                // Next, turn these into an ordered vec, erroring if we haven't matched on any exts yet.
                let mut params = Vec::new();
                for (idx, e) in metadata.extrinsic().signed_extensions().iter().enumerate() {
                    let Some(ext) = exts_by_index.remove(&idx) else {
                        if is_type_empty(e.extra_ty(), types) {
                            continue
                        } else {
                            return Err(ExtrinsicParamsError::UnknownSignedExtension(e.identifier().to_owned()));
                        }
                    };
                    params.push(ext);
                }

                Ok(AnyOf {
                    params,
                    _marker: core::marker::PhantomData
                })
            }
        }

        impl <T, $($ident),+> ExtrinsicParamsEncoder for AnyOf<T, ($($ident,)+)>
        where
            T: Config,
            $($ident: SignedExtension<T>,)+
        {
            fn encode_extra_to(&self, v: &mut Vec<u8>) {
                for ext in &self.params {
                    ext.encode_extra_to(v);
                }
            }
            fn encode_additional_to(&self, v: &mut Vec<u8>) {
                for ext in &self.params {
                    ext.encode_additional_to(v);
                }
            }
        }
    }
}

#[rustfmt::skip]
const _: () = {
    impl_tuples!(A 0);
    impl_tuples!(A 0, B 1);
    impl_tuples!(A 0, B 1, C 2);
    impl_tuples!(A 0, B 1, C 2, D 3);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, U 19);
    impl_tuples!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, U 19, V 20);
};

/// Checks to see whether the type being given is empty, ie would require
/// 0 bytes to encode.
fn is_type_empty(type_id: u32, types: &scale_info::PortableRegistry) -> bool {
    let Some(ty) = types.resolve(type_id) else {
        // Can't resolve; type may not be empty. Not expected to hit this.
        return false;
    };

    use scale_info::TypeDef;
    match &ty.type_def {
        TypeDef::Composite(c) => c.fields.iter().all(|f| is_type_empty(f.ty.id, types)),
        TypeDef::Array(a) => a.len == 0 || is_type_empty(a.type_param.id, types),
        TypeDef::Tuple(t) => t.fields.iter().all(|f| is_type_empty(f.id, types)),
        // Explicitly list these in case any additions are made in the future.
        TypeDef::BitSequence(_)
        | TypeDef::Variant(_)
        | TypeDef::Sequence(_)
        | TypeDef::Compact(_)
        | TypeDef::Primitive(_) => false,
    }
}
