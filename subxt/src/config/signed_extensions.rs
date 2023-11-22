// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module contains implementations for common signed extensions, each
//! of which implements [`SignedExtension`], and can be used in conjunction with
//! [`AnyOf`] to configure the set of signed extensions which are known about
//! when interacting with a chain.

use super::extrinsic_params::{ExtrinsicParams, ExtrinsicParamsEncoder, ExtrinsicParamsError};
use crate::utils::Era;
use crate::{client::OfflineClientT, Config};
use codec::{Compact, Encode};
use core::fmt::Debug;
use scale_decode::DecodeAsType;
use scale_encode::EncodeAsType;
use scale_info::PortableRegistry;

use std::collections::HashMap;

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
    ///
    /// The first match that returns true will be the entry that this signed extension
    /// is used to encode values for. This takes `&mut self`, allowing the extension to
    /// cache values if it likes when it finds the type it'll be encoding for.
    fn matches(
        identifier: &str,
        type_id: u32,
        types: &PortableRegistry,
    ) -> Result<bool, ExtrinsicParamsError>;
}

/// The [`CheckSpecVersion`] signed extension.
#[derive(Clone, Debug, EncodeAsType, DecodeAsType)]
pub struct CheckSpecVersion(u32);

impl<T: Config> ExtrinsicParams<T> for CheckSpecVersion {
    type OtherParams = ();

    fn new<Client: OfflineClientT<T>>(
        _nonce: u64,
        client: Client,
        _other_params: Self::OtherParams,
    ) -> Result<Self, ExtrinsicParamsError> {
        Ok(CheckSpecVersion(client.runtime_version().spec_version))
    }
}

impl ExtrinsicParamsEncoder for CheckSpecVersion {
    fn encode_additional_to(&self, v: &mut Vec<u8>) {
        self.0.encode_to(v);
    }
}

impl<T: Config> SignedExtension<T> for CheckSpecVersion {
    type Decoded = ();
    fn matches(
        _identifier: &str,
        type_id: u32,
        types: &PortableRegistry,
    ) -> Result<bool, ExtrinsicParamsError> {
        type_id_matches_ext_name(type_id, types, "CheckSpecVersion")
    }
}

/// The [`CheckNonce`] signed extension.
#[derive(Clone, Debug, EncodeAsType, DecodeAsType)]
pub struct CheckNonce(Compact<u64>);

impl<T: Config> ExtrinsicParams<T> for CheckNonce {
    type OtherParams = ();

    fn new<Client: OfflineClientT<T>>(
        nonce: u64,
        _client: Client,
        _other_params: Self::OtherParams,
    ) -> Result<Self, ExtrinsicParamsError> {
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
    fn matches(
        _identifier: &str,
        type_id: u32,
        types: &PortableRegistry,
    ) -> Result<bool, ExtrinsicParamsError> {
        type_id_matches_ext_name(type_id, types, "CheckNonce")
    }
}

/// The [`CheckTxVersion`] signed extension.
#[derive(Clone, Debug, EncodeAsType, DecodeAsType)]
pub struct CheckTxVersion(u32);

impl<T: Config> ExtrinsicParams<T> for CheckTxVersion {
    type OtherParams = ();

    fn new<Client: OfflineClientT<T>>(
        _nonce: u64,
        client: Client,
        _other_params: Self::OtherParams,
    ) -> Result<Self, ExtrinsicParamsError> {
        Ok(CheckTxVersion(client.runtime_version().transaction_version))
    }
}

impl ExtrinsicParamsEncoder for CheckTxVersion {
    fn encode_additional_to(&self, v: &mut Vec<u8>) {
        self.0.encode_to(v);
    }
}

impl<T: Config> SignedExtension<T> for CheckTxVersion {
    type Decoded = ();
    fn matches(
        _identifier: &str,
        type_id: u32,
        types: &PortableRegistry,
    ) -> Result<bool, ExtrinsicParamsError> {
        type_id_matches_ext_name(type_id, types, "CheckTxVersion")
    }
}

/// The [`CheckGenesis`] signed extension.
#[derive(Clone, EncodeAsType, DecodeAsType)]
#[decode_as_type(trait_bounds = "T::Hash: DecodeAsType")]
#[encode_as_type(trait_bounds = "T::Hash: EncodeAsType")]
pub struct CheckGenesis<T: Config>(T::Hash);

impl<T: Config> std::fmt::Debug for CheckGenesis<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CheckGenesis").field(&self.0).finish()
    }
}

