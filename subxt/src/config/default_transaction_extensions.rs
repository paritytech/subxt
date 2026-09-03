// Copyright 2019-2026 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

use super::{Config, HashFor, TransactionExtensions, transaction_extensions};
use crate::config::transaction_extension_traits::Params;
use crate::config::transaction_extensions::CheckMortalityParams;
use crate::error::TransactionExtensionError;
use crate::transactions::DefaultParams;
use scale_encode::EncodeAsType;
use scale_info::PortableRegistry;
use scale_value::Value;
use std::collections::BTreeMap;

/// The known transaction extensions used by [`DefaultTransactionExtensions`].
///
/// This is exposed for users who need to manually compose the default typed extensions.
pub type KnownDefaultTransactionExtensions<T> = (
    transaction_extensions::VerifySignature<T>,
    transaction_extensions::CheckSpecVersion,
    transaction_extensions::CheckTxVersion,
    transaction_extensions::CheckNonce,
    transaction_extensions::CheckGenesis<T>,
    transaction_extensions::CheckMortality<T>,
    transaction_extensions::ChargeAssetTxPayment<T>,
    transaction_extensions::ChargeTransactionPayment,
    transaction_extensions::CheckMetadataHash,
);

/// The parameters used to construct [`KnownDefaultTransactionExtensions`].
pub type KnownDefaultExtrinsicParams<T> =
    <KnownDefaultTransactionExtensions<T> as TransactionExtensions<T>>::Params;

/// The default set of transaction extensions, along with any custom extensions supplied for a
/// specific transaction.
pub struct DefaultTransactionExtensions<T: Config> {
    known: KnownDefaultTransactionExtensions<T>,
    custom: BTreeMap<String, Value>,
}

/// Parameters used to construct [`DefaultTransactionExtensions`].
pub struct DefaultExtrinsicParams<T: Config> {
    known: KnownDefaultExtrinsicParams<T>,
    custom: Vec<(String, Value)>,
}

impl<T: Config> DefaultExtrinsicParams<T> {
    /// Construct parameters from the known default transaction extension parameters.
    pub fn from_known(known: KnownDefaultExtrinsicParams<T>) -> Self {
        Self {
            known,
            custom: Vec::new(),
        }
    }

    /// Return the parameters for the known default transaction extensions.
    pub fn known(&self) -> &KnownDefaultExtrinsicParams<T> {
        &self.known
    }

    /// Return a mutable reference to the parameters for the known default
    /// transaction extensions.
    pub fn known_mut(&mut self) -> &mut KnownDefaultExtrinsicParams<T> {
        &mut self.known
    }

    /// Return the values provided for custom transaction extensions.
    pub fn custom(&self) -> &[(String, Value)] {
        &self.custom
    }
}

impl<T: Config> Default for DefaultExtrinsicParams<T> {
    fn default() -> Self {
        DefaultExtrinsicParamsBuilder::new().build()
    }
}

impl<T: Config> DefaultParams for DefaultExtrinsicParams<T> {
    fn default_params() -> Self {
        Self::default()
    }
}

impl<T: Config> Params<T> for DefaultExtrinsicParams<T> {
    fn inject_account_nonce(&mut self, nonce: u64) {
        self.known.inject_account_nonce(nonce);
    }

    fn inject_block(&mut self, number: u64, hash: HashFor<T>) {
        self.known.inject_block(number, hash);
    }
}

impl<T: Config> TransactionExtensions<T> for DefaultTransactionExtensions<T> {
    type Params = DefaultExtrinsicParams<T>;

    fn new(
        client: &super::ClientState<T>,
        params: Self::Params,
    ) -> Result<Self, TransactionExtensionError> {
        let known = <KnownDefaultTransactionExtensions<T> as TransactionExtensions<T>>::new(
            client,
            params.known,
        )?;
        let mut custom = BTreeMap::new();

        for (name, value) in params.custom {
            if frame_decode::extrinsics::TransactionExtensions::contains_extension(&known, &name) {
                return Err(TransactionExtensionError::custom(format!(
                    "Custom transaction extension '{name}' conflicts with a known transaction extension"
                )));
            }
            if custom.contains_key(&name) {
                return Err(TransactionExtensionError::custom(format!(
                    "Custom transaction extension '{name}' was provided more than once"
                )));
            }
            let in_metadata = client
                .metadata
                .extrinsic()
                .transaction_extensions_to_use_for_encoding()
                .any(|extension| extension.identifier() == name);
            if !in_metadata {
                return Err(TransactionExtensionError::custom(format!(
                    "Custom transaction extension '{name}' is not present in the runtime metadata"
                )));
            }
            custom.insert(name, value);
        }

        Ok(Self { known, custom })
    }

