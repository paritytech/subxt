// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module contains a trait which controls the parameters that must
//! be provided in order to successfully construct an extrinsic. A basic
//! implementation of the trait is provided ([`BaseSignedExtensions`]) which is
//! used by the provided Substrate and Polkadot configuration.

use crate::{client::OfflineClientT, Config};
use crate::config::substrate::Era;
use codec::{Compact, Encode};
use core::fmt::Debug;
use std::collections::HashMap;

/// An error that can be emitted when trying to construct
/// extrinsic parameters.
#[derive(thiserror::Error, Debug)]
pub enum ExtrinsicParamsError {
    /// A signed extension was encountered that we don't know about.
    #[error("Unknown signed extension: {0}")]
    UnknownSignedExtension(String)
}

impl From<std::convert::Infallible> for ExtrinsicParamsError {
    fn from(value: std::convert::Infallible) -> Self {
        match value {}
    }
}

/// This trait allows you to configure the "signed extra" and
/// "additional" parameters that are signed and used in transactions.
/// Tuples of [`SignedExtension`]'s automatically implement this.
pub trait ExtrinsicParams<T: Config>: ExtrinsicParamsEncoder + Sized + 'static {
    /// These parameters can be provided to the constructor along with
    /// some default parameters that `subxt` understands, in order to
    /// help construct your [`ExtrinsicParams`] object.
    type OtherParams;

    /// The type of error returned from [`ExtrinsicParams::new()`].
    type Error: Into<ExtrinsicParamsError>;

    /// Construct a new instance of our [`ExtrinsicParams`]
    fn new<Client: OfflineClientT<T>>(
        nonce: u64,
        client: Client,
        other_params: Self::OtherParams,
    ) -> Result<Self, Self::Error>;
}

/// This trait is expected to be implemented for any [`ExtrinsicParams`], and
/// defines how to encode the "additional" and "extra" params. Both functions
/// are optional and will encode nothing by default; many signed extensions for
/// instance will only encode one or the other.
pub trait ExtrinsicParamsEncoder: Debug + 'static {
    /// This is expected to SCALE encode the "signed extra" parameters
    /// to some buffer that has been provided. These are the parameters
    /// which are sent along with the transaction, as well as taken into
    /// account when signing the transaction.
    fn encode_extra_to(&self, _v: &mut Vec<u8>) {}

    /// This is expected to SCALE encode the "additional" parameters
    /// to some buffer that has been provided. These parameters are _not_
    /// sent along with the transaction, but are taken into account when
    /// signing it, meaning the client and node must agree on their values.
    fn encode_additional_to(&self, _v: &mut Vec<u8>) {}
}

/// A single [`SignedExtension`] has a unique name, but is otherwise the
/// same as [`ExtrinsicParams`] in describing how to encode the extra and
/// additional data.
pub trait SignedExtension<T: Config>: ExtrinsicParams<T> {
    /// The name of the signed extension. This is used to associate it
    /// with the signed extensions that the node is making use of.
    const NAME: &'static str;
}

/// The [`CheckSpecVersion`] signed extension.
#[derive(Debug)]
pub struct CheckSpecVersion(u32);