impl<T: Config> ExtrinsicParams<T> for CheckGenesis<T> {
    type OtherParams = ();

    fn new<Client: OfflineClientT<T>>(
        _nonce: u64,
        client: Client,
        _other_params: Self::OtherParams,
    ) -> Result<Self, ExtrinsicParamsError> {
        Ok(CheckGenesis(client.genesis_hash()))
    }
}

impl<T: Config> ExtrinsicParamsEncoder for CheckGenesis<T> {
    fn encode_additional_to(&self, v: &mut Vec<u8>) {
        self.0.encode_to(v);
    }
}

impl<T: Config> SignedExtension<T> for CheckGenesis<T> {
    type Decoded = ();
    fn matches(
        _identifier: &str,
        type_id: u32,
        types: &PortableRegistry,
    ) -> Result<bool, ExtrinsicParamsError> {
        type_id_matches_ext_name(type_id, types, "CheckGenesis")
    }
}

/// The [`CheckMortality`] signed extension.
#[derive(Clone, EncodeAsType, DecodeAsType)]
#[decode_as_type(trait_bounds = "T::Hash: DecodeAsType")]
#[encode_as_type(trait_bounds = "T::Hash: EncodeAsType")]
pub struct CheckMortality<T: Config> {
    era: Era,
    checkpoint: T::Hash,
}

/// Parameters to configure the [`CheckMortality`] signed extension.
#[derive(Clone, Debug)]
pub struct CheckMortalityParams<T: Config> {
    era: Era,
    checkpoint: Option<T::Hash>,
}

impl<T: Config> Default for CheckMortalityParams<T> {
    fn default() -> Self {
        Self {
            era: Default::default(),
            checkpoint: Default::default(),
        }
    }
}

impl<T: Config> CheckMortalityParams<T> {
    /// Configure a mortal transaction. The `period` is (roughly) how many
    /// blocks the transaction will be valid for. The `block_number` and
    /// `block_hash` should both point to the same block, and are the block that
    /// the transaction is mortal from.
    pub fn mortal(period: u64, block_number: u64, block_hash: T::Hash) -> Self {
        CheckMortalityParams {
            era: Era::mortal(period, block_number),
            checkpoint: Some(block_hash),
        }
    }
    /// An immortal transaction.
    pub fn immortal() -> Self {
        CheckMortalityParams {
            era: Era::Immortal,
            checkpoint: None,
        }
    }
}

impl<T: Config> std::fmt::Debug for CheckMortality<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CheckMortality")
            .field("era", &self.era)
            .field("checkpoint", &self.checkpoint)
            .finish()
    }
}

impl<T: Config> ExtrinsicParams<T> for CheckMortality<T> {
    type OtherParams = CheckMortalityParams<T>;

    fn new<Client: OfflineClientT<T>>(
        _nonce: u64,
        client: Client,
        other_params: Self::OtherParams,
    ) -> Result<Self, ExtrinsicParamsError> {
        Ok(CheckMortality {
            era: other_params.era,
            checkpoint: other_params.checkpoint.unwrap_or(client.genesis_hash()),
        })
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
    fn matches(
        _identifier: &str,
        type_id: u32,
        types: &PortableRegistry,
    ) -> Result<bool, ExtrinsicParamsError> {
        type_id_matches_ext_name(type_id, types, "CheckMortality")
    }
}

/// The [`ChargeAssetTxPayment`] signed extension.
#[derive(Clone, Debug, DecodeAsType)]
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
#[derive(Debug)]
pub struct ChargeAssetTxPaymentParams<T: Config> {
    tip: u128,
    asset_id: Option<T::AssetId>,
}

// Dev note: `#[derive(Clone)]` implies `T: Clone` instead of `T::AssetId: Clone`.
impl<T: Config> Clone for ChargeAssetTxPaymentParams<T> {
    fn clone(&self) -> Self {
        Self {
            tip: self.tip,
            asset_id: self.asset_id.clone(),
        }
    }
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
    type OtherParams = ChargeAssetTxPaymentParams<T>;

