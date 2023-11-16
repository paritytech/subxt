// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module contains implementations for common signed extensions, each
//! of which implements [`SignedExtension`], and can be used in conjunction with
//! [`AnyOf`] to configure the set of signed extensions which are known about
//! when interacting with a chain.

use super::extrinsic_params::{ExtrinsicParams, ExtrinsicParamsEncoder, ExtrinsicParamsError};
use crate::utils::Era;
use crate::{client::OfflineClientT, Config, Metadata};
use codec::{Compact, Encode};
use core::fmt::Debug;
use scale_decode::DecodeAsType;
use scale_encode::EncodeAsType;
use std::marker::PhantomData;

use std::collections::HashMap;

/// A single [`SignedExtension`] has a unique name, but is otherwise the
/// same as [`ExtrinsicParams`] in describing how to encode the extra and
/// additional data.
pub trait SignedExtension<T: Config>: ExtrinsicParams<T> {
    /// The name of the signed extension. This is used to associate it
    /// with the signed extensions that the node is making use of.
    const NAME: &'static str;

    /// The type representing the `extra` bytes of a signed extension.
    /// Decoding from this type should be symmetrical to the respective
    /// `ExtrinsicParamsEncoder::encode_extra_to()` implementation of this signed extension.
    type Decoded: DecodeAsType;
}

/// The [`CheckSpecVersion`] signed extension.
#[derive(Debug)]
pub struct CheckSpecVersion(u32);

impl<T: Config> ExtrinsicParams<T> for CheckSpecVersion {
    type OtherParams = ();
    type Error = std::convert::Infallible;

    fn new<Client: OfflineClientT<T>>(
        _nonce: u64,
        client: Client,
        _other_params: Self::OtherParams,
    ) -> Result<Self, Self::Error> {
        Ok(CheckSpecVersion(client.runtime_version().spec_version))
    }
}

impl ExtrinsicParamsEncoder for CheckSpecVersion {
    fn encode_additional_to(&self, v: &mut Vec<u8>) {
        self.0.encode_to(v);
    }
}

impl<T: Config> SignedExtension<T> for CheckSpecVersion {
    const NAME: &'static str = "CheckSpecVersion";
    type Decoded = ();
}

/// The [`CheckNonce`] signed extension.
#[derive(Debug)]
pub struct CheckNonce(Compact<u64>);

impl<T: Config> ExtrinsicParams<T> for CheckNonce {
    type OtherParams = ();
    type Error = std::convert::Infallible;

    fn new<Client: OfflineClientT<T>>(
        nonce: u64,
        _client: Client,
        _other_params: Self::OtherParams,
    ) -> Result<Self, Self::Error> {
        Ok(CheckNonce(Compact(nonce)))
    }
}

impl ExtrinsicParamsEncoder for CheckNonce {
    fn encode_extra_to(&self, v: &mut Vec<u8>) {
        self.0.encode_to(v);
    }
}

impl<T: Config> SignedExtension<T> for CheckNonce {
    const NAME: &'static str = "CheckNonce";
    type Decoded = Compact<u64>;
}

/// The [`CheckTxVersion`] signed extension.
#[derive(Debug)]
pub struct CheckTxVersion(u32);

impl<T: Config> ExtrinsicParams<T> for CheckTxVersion {
    type OtherParams = ();
    type Error = std::convert::Infallible;

    fn new<Client: OfflineClientT<T>>(
        _nonce: u64,
        client: Client,
        _other_params: Self::OtherParams,
    ) -> Result<Self, Self::Error> {
        Ok(CheckTxVersion(client.runtime_version().transaction_version))
    }
}

impl ExtrinsicParamsEncoder for CheckTxVersion {
    fn encode_additional_to(&self, v: &mut Vec<u8>) {
        self.0.encode_to(v);
    }
}

impl<T: Config> SignedExtension<T> for CheckTxVersion {
    const NAME: &'static str = "CheckTxVersion";
    type Decoded = ();
}

/// The [`CheckGenesis`] signed extension.
pub struct CheckGenesis<T: Config>(T::Hash);

impl<T: Config> std::fmt::Debug for CheckGenesis<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CheckGenesis").field(&self.0).finish()
    }
}

impl<T: Config> ExtrinsicParams<T> for CheckGenesis<T> {
    type OtherParams = ();
    type Error = std::convert::Infallible;

    fn new<Client: OfflineClientT<T>>(
        _nonce: u64,
        client: Client,
        _other_params: Self::OtherParams,
    ) -> Result<Self, Self::Error> {
        Ok(CheckGenesis(client.genesis_hash()))
    }
}

impl<T: Config> ExtrinsicParamsEncoder for CheckGenesis<T> {
    fn encode_additional_to(&self, v: &mut Vec<u8>) {
        self.0.encode_to(v);
    }
}

impl<T: Config> SignedExtension<T> for CheckGenesis<T> {
    const NAME: &'static str = "CheckGenesis";
    type Decoded = ();
}