impl <T: Config> ExtrinsicParams<T> for CheckSpecVersion {
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

impl <T: Config> SignedExtension<T> for CheckSpecVersion {
    const NAME: &'static str = "CheckSpecVersion";
}

/// The [`CheckNonce`] signed extension.
#[derive(Debug)]
pub struct CheckNonce(Compact<u64>);

impl <T: Config> ExtrinsicParams<T> for CheckNonce {
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

impl <T: Config> SignedExtension<T> for CheckNonce {
    const NAME: &'static str = "CheckNonce";
}

/// The [`CheckTxVersion`] signed extension.
#[derive(Debug)]
pub struct CheckTxVersion(u32);

impl <T: Config> ExtrinsicParams<T> for CheckTxVersion {
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

impl <T: Config> SignedExtension<T> for CheckTxVersion {
    const NAME: &'static str = "CheckTxVersion";
}

/// The [`CheckGenesis`] signed extension.
pub struct CheckGenesis<T: Config>(T::Hash);

impl <T: Config> std::fmt::Debug for CheckGenesis<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("CheckGenesis").field(&self.0).finish()
    }
}

impl <T: Config> ExtrinsicParams<T> for CheckGenesis<T> {
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

impl <T: Config> ExtrinsicParamsEncoder for CheckGenesis<T> {
    fn encode_additional_to(&self, v: &mut Vec<u8>) {
        self.0.encode_to(v);
    }
}

impl <T: Config> SignedExtension<T> for CheckGenesis<T> {
    const NAME: &'static str = "CheckGenesis";
}

/// The [`CheckMortality`] signed extension.
pub struct CheckMortality<T: Config> {
    era: Era,
    checkpoint: T::Hash
}

/// Parameters to configure the [`CheckMortality`] signed extension.
pub struct CheckMortalityParams<T: Config> {
    era: Era,
    checkpoint: Option<T::Hash>
}

impl <T: Config> CheckMortalityParams<T> {
    /// Configure a mortal transaction. The `period` is (roughly) how many
    /// blocks the transaction will be valid for. The `block_number` and
    /// `block_hash` should both point to the same block, and are the block that
    /// the transaction is mortal from.
    pub fn mortal(period: u64, block_number: u64, block_hash: T::Hash) -> Self {
        CheckMortalityParams { era: Era::mortal(period, block_number), checkpoint: Some(block_hash) }
    }
    /// An immortal transaction.
    pub fn immortal() -> Self {
        CheckMortalityParams { era: Era::Immortal, checkpoint: None }
    }
}

impl <T: Config> std::fmt::Debug for CheckMortality<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CheckMortality")
            .field("era", &self.era)
            .field("checkpoint", &self.checkpoint)
            .finish()
    }
}

impl <T: Config> ExtrinsicParams<T> for CheckMortality<T> {
    type OtherParams = CheckMortalityParams<T>;
    type Error = std::convert::Infallible;

    fn new<Client: OfflineClientT<T>>(
        _nonce: u64,
        client: Client,
        other_params: Self::OtherParams,
    ) -> Result<Self, Self::Error> {
        Ok(CheckMortality {
            era: other_params.era,
            checkpoint: other_params.checkpoint.unwrap_or(client.genesis_hash())
        })
    }
}

impl <T: Config> ExtrinsicParamsEncoder for CheckMortality<T> {
    fn encode_extra_to(&self, v: &mut Vec<u8>) {
        self.era.encode_to(v);
    }
    fn encode_additional_to(&self, v: &mut Vec<u8>) {
        self.checkpoint.encode_to(v)
    }
}

impl <T: Config> SignedExtension<T> for CheckMortality<T> {
    const NAME: &'static str = "CheckMortality";
}

/// The [`ChargeAssetTxPayment`] signed extension.
#[derive(Debug)]
pub struct ChargeAssetTxPayment {
    tip: Compact<u128>,
    asset_id: Option<u32>
}

/// Parameters to configure the [`ChargeAssetTxPayment`] signed extension.
pub struct ChargeAssetTxPaymentParams {
    tip: u128,
    asset_id: Option<u32>
}

impl ChargeAssetTxPaymentParams {
    /// Don't provide a tip to the extrinsic author.
    pub fn no_tip() -> Self {
        ChargeAssetTxPaymentParams { tip: 0, asset_id: None }
    }
    /// Tip the extrinsic author in the native chain token.
    pub fn tip(tip: u128) -> Self {
        ChargeAssetTxPaymentParams { tip, asset_id: None }
    }
    /// Tip the extrinsic author using the asset ID given.
    pub fn tip_of(tip: u128, asset_id: u32) -> Self {
        ChargeAssetTxPaymentParams { tip, asset_id: Some(asset_id) }
    }
}

impl <T: Config> ExtrinsicParams<T> for ChargeAssetTxPayment {
    type OtherParams = ChargeAssetTxPaymentParams;
    type Error = std::convert::Infallible;

    fn new<Client: OfflineClientT<T>>(
        _nonce: u64,
        _client: Client,
        other_params: Self::OtherParams,
    ) -> Result<Self, Self::Error> {
        Ok(ChargeAssetTxPayment {
            tip: Compact(other_params.tip),
            asset_id: other_params.asset_id
        })
    }
}

impl ExtrinsicParamsEncoder for ChargeAssetTxPayment {
    fn encode_extra_to(&self, v: &mut Vec<u8>) {
        (self.tip, self.asset_id).encode_to(v);
    }
}