    fn inject_signature(&mut self, account_id: &T::AccountId, signature: &T::Signature) {
        self.known.inject_signature(account_id, signature);
    }
}

impl<T: Config> DefaultTransactionExtensions<T> {
    fn encode_custom_value_to(
        &self,
        name: &str,
        type_id: u32,
        type_resolver: &PortableRegistry,
        out: &mut Vec<u8>,
    ) -> Result<(), frame_decode::extrinsics::TransactionExtensionsError> {
        let value = self.custom.get(name).ok_or_else(|| {
            frame_decode::extrinsics::TransactionExtensionsError::NotFound(name.to_owned())
        })?;
        let original_len = out.len();
        let result = value.encode_as_type_to(type_id, type_resolver, out);
        result.map_err(|error| {
            out.truncate(original_len);
            frame_decode::extrinsics::TransactionExtensionsError::Other {
                extension_name: name.to_owned(),
                error: Box::new(error),
            }
        })
    }
}

impl<T: Config> frame_decode::extrinsics::TransactionExtensions<PortableRegistry>
    for DefaultTransactionExtensions<T>
{
    fn contains_extension(&self, name: &str) -> bool {
        frame_decode::extrinsics::TransactionExtensions::contains_extension(&self.known, name)
            || self.custom.contains_key(name)
    }

    fn is_authorization_extension(&self, name: &str) -> bool {
        frame_decode::extrinsics::TransactionExtensions::is_authorization_extension(
            &self.known,
            name,
        )
    }

    fn encode_extension_value_to(
        &self,
        name: &str,
        type_id: u32,
        type_resolver: &PortableRegistry,
        out: &mut Vec<u8>,
    ) -> Result<(), frame_decode::extrinsics::TransactionExtensionsError> {
        if frame_decode::extrinsics::TransactionExtensions::contains_extension(&self.known, name) {
            frame_decode::extrinsics::TransactionExtensions::encode_extension_value_to(
                &self.known,
                name,
                type_id,
                type_resolver,
                out,
            )
        } else {
            self.encode_custom_value_to(name, type_id, type_resolver, out)
        }
    }

    fn encode_extension_implicit_to(
        &self,
        name: &str,
        type_id: u32,
        type_resolver: &PortableRegistry,
        out: &mut Vec<u8>,
    ) -> Result<(), frame_decode::extrinsics::TransactionExtensionsError> {
        if frame_decode::extrinsics::TransactionExtensions::contains_extension(&self.known, name) {
            frame_decode::extrinsics::TransactionExtensions::encode_extension_implicit_to(
                &self.known,
                name,
                type_id,
                type_resolver,
                out,
            )
        } else if self.custom.contains_key(name) {
            Err(frame_decode::extrinsics::TransactionExtensionsError::Other {
                extension_name: name.to_owned(),
                error: format!(
                    "Custom transaction extension '{name}' requires non-empty implicit data, which is not supported"
                )
                .into(),
            })
        } else {
            Err(frame_decode::extrinsics::TransactionExtensionsError::NotFound(name.to_owned()))
        }
    }
}

/// A builder that outputs the set of parameters required to configure transactions when
/// [`DefaultTransactionExtensions`] is used. This may expose methods that aren't applicable
/// to the current chain; such values will simply be ignored if so.
pub struct DefaultExtrinsicParamsBuilder<T: Config> {
    /// `None` means the tx will be immortal, else it's mortality is described.
    mortality: transaction_extensions::CheckMortalityParams<T>,
    /// `None` means the nonce will be automatically set.
    nonce: Option<u64>,
    /// `None` means we'll use the native token.
    tip_of_asset_id: Option<T::AssetId>,
    tip_of: u128,
    /// A fallback tip used when no Asset ID is given (or the chain doesn't support it).
    tip: u128,
    custom: Vec<(String, Value)>,
}

