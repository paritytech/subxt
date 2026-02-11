// Copyright 2019-2026 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! This module contains implementations for common transaction extensions, each
//! of which implements [`TransactionExtension`], and can be used in conjunction with
//! [`AnyOf`] to configure the set of transaction extensions which are known about
//! when interacting with a chain.

use crate::config::{Config, HashFor, ClientState, TransactionExtension, Params};
use crate::error::ExtrinsicParamsError;
use crate::utils::{Era, Static};
use codec::{Compact, Encode};
use core::any::Any;
use core::fmt::Debug;
use derive_where::derive_where;
use scale_decode::DecodeAsType;
use scale_info::PortableRegistry;
use std::collections::HashMap;

/// The [`VerifySignature`] extension. For V5 General transactions, this is how a signature
/// is provided. The signature is constructed by signing a payload which contains the
/// transaction call data as well as the encoded "additional" bytes for any extensions _after_
/// this one in the list.
pub struct VerifySignature<T: Config>(VerifySignatureDetails<T>);

impl<T: Config> TransactionExtension<T> for VerifySignature<T> {
    type Params = ();
    type Decoded = Static<VerifySignatureDetails<T>>;

    fn new(_client: &ClientState<T>, _params: Self::Params) -> Result<Self, ExtrinsicParamsError> {
        Ok(VerifySignature(VerifySignatureDetails::Disabled))
    }
}

impl<T: Config> frame_decode::extrinsics::TransactionExtension for VerifySignature<T> {
    fn extension_name(&self) -> &'static str {
        "VerifySignature"
    }
    fn encode_value_to(&self, v: &mut Vec<u8>) {
        self.0.encode_to(v);
    }
    fn encode_signer_payload_value_to(&self, v: &mut Vec<u8>) {
        // This extension is never encoded to the signer payload, and extensions
        // prior to this are ignored when creating said payload, so clear anything
        // we've seen so far.
        v.clear();
    }
    fn encode_implicit_to(&self, v: &mut Vec<u8>) {
        // We only use the "implicit" data for extensions _after_ this one
        // in the pipeline to form the signer payload. Thus, clear anything
        // we've seen so far.
        v.clear();
    }

    fn inject_signature(&mut self, account: &dyn Any, signature: &dyn Any) {
        // Downcast refs back to concrete types (we use `&dyn Any`` so that the trait remains object safe)
        let account = account
            .downcast_ref::<T::AccountId>()
            .expect("A T::AccountId should have been provided")
            .clone();
        let signature = signature
            .downcast_ref::<T::Signature>()
            .expect("A T::Signature should have been provided")
            .clone();

        // The signature is not set through params, only here, once given by a user:
        self.0 = VerifySignatureDetails::Signed { signature, account }
    }
}

impl<T: Config> TransactionExtension<T> for VerifySignature<T> {
    type Decoded = Static<VerifySignatureDetails<T>>;
    fn matches(identifier: &str, _type_id: u32, _types: &PortableRegistry) -> bool {
        identifier == "VerifySignature"
    }
}

/// This allows a signature to be provided to the [`VerifySignature`] transaction extension.
// Dev note: this must encode identically to https://github.com/paritytech/polkadot-sdk/blob/fd72d58313c297a10600037ce1bb88ec958d722e/substrate/frame/verify-signature/src/extension.rs#L43
#[derive(codec::Encode, codec::Decode)]
pub enum VerifySignatureDetails<T: Config> {
    /// A signature has been provided.
    Signed {
        /// The signature.
        signature: T::Signature,
        /// The account that generated the signature.
        account: T::AccountId,
    },
    /// No signature was provided.
    Disabled,
}

/// The [`CheckMetadataHash`] transaction extension.
pub struct CheckMetadataHash {
    // Eventually we might provide or calculate the metadata hash here,
    // but for now we never provide a hash and so this is empty.
}