    fn new<Client: OfflineClientT<T>>(
        _nonce: u64,
        _client: Client,
        other_params: Self::OtherParams,
    ) -> Result<Self, ExtrinsicParamsError> {
        Ok(ChargeAssetTxPayment {
            tip: Compact(other_params.tip),
            asset_id: other_params.asset_id,
        })
    }
}

impl<T: Config> ExtrinsicParamsEncoder for ChargeAssetTxPayment<T> {
    fn encode_extra_to(&self, v: &mut Vec<u8>) {
        (self.tip, &self.asset_id).encode_to(v);
    }
}

impl<T: Config> SignedExtension<T> for ChargeAssetTxPayment<T> {
    type Decoded = Self;
    fn matches(
        _identifier: &str,
        type_id: u32,
        types: &PortableRegistry,
    ) -> Result<bool, ExtrinsicParamsError> {
        type_id_matches_ext_name(type_id, types, "ChargeAssetTxPayment")
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
    type OtherParams = ChargeTransactionPaymentParams;

    fn new<Client: OfflineClientT<T>>(
        _nonce: u64,
        _client: Client,
        other_params: Self::OtherParams,
    ) -> Result<Self, ExtrinsicParamsError> {
        Ok(ChargeTransactionPayment {
            tip: Compact(other_params.tip),
        })
    }
}

impl ExtrinsicParamsEncoder for ChargeTransactionPayment {
    fn encode_extra_to(&self, v: &mut Vec<u8>) {
        self.tip.encode_to(v);
    }
}

impl<T: Config> SignedExtension<T> for ChargeTransactionPayment {
    type Decoded = Self;
    fn matches(
        _identifier: &str,
        type_id: u32,
        types: &PortableRegistry,
    ) -> Result<bool, ExtrinsicParamsError> {
        type_id_matches_ext_name(type_id, types, "ChargeTransactionPayment")
    }
}

/// This accepts a tuple of [`SignedExtension`]s, and will dynamically make use of whichever
/// ones are actually required for the chain in the correct order, ignoring the rest. This
/// is a sensible default, and allows for a single configuration to work across multiple chains.
pub struct AnyOf<T, Params> {
    params: Vec<Box<dyn ExtrinsicParamsEncoder>>,
    _marker: std::marker::PhantomData<(T, Params)>,
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
            type OtherParams = ($($ident::OtherParams,)+);

            fn new<Client: OfflineClientT<T>>(
                nonce: u64,
                client: Client,
                other_params: Self::OtherParams,
            ) -> Result<Self, ExtrinsicParamsError> {
                let metadata = client.metadata();
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
                        if $ident::matches(e.identifier(), e.extra_ty(), types)? {
                            let ext = $ident::new(nonce, client.clone(), other_params.$index)?;
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
                    _marker: std::marker::PhantomData
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

/// Resolve a type ID and check that its path equals the extension name provided.
/// As an exception; if the name of the extension is the transparent "SkipCheckIfFeeless"
/// signed extension, then we look at the inner type of that to determine whether it's a match.
fn type_id_matches_ext_name(
    type_id: u32,
    types: &PortableRegistry,
    ext_name: &'static str,
) -> Result<bool, ExtrinsicParamsError> {
    let Some(ty) = types.resolve(type_id) else {
        return Err(ExtrinsicParamsError::MissingTypeId {
            type_id,
            context: ext_name,
        });
    };
    let Some(name) = ty.path.segments.last() else {
        return Ok(false);
    };
    if name == "SkipCheckIfFeeless" {
        // SkipCheckIfFeeless is a transparent wrapper that can be applied around any signed extension.
        // It should have 2 generic types: the inner signed extension and a phantom data type. Phantom data does
        // not have a type associated, so we find the type that does to find the inner signed extension.
        // If this doesn't pan out, don't error, just don't match.
        let Some(inner_type_id) = ty
            .type_params
            .iter()
            .find_map(|param| param.ty.map(|ty| ty.id))
        else {
            return Ok(false);
        };

        type_id_matches_ext_name(inner_type_id, types, ext_name)
    } else {
        Ok(name == ext_name)
    }
}