impl <T: Config> SignedExtension<T> for ChargeAssetTxPayment {
    const NAME: &'static str = "ChargeAssetTxPayment";
}

/// The [`ChargeAssetTxPayment`] signed extension.
#[derive(Debug)]
pub struct ChargeTransactionPayment {
    tip: Compact<u128>,
}

/// Parameters to configure the [`ChargeTransactionPayment`] signed extension.
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

impl <T: Config> ExtrinsicParams<T> for ChargeTransactionPayment {
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

impl <T: Config> SignedExtension<T> for ChargeTransactionPayment {
    const NAME: &'static str = "ChargeTransactionPayment";
}

/// This accepts a tuple of [`SignedExtension`]s, and will dynamically make use of whichever
/// ones are actually required for the chain in the correct order, ignoring the rest. This
/// is a sensible default, and allows for a single configuration to work across multiple chains.
pub struct DynamicExtrinsicParams<T, Params> {
    params: Vec<Box<dyn ExtrinsicParamsEncoder>>,
    _marker: std::marker::PhantomData<(T, Params)>
}

impl <T, Params> std::fmt::Debug for DynamicExtrinsicParams<T, Params> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynamicExtrinsicParams")
            .field("params", &self.params)
            .field("_marker", &"std::marker::PhantomData<T>")
            .finish()
    }
}

/// This accepts a tuple of [`SignedExtension`]s, and will ignore the metadata of the chain we're
/// connected to and simply encode exactly the additional and extra data of each of the extensions
/// in the order that they are provided. Prefer to use [`DynamicExtrinsicParams`].
pub struct StaticExtrinsicParams<T, Params> {
    params: Params,
    _marker: std::marker::PhantomData<T>
}

impl <T, Params> std::fmt::Debug for StaticExtrinsicParams<T, Params> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StaticExtrinsicParams")
            .field("params", &"<tuple of params>")
            .field("_marker", &"std::marker::PhantomData<T>")
            .finish()
    }
}

macro_rules! impl_tuples {
    ($($ident:ident $index:tt),+) => {
        // We do some magic when the tuple is wrapped in DynamicExtrinsicParams. We
        // look at the metadata, and use this to select and make use of only the extensions
        // that we actually need for the chain we're dealing with.
        impl <T, $($ident),+> ExtrinsicParams<T> for DynamicExtrinsicParams<T, ($($ident,)+)>
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

                Ok(DynamicExtrinsicParams {
                    params: params,
                    _marker: std::marker::PhantomData
                })
            }
        }

        impl <T, $($ident),+> ExtrinsicParamsEncoder for DynamicExtrinsicParams<T, ($($ident,)+)>
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

        // If we know exactly the structure of signed extensions that we need, and we don't want to
        // use the node metadata to decide on which to encode, then we can instead provide a StaticExtrinsicParams
        // wrapping a tuple of signed extensions. Here, using a tuple of 1 signed extension has the same behaviour
        // as the signed extension on its own. Using `DynamicExtrinsicParams` is strongly preferred, but this is a
        // little faster if you know precisely the signed extensions that the chain needs.
        impl <T, $($ident),+> ExtrinsicParams<T> for StaticExtrinsicParams<T, ($($ident,)+)>
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
                Ok(StaticExtrinsicParams {
                    params: (
                        $($ident::new(nonce, client.clone(), other_params.$index).map_err(Into::into)?,)+
                    ),
                    _marker: std::marker::PhantomData
                })
            }
        }

        impl <T, $($ident),+> ExtrinsicParamsEncoder for StaticExtrinsicParams<T, ($($ident,)+)>
        where
            T: Config,
            $($ident: SignedExtension<T>,)+
        {
            fn encode_extra_to(&self, v: &mut Vec<u8>) {
                $(
                    self.params.$index.encode_extra_to(v);
                )+
            }
            fn encode_additional_to(&self, v: &mut Vec<u8>) {
                $(
                    self.params.$index.encode_additional_to(v);
                )+
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

/// Tries to encode an empty tuple into the type given. Returns true if this
/// succeeds, and thus if the type given is empty (and compatible with an empty tuple)
fn is_type_empty(type_id: u32, types: &scale_info::PortableRegistry) -> bool {
    use scale_encode::EncodeAsType;
    ().encode_as_type(type_id, types).is_ok()
}