impl<T: Config> ExtrinsicParams<T> for CheckMetadataHash {
    type Params = ();

    fn new(_client: &ClientState<T>, _params: Self::Params) -> Result<Self, ExtrinsicParamsError> {
        Ok(CheckMetadataHash {})
    }
}

impl ExtrinsicParamsEncoder for CheckMetadataHash {
    fn encode_value_to(&self, v: &mut Vec<u8>) {
        // A single 0 byte in the TX payload indicates that the chain should
        // _not_ expect any metadata hash to exist in the signer payload.
        0u8.encode_to(v);
    }
    fn encode_implicit_to(&self, v: &mut Vec<u8>) {
        // We provide no metadata hash in the signer payload to align with the above.
        None::<()>.encode_to(v);
    }
}

impl<T: Config> TransactionExtension<T> for CheckMetadataHash {
    type Decoded = CheckMetadataHashMode;
    fn matches(identifier: &str, _type_id: u32, _types: &PortableRegistry) -> bool {
        identifier == "CheckMetadataHash"
    }
}

/// Is metadata checking enabled or disabled?
// Dev note: The "Disabled" and "Enabled" variant names match those that the
// transaction extension will be encoded with, in order that DecodeAsType will work
// properly.
#[derive(Copy, Clone, Debug, DecodeAsType)]
pub enum CheckMetadataHashMode {
    /// No hash was provided in the signer payload.
    Disabled,
    /// A hash was provided in the signer payload.
    Enabled,
}

impl CheckMetadataHashMode {
    /// Is metadata checking enabled or disabled for this transaction?
    pub fn is_enabled(&self) -> bool {
        match self {
            CheckMetadataHashMode::Disabled => false,
            CheckMetadataHashMode::Enabled => true,
        }
    }
}

/// The [`CheckSpecVersion`] transaction extension.
pub struct CheckSpecVersion(u32);

impl<T: Config> ExtrinsicParams<T> for CheckSpecVersion {
    type Params = ();

    fn new(client: &ClientState<T>, _params: Self::Params) -> Result<Self, ExtrinsicParamsError> {
        Ok(CheckSpecVersion(client.spec_version))
    }
}

impl ExtrinsicParamsEncoder for CheckSpecVersion {
    fn encode_implicit_to(&self, v: &mut Vec<u8>) {
        self.0.encode_to(v);
    }
}

impl<T: Config> TransactionExtension<T> for CheckSpecVersion {
    type Decoded = ();
    fn matches(identifier: &str, _type_id: u32, _types: &PortableRegistry) -> bool {
        identifier == "CheckSpecVersion"
    }
}

/// The [`CheckNonce`] transaction extension.
pub struct CheckNonce(u64);

impl<T: Config> ExtrinsicParams<T> for CheckNonce {
    type Params = CheckNonceParams;

    fn new(_client: &ClientState<T>, params: Self::Params) -> Result<Self, ExtrinsicParamsError> {
        Ok(CheckNonce(params.0.unwrap_or(0)))
    }
}

impl ExtrinsicParamsEncoder for CheckNonce {
    fn encode_value_to(&self, v: &mut Vec<u8>) {
        Compact(self.0).encode_to(v);
    }
}

impl<T: Config> TransactionExtension<T> for CheckNonce {
    type Decoded = u64;
    fn matches(identifier: &str, _type_id: u32, _types: &PortableRegistry) -> bool {
        identifier == "CheckNonce"
    }
}

/// Configure the nonce used.
#[derive(Debug, Clone, Default)]
pub struct CheckNonceParams(Option<u64>);

impl CheckNonceParams {
    /// Retrieve the nonce from the chain and use that.
    pub fn from_chain() -> Self {
        Self(None)
    }
    /// Manually set an account nonce to use.
    pub fn with_nonce(nonce: u64) -> Self {
        Self(Some(nonce))
    }
}