impl<T: Config> Default for DefaultExtrinsicParamsBuilder<T> {
    fn default() -> Self {
        Self {
            mortality: CheckMortalityParams::<T>::default(),
            tip: 0,
            tip_of: 0,
            tip_of_asset_id: None,
            nonce: None,
            custom: Vec::new(),
        }
    }
}

impl<T: Config> DefaultExtrinsicParamsBuilder<T> {
    /// Configure new extrinsic params. We default to providing no tip
    /// and using an immortal transaction unless otherwise configured
    pub fn new() -> Self {
        Default::default()
    }

    /// Make the transaction immortal, meaning it will never expire. This means that it could, in
    /// theory, be pending for a long time and only be included many blocks into the future.
    pub fn immortal(mut self) -> Self {
        self.mortality = transaction_extensions::CheckMortalityParams::<T>::immortal();
        self
    }

    /// Make the transaction mortal, given a number of blocks it will be mortal for from
    /// the current block at the time of submission.
    ///
    /// # Warning
    ///
    /// This will ultimately return an error if used for creating extrinsic offline, because we need
    /// additional information in order to set the mortality properly.
    ///
    /// When creating offline transactions, you must use [`Self::mortal_from_unchecked`] instead to set
    /// the mortality. This provides all of the necessary information which we must otherwise be online
    /// in order to obtain.
    pub fn mortal(mut self, for_n_blocks: u64) -> Self {
        self.mortality = transaction_extensions::CheckMortalityParams::<T>::mortal(for_n_blocks);
        self
    }

    /// Configure a transaction that will be mortal for the number of blocks given, and from the
    /// block details provided. Prefer to use [`Self::mortal()`] where possible, which prevents
    /// the block number and hash from being misaligned.
    pub fn mortal_from_unchecked(
        mut self,
        for_n_blocks: u64,
        from_block_n: u64,
        from_block_hash: HashFor<T>,
    ) -> Self {
        self.mortality = transaction_extensions::CheckMortalityParams::mortal_from_unchecked(
            for_n_blocks,
            from_block_n,
            from_block_hash,
        );
        self
    }

    /// Provide a specific nonce for the submitter of the extrinsic
    pub fn nonce(mut self, nonce: u64) -> Self {
        self.nonce = Some(nonce);
        self
    }

    /// Provide a tip to the block author in the chain's native token.
    pub fn tip(mut self, tip: u128) -> Self {
        self.tip = tip;
        self.tip_of = tip;
        self.tip_of_asset_id = None;
        self
    }

    /// Provide a tip to the block author using the token denominated by the `asset_id` provided. This
    /// is not applicable on chains which don't use the `ChargeAssetTxPayment` signed extension; in this
    /// case, you can also call [`Self::tip`] to configure a tip in the native asset in case this is not
    /// applicable.
    pub fn tip_of(mut self, tip: u128, asset_id: T::AssetId) -> Self {
        self.tip_of = tip;
        self.tip_of_asset_id = Some(asset_id);
        self
    }

    /// Provide a metadata-aware value for a custom transaction extension.
    ///
    /// This is for extensions that a chain declares but Subxt has no typed support for; the
    /// value given here is encoded using the type information in the runtime metadata.
    ///
    /// Extensions absent from runtime metadata are rejected, as are known or duplicate names
    /// and extensions with non-empty implicit data. Custom authorization extensions are
    /// unsupported.
    ///
    /// # Example
    ///
    /// ```rust
    /// use subxt::config::{DefaultExtrinsicParamsBuilder, PolkadotConfig};
    ///
    /// // The name must match an extension identifier in the chain's metadata, and the value
    /// // must encode to the type that the metadata declares for it.
    /// let params = DefaultExtrinsicParamsBuilder::<PolkadotConfig>::new()
    ///     .tip(100)
    ///     .custom_extension("MyCustomExtension", true)
    ///     .build();
    ///
    /// assert_eq!(params.custom(), [("MyCustomExtension".to_owned(), true.into())]);
    /// ```
    pub fn custom_extension(mut self, name: impl Into<String>, value: impl Into<Value>) -> Self {
        self.custom.push((name.into(), value.into()));
        self
    }