/// The [`CheckMortality`] signed extension.
pub struct CheckMortality<T: Config> {
    era: Era,
    checkpoint: T::Hash,
}

/// Parameters to configure the [`CheckMortality`] signed extension.
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
    type Error = std::convert::Infallible;

    fn new<Client: OfflineClientT<T>>(
        _nonce: u64,
        client: Client,
        other_params: Self::OtherParams,
    ) -> Result<Self, Self::Error> {
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
        self.checkpoint.encode_to(v)
    }
}

impl<T: Config> SignedExtension<T> for CheckMortality<T> {
    const NAME: &'static str = "CheckMortality";
    type Decoded = Era;
}

/// The [`ChargeAssetTxPayment`] signed extension.
#[derive(Debug, DecodeAsType, EncodeAsType, Clone)]
#[decode_as_type(trait_bounds = "T::AssetId: DecodeAsType")]
#[encode_as_type(trait_bounds = "T::AssetId: EncodeAsType")]
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

// Dev note: `#[derive(Clone)]` implies `T: Clone` instead of `T::AssetId: Clone`.
impl<T: Config> Clone for ChargeAssetTxPaymentParams<T> {
    fn clone(&self) -> Self {
        Self {
            tip: self.tip.clone(),
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
    type Error = std::convert::Infallible;

    fn new<Client: OfflineClientT<T>>(
        _nonce: u64,
        _client: Client,
        other_params: Self::OtherParams,
    ) -> Result<Self, Self::Error> {
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
    const NAME: &'static str = "ChargeAssetTxPayment";
    type Decoded = Self;
}

/// The [`ChargeTransactionPayment`] signed extension.
#[derive(Debug, DecodeAsType, EncodeAsType)]
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
    type Error = std::convert::Infallible;

    fn new<Client: OfflineClientT<T>>(
        _nonce: u64,
        _client: Client,
        other_params: Self::OtherParams,
    ) -> Result<Self, Self::Error> {
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
    const NAME: &'static str = "ChargeTransactionPayment";
    type Decoded = Self;
}

/// Information needed to encode the [`SkipCheckIfFeeless`] signed extension.
#[derive(Debug)]
struct SkipCheckIfFeelessEncodingData {
    metadata: Metadata,
    type_id: u32,
}

impl SkipCheckIfFeelessEncodingData {
    /// Construct [`SkipCheckIfFeelessEncodingData`].
    fn new(
        metadata: Metadata,
        extension: &str,
        inner_extension: &str,
    ) -> Result<Self, ExtrinsicParamsError> {
        let skip_check_type_id = metadata
            .extrinsic()
            .signed_extensions()
            .iter()
            .find_map(|ext| {
                if ext.identifier() == extension {
                    Some(ext.extra_ty())
                } else {
                    None
                }
            });
        let Some(skip_check_type_id) = skip_check_type_id else {
            return Err(ExtrinsicParamsError::UnknownSignedExtension(
                inner_extension.to_owned(),
            ));
        };

        // Ensure that the `SkipCheckIfFeeless` type has the same inner signed extension as provided.
        let Some(skip_check_ty) = metadata.types().resolve(skip_check_type_id) else {
            return Err(ExtrinsicParamsError::MissingTypeId(
                inner_extension.to_owned(),
                skip_check_type_id,
            ));
        };

        // The substrate's `SkipCheckIfFeeless` contains 2 types: the inner signed extension and a phantom data.
        // Phantom data does not have a type associated, so we need to find the inner signed extension.
        let Some(inner_type_id) = skip_check_ty
            .type_params
            .iter()
            .find_map(|param| param.ty.map(|ty| ty.id))
        else {
            return Err(ExtrinsicParamsError::MissingInnerSignedExtension(
                inner_extension.to_owned(),
            ));
        };

        // Get the inner type of the `SkipCheckIfFeeless` extension to check if the naming matches the provided parameters.
        let Some(inner_extension_ty) = metadata.types().resolve(inner_type_id) else {
            return Err(ExtrinsicParamsError::MissingTypeId(
                inner_extension.to_owned(),
                inner_type_id,
            ));
        };

        let Some(inner_extension_name) = inner_extension_ty.path.segments.last() else {
            return Err(ExtrinsicParamsError::ExpectedNamedTypeId(
                inner_extension.to_owned(),
                inner_type_id,
            ));
        };

        if inner_extension_name != inner_extension {
            return Err(ExtrinsicParamsError::ExpectedAnotherExtension(
                inner_extension.to_owned(),
                inner_extension_name.to_owned(),
            ));
        }

        Ok(SkipCheckIfFeelessEncodingData {
            metadata,
            type_id: inner_type_id,
        })
    }
}

/// The [`SkipCheckIfFeeless`] signed extension.
#[derive(Debug, DecodeAsType)]
#[decode_as_type(trait_bounds = "S: DecodeAsType")]
pub struct SkipCheckIfFeeless<T, S>
where
    T: Config,
    S: SignedExtension<T> + DecodeAsType + EncodeAsType,
{
    inner: S,
    // Dev note: This is `Option` because `#[derive(DecodeAsType)]` requires the
    // `Default` bound on skipped parameters.
    // This field is populated when the [`SkipCheckIfFeeless`] is constructed from
    // [`ExtrinsicParams`] (ie, when subxt submits extrinsics). However, it is not
    // populated when decoding signed extensions from the node.
    #[decode_as_type(skip)]
    encoding_data: Option<SkipCheckIfFeelessEncodingData>,
    #[decode_as_type(skip)]
    _phantom: PhantomData<T>,
}

impl<T, S> SkipCheckIfFeeless<T, S>
where
    T: Config,
    S: SignedExtension<T> + DecodeAsType + EncodeAsType,
{
    /// The inner signed extension.
    pub fn inner_signed_extension(&self) -> &S {
        &self.inner
    }
}

impl<T, S> ExtrinsicParams<T> for SkipCheckIfFeeless<T, S>
where
    T: Config,
    S: SignedExtension<T> + DecodeAsType + EncodeAsType,
    <S as ExtrinsicParams<T>>::OtherParams: Default,
{
    type OtherParams = SkipCheckIfFeelessParams<T, S>;
    type Error = ExtrinsicParamsError;

    fn new<Client: OfflineClientT<T>>(
        nonce: u64,
        client: Client,
        other_params: Self::OtherParams,
    ) -> Result<Self, Self::Error> {
        let other_params = match other_params.0 {
            Some(other_params) => other_params,
            None => <S as ExtrinsicParams<T>>::OtherParams::default(),
        };

        let metadata = client.metadata();
        let encoding_data = SkipCheckIfFeelessEncodingData::new(metadata, Self::NAME, S::NAME)?;
        let inner_extension = S::new(nonce, client, other_params).map_err(Into::into)?;

        Ok(SkipCheckIfFeeless {
            inner: inner_extension,
            encoding_data: Some(encoding_data),
            _phantom: PhantomData,
        })
    }
}

impl<T, S> ExtrinsicParamsEncoder for SkipCheckIfFeeless<T, S>
where
    T: Config,
    S: SignedExtension<T> + DecodeAsType + EncodeAsType,
{
    fn encode_extra_to(&self, v: &mut Vec<u8>) {
        if let Some(encoding_data) = &self.encoding_data {
            let _ = self.inner.encode_as_type_to(
                encoding_data.type_id,
                encoding_data.metadata.types(),
                v,
            );
        }
    }
}

impl<T, S> SignedExtension<T> for SkipCheckIfFeeless<T, S>
where
    T: Config,
    S: SignedExtension<T> + DecodeAsType + EncodeAsType,
    <S as ExtrinsicParams<T>>::OtherParams: Default,
{
    const NAME: &'static str = "SkipCheckIfFeeless";
    type Decoded = Self;
}

/// Parameters to configure the [`SkipCheckIfFeeless`] signed extension.
pub struct SkipCheckIfFeelessParams<T, S>(Option<<S as ExtrinsicParams<T>>::OtherParams>)
where
    T: Config,
    S: SignedExtension<T>;

impl<T: Config, S: SignedExtension<T>> Default for SkipCheckIfFeelessParams<T, S>
where
    T: Config,
    S: SignedExtension<T>,
{
    fn default() -> Self {
        SkipCheckIfFeelessParams(None)
    }
}

impl<T, S> SkipCheckIfFeelessParams<T, S>
where
    T: Config,
    S: SignedExtension<T>,
{
    /// Skip the check if the transaction is feeless.
    pub fn from(extrinsic_params: <S as ExtrinsicParams<T>>::OtherParams) -> Self {
        SkipCheckIfFeelessParams(Some(extrinsic_params))
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
            type Error = ExtrinsicParamsError;

            fn new<Client: OfflineClientT<T>>(
                nonce: u64,
                client: Client,
                other_params: Self::OtherParams,
            ) -> Result<Self, Self::Error> {
                // First, push encoders to map as we are given them:
                let mut map = HashMap::new();
                $({
                    let e: Box<dyn ExtrinsicParamsEncoder>
                        = Box::new($ident::new(nonce, client.clone(), other_params.$index).map_err(Into::into)?);
                    map.insert($ident::NAME, e);
                })+

                // Next, based on metadata, push to vec in the order the node needs:
                let mut params = Vec::new();
                let metadata = client.metadata();
                let types = metadata.types();
                for ext in metadata.extrinsic().signed_extensions() {
                    if let Some(ext) = map.remove(ext.identifier()) {
                        params.push(ext)
                    } else {
                        if is_type_empty(ext.extra_ty(), types) && is_type_empty(ext.additional_ty(), types) {
                            // If we don't know about the signed extension, _but_ it appears to require zero bytes
                            // to encode its extra and additional data, then we can safely ignore it as it makes
                            // no difference either way.
                            continue;
                        }
                        return Err(ExtrinsicParamsError::UnknownSignedExtension(ext.identifier().to_owned()));
                    }
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