impl<T: Config> Params<T> for CheckNonceParams {
    fn inject_account_nonce(&mut self, nonce: u64) {
        if self.0.is_none() {
            self.0 = Some(nonce)
        }
    }
}

/// The [`CheckTxVersion`] transaction extension.
pub struct CheckTxVersion(u32);

impl<T: Config> ExtrinsicParams<T> for CheckTxVersion {
    type Params = ();

    fn new(client: &ClientState<T>, _params: Self::Params) -> Result<Self, ExtrinsicParamsError> {
        Ok(CheckTxVersion(client.transaction_version))
    }
}

impl ExtrinsicParamsEncoder for CheckTxVersion {
    fn encode_implicit_to(&self, v: &mut Vec<u8>) {
        self.0.encode_to(v);
    }
}

impl<T: Config> TransactionExtension<T> for CheckTxVersion {
    type Decoded = ();
    fn matches(identifier: &str, _type_id: u32, _types: &PortableRegistry) -> bool {
        identifier == "CheckTxVersion"
    }
}

/// The [`CheckGenesis`] transaction extension.
pub struct CheckGenesis<T: Config>(HashFor<T>);

impl<T: Config> ExtrinsicParams<T> for CheckGenesis<T> {
    type Params = ();

    fn new(client: &ClientState<T>, _params: Self::Params) -> Result<Self, ExtrinsicParamsError> {
        Ok(CheckGenesis(client.genesis_hash))
    }
}

impl<T: Config> ExtrinsicParamsEncoder for CheckGenesis<T> {
    fn encode_implicit_to(&self, v: &mut Vec<u8>) {
        self.0.encode_to(v);
    }
}

impl<T: Config> TransactionExtension<T> for CheckGenesis<T> {
    type Decoded = ();
    fn matches(identifier: &str, _type_id: u32, _types: &PortableRegistry) -> bool {
        identifier == "CheckGenesis"
    }
}

/// The [`CheckMortality`] transaction extension.
pub struct CheckMortality<T: Config> {
    params: CheckMortalityParamsInner<T>,
    genesis_hash: HashFor<T>,
}

impl<T: Config> ExtrinsicParams<T> for CheckMortality<T> {
    type Params = CheckMortalityParams<T>;

    fn new(client: &ClientState<T>, params: Self::Params) -> Result<Self, ExtrinsicParamsError> {
        // If a user has explicitly configured the transaction to be mortal for n blocks, but we get
        // to this stage and no injected information was able to turn this into MortalFromBlock{..},
        // then we hit an error as we are unable to construct a mortal transaction here.
        if matches!(&params.0, CheckMortalityParamsInner::MortalForBlocks(_)) {
            return Err(ExtrinsicParamsError::custom(
                "CheckMortality: We cannot construct an offline extrinsic with only the number of blocks it is mortal for. Use mortal_from_unchecked instead.",
            ));
        }

        Ok(CheckMortality {
            // if nothing has been explicitly configured, we will have a mortal transaction
            // valid for 32 blocks if block info is available.
            params: params.0,
            genesis_hash: client.genesis_hash,
        })
    }
}

impl<T: Config> ExtrinsicParamsEncoder for CheckMortality<T> {
    fn encode_value_to(&self, v: &mut Vec<u8>) {
        match &self.params {
            CheckMortalityParamsInner::MortalFromBlock {
                for_n_blocks,
                from_block_n,
                ..
            } => {
                Era::mortal(*for_n_blocks, *from_block_n).encode_to(v);
            }
            _ => {
                // Note: if we see `CheckMortalityInner::MortalForBlocks`, then it means the user has
                // configured a block to be mortal for N blocks, but the current block was never injected,
                // so we don't know where to start from and default back to building an immortal tx.
                Era::Immortal.encode_to(v);
            }
        }
    }
    fn encode_implicit_to(&self, v: &mut Vec<u8>) {
        match &self.params {
            CheckMortalityParamsInner::MortalFromBlock {
                from_block_hash, ..
            } => {
                from_block_hash.encode_to(v);
            }
            _ => {
                self.genesis_hash.encode_to(v);
            }
        }
    }
}