    /// Build the extrinsic parameters.
    pub fn build(self) -> DefaultExtrinsicParams<T> {
        let check_mortality_params = self.mortality;

        let charge_asset_tx_params = if let Some(asset_id) = self.tip_of_asset_id {
            transaction_extensions::ChargeAssetTxPaymentParams::tip_of(self.tip_of, asset_id)
        } else {
            transaction_extensions::ChargeAssetTxPaymentParams::tip(self.tip_of)
        };

        let charge_transaction_params =
            transaction_extensions::ChargeTransactionPaymentParams::tip(self.tip);

        let check_nonce_params = if let Some(nonce) = self.nonce {
            transaction_extensions::CheckNonceParams::with_nonce(nonce)
        } else {
            transaction_extensions::CheckNonceParams::from_chain()
        };

        DefaultExtrinsicParams {
            known: (
                (),
                (),
                (),
                check_nonce_params,
                (),
                check_mortality_params,
                charge_asset_tx_params,
                charge_transaction_params,
                (),
            ),
            custom: self.custom,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::polkadot::H256;
    use crate::config::{ClientState, PolkadotConfig};
    use crate::metadata::Metadata;
    use crate::utils::{AccountId32, MultiSignature};
    use assert_matches::assert_matches;
    use codec::Decode;
    use frame_decode::extrinsics::{
        ExtrinsicCallInfo, ExtrinsicEncodeError, ExtrinsicExtensionInfo, ExtrinsicExtensionInfoArg,
        ExtrinsicSignatureInfo, TransactionExtensionsError,
    };
    use scale_info::{MetaType, Registry};
    use scale_value::Composite;
    use std::borrow::Cow;
    use std::sync::Arc;

    fn assert_default<T: Default>(_t: T) {}

    fn client_state() -> ClientState<PolkadotConfig> {
        let metadata = Metadata::decode(
            &mut &include_bytes!("../../../artifacts/polkadot_metadata_small.scale")[..],
        )
        .unwrap();

        ClientState {
            genesis_hash: H256::zero(),
            spec_version: 0,
            transaction_version: 0,
            metadata: Arc::new(metadata),
        }
    }

    fn type_info<T: scale_info::TypeInfo + 'static>() -> (u32, PortableRegistry) {
        let mut types = Registry::new();
        let id = types.register_type(&MetaType::new::<T>());
        (id.id, types.into())
    }

    fn encoding_info() -> (
        ExtrinsicCallInfo<'static, u32>,
        ExtrinsicExtensionInfo<'static, u32>,
        ExtrinsicSignatureInfo<u32>,
        PortableRegistry,
    ) {
        let mut types = Registry::new();
        let bool_id = types.register_type(&MetaType::new::<bool>()).id;
        let unit_id = types.register_type(&MetaType::new::<()>()).id;
        let u8_id = types.register_type(&MetaType::new::<u8>()).id;

        (
            ExtrinsicCallInfo {
                pallet_index: 1,
                call_index: 2,
                pallet_name: Cow::Borrowed("Test"),
                call_name: Cow::Borrowed("call"),
                args: Vec::new(),
            },
            ExtrinsicExtensionInfo {
                extension_ids: vec![ExtrinsicExtensionInfoArg {
                    name: Cow::Borrowed("CheckWeight"),
                    id: bool_id,
                    implicit_id: unit_id,
                }],
            },
            ExtrinsicSignatureInfo {
                address_id: u8_id,
                signature_id: u8_id,
            },
            types.into(),
        )
    }

    #[test]
    fn params_are_default() {
        let params = DefaultExtrinsicParamsBuilder::<PolkadotConfig>::new().build();
        assert_default(params)
    }

    #[test]
    fn unknown_extension_without_custom_value_still_errors() {
        let params = DefaultExtrinsicParamsBuilder::<PolkadotConfig>::new().build();
        let extensions = DefaultTransactionExtensions::new(&client_state(), params).unwrap();
        let (call_info, extension_info, _, types) = encoding_info();
        let call_data = Composite::<()>::Unnamed(Vec::new());

        let error = frame_decode::extrinsics::encode_v4_signer_payload_with_info(
            &call_data,
            &extensions,
            &types,
            &call_info,
            &extension_info,
        )
        .unwrap_err();

        assert_matches!(
            error,
            ExtrinsicEncodeError::TransactionExtensions(TransactionExtensionsError::NotFound(name))
                if name == "CheckWeight"
        );
    }

    #[test]
    fn authorization_extension_check_is_forwarded() {
        let params = DefaultExtrinsicParamsBuilder::<PolkadotConfig>::new()
            .custom_extension("CheckWeight", true)
            .build();
        let extensions = DefaultTransactionExtensions::new(&client_state(), params).unwrap();
        assert!(
            frame_decode::extrinsics::TransactionExtensions::is_authorization_extension(
                &extensions,
                "VerifyMultiSignature"
            )
        );
        // Custom extensions are never authorization extensions.
        assert!(
            !frame_decode::extrinsics::TransactionExtensions::is_authorization_extension(
                &extensions,
                "CheckWeight"
            )
        );
    }

    #[test]
    fn signature_injection_is_forwarded() {
        #[allow(dead_code)]
        #[derive(scale_info::TypeInfo)]
        enum SignatureDetails {
            Signed {
                signature: MultiSignature,
                account: AccountId32,
            },
            Disabled,
        }

        let params = DefaultExtrinsicParamsBuilder::<PolkadotConfig>::new().build();
        let mut extensions = DefaultTransactionExtensions::new(&client_state(), params).unwrap();
        extensions.inject_signature(
            &AccountId32::from([1; 32]),
            &MultiSignature::Sr25519([2; 64]),
        );
        let (type_id, types) = type_info::<SignatureDetails>();
        let mut out = Vec::new();

        frame_decode::extrinsics::TransactionExtensions::encode_extension_value_to(
            &extensions,
            "VerifyMultiSignature",
            type_id,
            &types,
            &mut out,
        )
        .unwrap();

        let mut expected = vec![0, 1];
        expected.extend([2; 64]);
        expected.extend([1; 32]);
        assert_eq!(out, expected);
    }

    #[test]
    fn params_forward_injected_nonce_and_block() {
        let mut params = DefaultExtrinsicParamsBuilder::<PolkadotConfig>::new()
            .custom_extension("CheckWeight", true)
            .build();
        params.inject_account_nonce(7);
        params.inject_block(10, H256::repeat_byte(1));
        let extensions = DefaultTransactionExtensions::new(&client_state(), params).unwrap();
        let mut nonce = Vec::new();
        let mut mortality = Vec::new();
        let (_, types) = type_info::<bool>();

        frame_decode::extrinsics::TransactionExtensions::encode_extension_value_to(
            &extensions,
            "CheckNonce",
            0,
            &types,
            &mut nonce,
        )
        .unwrap();
        frame_decode::extrinsics::TransactionExtensions::encode_extension_value_to(
            &extensions,
            "CheckMortality",
            0,
            &types,
            &mut mortality,
        )
        .unwrap();

        assert_eq!(nonce, [28]);
        assert_ne!(mortality, [0]);
    }

    #[test]
    fn custom_extension_cannot_override_known_extension() {
        let params = DefaultExtrinsicParamsBuilder::<PolkadotConfig>::new()
            .custom_extension("CheckNonce", 1u128)
            .build();

        let error = DefaultTransactionExtensions::new(&client_state(), params)
            .err()
            .unwrap();

        assert!(error.to_string().contains("conflicts with a known"));
    }

    #[test]
    fn custom_extension_name_cannot_be_repeated() {
        let params = DefaultExtrinsicParamsBuilder::<PolkadotConfig>::new()
            .custom_extension("CheckWeight", true)
            .custom_extension("CheckWeight", false)
            .build();

        let error = DefaultTransactionExtensions::new(&client_state(), params)
            .err()
            .unwrap();

        assert!(error.to_string().contains("provided more than once"));
    }

    #[test]
    fn contains_known_and_custom_extensions() {
        let params = DefaultExtrinsicParamsBuilder::<PolkadotConfig>::new()
            .custom_extension("CheckWeight", true)
            .build();
        let extensions = DefaultTransactionExtensions::new(&client_state(), params).unwrap();

        assert!(
            frame_decode::extrinsics::TransactionExtensions::contains_extension(
                &extensions,
                "CheckNonce"
            )
        );
        assert!(
            frame_decode::extrinsics::TransactionExtensions::contains_extension(
                &extensions,
                "CheckWeight"
            )
        );
        assert!(
            !frame_decode::extrinsics::TransactionExtensions::contains_extension(
                &extensions,
                "Unknown"
            )
        );
    }

    #[test]
    fn nonempty_custom_implicit_has_a_specific_error() {
        let params = DefaultExtrinsicParamsBuilder::<PolkadotConfig>::new()
            .custom_extension("WeightReclaim", true)
            .build();
        let extensions = DefaultTransactionExtensions::new(&client_state(), params).unwrap();
        let (type_id, types) = type_info::<u32>();
        let mut out = Vec::new();

        let error = frame_decode::extrinsics::TransactionExtensions::encode_extension_implicit_to(
            &extensions,
            "WeightReclaim",
            type_id,
            &types,
            &mut out,
        )
        .unwrap_err();

        assert!(
            error
                .to_string()
                .contains("requires non-empty implicit data")
        );
    }

    #[test]
    fn custom_encoding_error_does_not_modify_output() {
        let params = DefaultExtrinsicParamsBuilder::<PolkadotConfig>::new()
            .custom_extension("WeightReclaim", Value::u128(1))
            .build();
        let extensions = DefaultTransactionExtensions::new(&client_state(), params).unwrap();
        let (type_id, types) = type_info::<bool>();
        let mut out = vec![42];

        let error = frame_decode::extrinsics::TransactionExtensions::encode_extension_value_to(
            &extensions,
            "WeightReclaim",
            type_id,
            &types,
            &mut out,
        )
        .unwrap_err();

        assert_eq!(out, [42]);
        assert_matches!(
            error,
            TransactionExtensionsError::Other { extension_name, .. }
                if extension_name == "WeightReclaim"
        );
    }

    #[test]
    fn custom_extension_is_used_in_v4_payload_and_extrinsic() {
        let params = DefaultExtrinsicParamsBuilder::<PolkadotConfig>::new()
            .custom_extension("CheckWeight", true)
            .build();
        let extensions = DefaultTransactionExtensions::new(&client_state(), params).unwrap();
        let (call_info, extension_info, signature_info, types) = encoding_info();
        let call_data = Composite::<()>::Unnamed(Vec::new());

        let payload = frame_decode::extrinsics::encode_v4_signer_payload_with_info(
            &call_data,
            &extensions,
            &types,
            &call_info,
            &extension_info,
        )
        .unwrap();
        let mut extrinsic = Vec::new();
        frame_decode::extrinsics::encode_v4_signed_with_info_to(
            &call_data,
            &extensions,
            &3u8,
            &4u8,
            &types,
            &call_info,
            &signature_info,
            &extension_info,
            &mut extrinsic,
        )
        .unwrap();
        let inner = Vec::<u8>::decode(&mut &*extrinsic).unwrap();

        assert_eq!(payload, [1, 2, 1]);
        assert_eq!(inner, [0x84, 3, 4, 1, 1, 2]);
    }

    #[test]
    fn custom_extension_absent_from_metadata_is_rejected() {
        let params = DefaultExtrinsicParamsBuilder::<PolkadotConfig>::new()
            .custom_extension("ChckWeight", true)
            .build();

        let error = DefaultTransactionExtensions::new(&client_state(), params)
            .err()
            .unwrap();

        assert!(
            error
                .to_string()
                .contains("is not present in the runtime metadata")
        );
    }

    #[test]
    fn tip_of_sets_correct_tip_on_charge_asset_tx_payment() {
        let params = DefaultExtrinsicParamsBuilder::<PolkadotConfig>::new()
            .tip(100) // Set the "basic" tip for ChargeTransactionPayment
            .tip_of(200, 42) // Set the asset-based tip for ChargeAssetTxPayment
            .build();

        // Type signatures here ensure we're getting the params we think we are:
        let known = params.known();
        let charge_asset_params: &transaction_extensions::ChargeAssetTxPaymentParams<_> = &known.6;
        let charge_transaction_params: &transaction_extensions::ChargeTransactionPaymentParams =
            &known.7;

        // Verify that the params are properly set:
        assert_eq!(
            *charge_asset_params,
            transaction_extensions::ChargeAssetTxPaymentParams::tip_of(200, 42)
        );
        assert_eq!(
            *charge_transaction_params,
            transaction_extensions::ChargeTransactionPaymentParams::tip(100)
        )
    }
}