impl<T: Config> TransactionExtension<T> for CheckMortality<T> {
    type Decoded = Era;
    fn matches(identifier: &str, _type_id: u32, _types: &PortableRegistry) -> bool {
        identifier == "CheckMortality"
    }
}

/// Parameters to configure the [`CheckMortality`] transaction extension.
pub struct CheckMortalityParams<T: Config>(CheckMortalityParamsInner<T>);

enum CheckMortalityParamsInner<T: Config> {
    /// The transaction will be immortal.
    Immortal,
    /// The transaction is mortal for N blocks. This must be "upgraded" into
    /// [`CheckMortalityParamsInner::MortalFromBlock`] to ultimately work.
    MortalForBlocks(u64),
    /// The transaction is mortal for N blocks, but if it cannot be "upgraded",
    /// then it will be set to immortal instead. This is the default if unset.
    MortalForBlocksOrImmortalIfNotPossible(u64),
    /// The transaction is mortal and all of the relevant information is provided.
    MortalFromBlock {
        for_n_blocks: u64,
        from_block_n: u64,
        from_block_hash: HashFor<T>,
    },
}

impl<T: Config> Default for CheckMortalityParams<T> {
    fn default() -> Self {
        // default to being mortal for 32 blocks if possible, else immortal:
        CheckMortalityParams(CheckMortalityParamsInner::MortalForBlocksOrImmortalIfNotPossible(32))
    }
}

impl<T: Config> CheckMortalityParams<T> {
    /// Configure a transaction that will be mortal for the number of blocks given.
    pub fn mortal(for_n_blocks: u64) -> Self {
        Self(CheckMortalityParamsInner::MortalForBlocks(for_n_blocks))
    }

    /// Configure a transaction that will be mortal for the number of blocks given,
    /// and from the block details provided. Prefer to use [`CheckMortalityParams::mortal()`]
    /// where possible, which prevents the block number and hash from being misaligned.
    pub fn mortal_from_unchecked(
        for_n_blocks: u64,
        from_block_n: u64,
        from_block_hash: HashFor<T>,
    ) -> Self {
        Self(CheckMortalityParamsInner::MortalFromBlock {
            for_n_blocks,
            from_block_n,
            from_block_hash,
        })
    }
    /// An immortal transaction.
    pub fn immortal() -> Self {
        Self(CheckMortalityParamsInner::Immortal)
    }
}

impl<T: Config> Params<T> for CheckMortalityParams<T> {
    fn inject_block(&mut self, from_block_n: u64, from_block_hash: HashFor<T>) {
        match &self.0 {
            CheckMortalityParamsInner::MortalForBlocks(n)
            | CheckMortalityParamsInner::MortalForBlocksOrImmortalIfNotPossible(n) => {
                self.0 = CheckMortalityParamsInner::MortalFromBlock {
                    for_n_blocks: *n,
                    from_block_n,
                    from_block_hash,
                }
            }
            _ => {
                // Don't change anything if explicit Immortal or explicit block set.
            }
        }
    }
}

/// The [`ChargeAssetTxPayment`] transaction extension.
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

impl<T: Config> ExtrinsicParams<T> for ChargeAssetTxPayment<T> {
    type Params = ChargeAssetTxPaymentParams<T>;

    fn new(_client: &ClientState<T>, params: Self::Params) -> Result<Self, ExtrinsicParamsError> {
        Ok(ChargeAssetTxPayment {
            tip: Compact(params.tip),
            asset_id: params.asset_id,
        })
    }
}

impl<T: Config> ExtrinsicParamsEncoder for ChargeAssetTxPayment<T> {
    fn encode_value_to(&self, v: &mut Vec<u8>) {
        (self.tip, &self.asset_id).encode_to(v);
    }
}

impl<T: Config> TransactionExtension<T> for ChargeAssetTxPayment<T> {
    type Decoded = Self;
    fn matches(identifier: &str, _type_id: u32, _types: &PortableRegistry) -> bool {
        identifier == "ChargeAssetTxPayment"
    }
}

/// Parameters to configure the [`ChargeAssetTxPayment`] transaction extension.
#[cfg_attr(test, derive_where(PartialEq; T::AssetId))]
#[derive_where(Debug)]
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

impl<T: Config> Params<T> for ChargeAssetTxPaymentParams<T> {}

/// The [`ChargeTransactionPayment`] transaction extension.
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

impl<T: Config> ExtrinsicParams<T> for ChargeTransactionPayment {
    type Params = ChargeTransactionPaymentParams;

    fn new(_client: &ClientState<T>, params: Self::Params) -> Result<Self, ExtrinsicParamsError> {
        Ok(ChargeTransactionPayment {
            tip: Compact(params.tip),
        })
    }
}

impl ExtrinsicParamsEncoder for ChargeTransactionPayment {
    fn encode_value_to(&self, v: &mut Vec<u8>) {
        self.tip.encode_to(v);
    }
}

impl<T: Config> TransactionExtension<T> for ChargeTransactionPayment {
    type Decoded = Self;
    fn matches(identifier: &str, _type_id: u32, _types: &PortableRegistry) -> bool {
        identifier == "ChargeTransactionPayment"
    }
}

/// Parameters to configure the [`ChargeTransactionPayment`] transaction extension.
#[cfg_attr(test, derive(PartialEq))]
#[derive(Default, Debug)]
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

impl<T: Config> Params<T> for ChargeTransactionPaymentParams {}

/// This accepts a tuple of [`TransactionExtension`]s, and will dynamically make use of whichever
/// ones are actually required for the chain in the correct order, ignoring the rest. This
/// is a sensible default, and allows for a single configuration to work across multiple chains.
pub struct AnyOf<T, Params> {
    params: Vec<Box<dyn ExtrinsicParamsEncoder + Send + 'static>>,
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
            $($ident: TransactionExtension<T>,)+
        {
            type Params = ($($ident::Params,)+);

            fn new(
                client: &ClientState<T>,
                params: Self::Params,
            ) -> Result<Self, ExtrinsicParamsError> {
                let metadata = &client.metadata;
                let types = metadata.types();

                // For each transaction extension in the tuple, find the matching index in the metadata, if
                // there is one, and add it to a map with that index as the key.
                let mut exts_by_index = HashMap::new();
                $({
                    for (idx, e) in metadata.extrinsic().transaction_extensions_to_use_for_encoding().enumerate() {
                        // Skip over any exts that have a match already:
                        if exts_by_index.contains_key(&idx) {
                            continue
                        }
                        // Break and record as soon as we find a match:
                        if $ident::matches(e.identifier(), e.extra_ty(), types) {
                            let ext = $ident::new(client, params.$index)?;
                            let boxed_ext: Box<dyn ExtrinsicParamsEncoder + Send + 'static> = Box::new(ext);
                            exts_by_index.insert(idx, boxed_ext);
                            break
                        }
                    }
                })+

                // Next, turn these into an ordered vec, erroring if we haven't matched on any exts yet.
                let mut params = Vec::new();
                for (idx, e) in metadata.extrinsic().transaction_extensions_to_use_for_encoding().enumerate() {
                    let Some(ext) = exts_by_index.remove(&idx) else {
                        if is_type_empty(e.extra_ty(), types) {
                            continue
                        } else {
                            return Err(ExtrinsicParamsError::UnknownTransactionExtension(e.identifier().to_owned()));
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
            $($ident: TransactionExtension<T>,)+
        {
            fn inject_signature(&mut self, account_id: &dyn Any, signature: &dyn Any) {
                for ext in &mut self.params {
                    ext.inject_signature(account_id, signature);
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
