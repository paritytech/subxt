// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! An ethereum keypair implementation.

use crate::ecdsa;
use alloc::format;
use alloc::string::String;
use core::fmt::{Display, Formatter};
use core::str::FromStr;
use derive_more::Display;
use keccak_hash::keccak;
use secp256k1::Message;

const SEED_LENGTH: usize = 32;

/// Seed bytes used to generate a key pair.
pub type Seed = [u8; SEED_LENGTH];

/// An ethereum keypair implementation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keypair(ecdsa::Keypair);

impl From<ecdsa::Keypair> for Keypair {
    fn from(kp: ecdsa::Keypair) -> Self {
        Self(kp)
    }
}

impl Keypair {
    /// Create a keypair from a BIP-39 mnemonic phrase, optional password, account index, and
    /// derivation type.
    ///
    /// **Note:** if the `std` feature is not enabled, we won't attempt to normalize the provided password
    /// to NFKD first, and so this is your responsibility. This is not a concern if only ASCII
    /// characters are used in the password.
    ///
    /// # Example
    ///
    /// ```rust
    /// use subxt_signer::{ bip39::Mnemonic, eth::Keypair };
    ///
    /// let phrase = "bottom drive obey lake curtain smoke basket hold race lonely fit walk";
    /// let mnemonic = Mnemonic::parse(phrase).unwrap();
    /// let keypair = Keypair::from_phrase(&mnemonic, None, DerivationPath::eth(0,0)).unwrap();
    ///
    /// keypair.sign(b"Hello world!");
    /// ```
    pub fn from_phrase(
        mnemonic: &bip39::Mnemonic,
        password: Option<&str>,
        derivation_path: DerivationPath,
    ) -> Result<Self, Error> {
        // `to_seed` isn't available unless std is enabled in bip39.
        #[cfg(feature = "std")]
        let seed = mnemonic.to_seed(password.unwrap_or(""));
        #[cfg(not(feature = "std"))]
        let seed = mnemonic.to_seed_normalized(password.unwrap_or(""));

        // TODO: Currently, we use bip32 to derive private keys which under the hood uses
        // the Rust k256 crate. We _also_ use the secp256k1 crate (which is very similar).
        // It'd be great if we could 100% use just one of the two crypto libs. bip32 has
        // a feature flag to use secp256k1, but it's unfortunately a different version (older)
        // than ours.
        let private = bip32::XPrv::derive_from_path(seed, &derivation_path.inner)
            .map_err(|_| Error::DeriveFromPath)?;

        Keypair::from_seed(private.to_bytes())
    }

    /// Turn a 32 byte seed into a keypair.
    ///
    /// # Warning
    ///
    /// This will only be secure if the seed is secure!
    pub fn from_seed(seed: Seed) -> Result<Self, Error> {
        ecdsa::Keypair::from_seed(seed)
            .map(Self)
            .map_err(|_| Error::InvalidSeed)
    }

    /// Obtain the [`ecdsa::PublicKey`] of this keypair.
    pub fn public_key(&self) -> ecdsa::PublicKey {
        self.0.public_key()
    }

    /// Obtains the public address of the account by taking the last 20 bytes
    /// of the Keccak-256 hash of the public key.
    pub fn account_id(&self) -> AccountId20 {
        let uncompressed = self.0 .0.public_key().serialize_uncompressed();
        let hash = keccak(&uncompressed[1..]).0;
        let hash20 = hash[12..].try_into().expect("should be 20 bytes");
        AccountId20(hash20)
    }

    /// Signs an arbitrary message payload.
    pub fn sign(&self, signer_payload: &[u8]) -> Signature {
        let message_hash = keccak(signer_payload);
        let wrapped =
            Message::from_digest_slice(message_hash.as_bytes()).expect("Message is 32 bytes; qed");
        Signature(ecdsa::internal::sign(&self.0 .0.secret_key(), &wrapped))
    }
}

/// A derivation path. This can be parsed from a valid derivation path string like
/// `"m/44'/60'/0'/0/0"`, or we can construct one using the helpers [`DerivationPath::empty()`]
/// and [`DerivationPath::eth()`].
#[derive(Clone, Debug)]
pub struct DerivationPath {
    inner: bip32::DerivationPath,
}

impl DerivationPath {
    /// An empty derivation path (in other words, just use the master-key as is).
    pub fn empty() -> Self {
        let inner = bip32::DerivationPath::from_str("m").unwrap();
        DerivationPath { inner }
    }

    /// A BIP44 Ethereum compatible derivation using the path "m/44'/60'/account'/0/address_index".
    ///
    /// # Panics
    ///
    /// Panics if the `account` or `address_index` provided are >= 2^31.
    pub fn eth(account: u32, address_index: u32) -> Self {
        assert!(
            account < bip32::ChildNumber::HARDENED_FLAG,
            "account must be less than 2^31"
        );
        assert!(
            address_index < bip32::ChildNumber::HARDENED_FLAG,
            "address_index must be less than 2^31"
        );

        let derivation_string = format!("m/44'/60'/{account}'/0/{address_index}");
        let inner = bip32::DerivationPath::from_str(&derivation_string).unwrap();
        DerivationPath { inner }
    }
}

impl FromStr for DerivationPath {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = bip32::DerivationPath::from_str(s).map_err(|_| Error::DeriveFromPath)?;
        Ok(DerivationPath { inner })
    }
}

/// A signature generated by [`Keypair::sign()`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, codec::Encode)]
pub struct Signature(pub [u8; 65]);

impl AsRef<[u8; 65]> for Signature {
    fn as_ref(&self) -> &[u8; 65] {
        &self.0
    }
}

/// A 20-byte cryptographic identifier.
#[derive(Debug, Copy, Clone, PartialEq, Eq, codec::Encode)]
pub struct AccountId20(pub [u8; 20]);

impl AccountId20 {
    fn checksum(&self) -> String {
        let hex_address = hex::encode(self.0);
        let hash = keccak(hex_address.as_bytes());

        let mut checksum_address = String::with_capacity(42);
        checksum_address.push_str("0x");

        for (i, ch) in hex_address.chars().enumerate() {
            // Get the corresponding nibble from the hash
            let nibble = hash[i / 2] >> (if i % 2 == 0 { 4 } else { 0 }) & 0xf;

            if nibble >= 8 {
                checksum_address.push(ch.to_ascii_uppercase());
            } else {
                checksum_address.push(ch);
            }
        }

        checksum_address
    }
}

impl AsRef<[u8]> for AccountId20 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Display for AccountId20 {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.checksum())
    }
}

pub fn verify<M: AsRef<[u8]>>(sig: &Signature, message: M, pubkey: &ecdsa::PublicKey) -> bool {
    let message_hash = keccak(message.as_ref());
    let wrapped =
        Message::from_digest_slice(message_hash.as_bytes()).expect("Message is 32 bytes; qed");

    ecdsa::internal::verify(&sig.0, &wrapped, pubkey)
}

/// An error handed back if creating a keypair fails.
#[derive(Debug, PartialEq, Display)]
pub enum Error {
    /// Invalid seed.
    #[display(fmt = "Invalid seed (was it the wrong length?)")]
    InvalidSeed,
    /// Invalid derivation path.
    #[display(fmt = "Could not derive from path; some valeus in the path may have been >= 2^31?")]
    DeriveFromPath,
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// Dev accounts, helpful for testing but not to be used in production,
/// since the secret keys are known.
pub mod dev {
    use core::str::FromStr;

    use crate::DEV_PHRASE;

    use super::*;

    once_static_cloned! {
        pub fn alith() -> Keypair {
            Keypair::from_phrase(
                &bip39::Mnemonic::from_str(DEV_PHRASE).unwrap(), None, DerivationPath::eth(0, 0)).unwrap()
        }
        pub fn baltathar() -> Keypair {
            Keypair::from_phrase(
                &bip39::Mnemonic::from_str(DEV_PHRASE).unwrap(), None, DerivationPath::eth(0, 1)).unwrap()
        }
        pub fn charleth() -> Keypair {
            Keypair::from_phrase(
                &bip39::Mnemonic::from_str(DEV_PHRASE).unwrap(), None, DerivationPath::eth(0, 2)).unwrap()
        }
        pub fn dorothy() -> Keypair {
            Keypair::from_phrase(
                &bip39::Mnemonic::from_str(DEV_PHRASE).unwrap(), None, DerivationPath::eth(0, 3)).unwrap()
        }
        pub fn ethan() -> Keypair {
            Keypair::from_phrase(
                &bip39::Mnemonic::from_str(DEV_PHRASE).unwrap(), None, DerivationPath::eth(0, 4)).unwrap()
        }
        pub fn faith() -> Keypair {
            Keypair::from_phrase(
                &bip39::Mnemonic::from_str(DEV_PHRASE).unwrap(), None, DerivationPath::eth(0, 5)).unwrap()
        }
    }
}

#[cfg(feature = "subxt")]
mod subxt_compat {
    use subxt_core::config::Config;
    use subxt_core::tx::Signer as SignerT;

    use super::*;

    impl<T: Config> SignerT<T> for Keypair
    where
        T::AccountId: From<AccountId20>,
        T::Address: From<AccountId20>,
        T::Signature: From<Signature>,
    {
        fn account_id(&self) -> T::AccountId {
            self.account_id().into()
        }

        fn address(&self) -> T::Address {
            self.account_id().into()
        }

        fn sign(&self, signer_payload: &[u8]) -> T::Signature {
            self.sign(signer_payload).into()
        }
    }
}

#[cfg(test)]
mod test {
    use proptest::prelude::*;
    use secp256k1::Secp256k1;

    use subxt_core::{config::*, tx::Signer as SignerT, utils::H256};

    use super::*;

    enum StubEthRuntimeConfig {}

    impl Config for StubEthRuntimeConfig {
        type Hash = H256;
        type AccountId = AccountId20;
        type Address = AccountId20;
        type Signature = Signature;
        type Hasher = substrate::BlakeTwo256;
        type Header = substrate::SubstrateHeader<u32, substrate::BlakeTwo256>;
        type ExtrinsicParams = SubstrateExtrinsicParams<Self>;
        type AssetId = u32;
    }

    type SubxtSigner = dyn SignerT<StubEthRuntimeConfig>;

    prop_compose! {
        fn keypair()(seed in any::<[u8; 32]>()) -> Keypair {
            let secret = secp256k1::SecretKey::from_slice(&seed).expect("valid secret key");
            let inner = secp256k1::Keypair::from_secret_key(
                &Secp256k1::new(),
                &secret,
            );

            Keypair(ecdsa::Keypair(inner))
        }
    }

    proptest! {
        #[test]
        fn check_from_phrase(
            entropy in any::<[u8; 32]>(),
            password in any::<Option<String>>(),
            address in 1..(i32::MAX as u32),
            account_idx in 1..(i32::MAX as u32),
        ) {
            let mnemonic = bip39::Mnemonic::from_entropy(&entropy).expect("valid mnemonic");
            let derivation_path = format!("m/44'/60'/{address}'/0/{account_idx}").parse().expect("valid derivation path");
            let private = bip32::XPrv::derive_from_path(
                mnemonic.to_seed(password.clone().unwrap_or("".to_string())),
                &derivation_path,
            ).expect("valid private");

            // Creating our own keypairs should be equivalent to using bip32 crate to do it:
            assert_eq!(
                Keypair::from_phrase(&mnemonic, password.as_deref(), DerivationPath::eth(address, account_idx)).expect("valid keypair"),
                Keypair(ecdsa::Keypair::from_seed(private.to_bytes()).expect("valid ecdsa keypair"))
            );
        }

       #[test]
        fn check_from_phrase_bad_index(
            address in (i32::MAX as u32)..=u32::MAX,
            account_idx in (i32::MAX as u32)..=u32::MAX,
        ) {
            let derivation_path_err = format!("m/44'/60'/{address}'/0/{account_idx}").parse::<DerivationPath>().expect_err("bad path expected");

            // Creating invalid derivation paths (ie values too large) will result in an error.
            assert_eq!(
                derivation_path_err,
                Error::DeriveFromPath
            );
        }

        #[test]
        fn check_subxt_signer_implementation_matches(keypair in keypair(), msg in ".*") {
            let msg_as_bytes = msg.as_bytes();

            assert_eq!(SubxtSigner::account_id(&keypair), keypair.account_id());
            assert_eq!(SubxtSigner::sign(&keypair, msg_as_bytes), keypair.sign(msg_as_bytes));
        }

        #[test]
        fn check_account_id(keypair in keypair()) {
            // https://github.com/ethereumbook/ethereumbook/blob/develop/04keys-addresses.asciidoc#ethereum-addresses
            let account_id = {
                let uncompressed = keypair.0.0.public_key().serialize_uncompressed();
                let hash = keccak(&uncompressed[1..]).0;
                let hash20 = hash[12..].try_into().expect("should be 20 bytes");
                AccountId20(hash20)
            };

            assert_eq!(keypair.account_id(), account_id);

        }

        #[test]
        fn check_account_id_eq_address(keypair in keypair()) {
            assert_eq!(SubxtSigner::account_id(&keypair), SubxtSigner::address(&keypair));
        }

        #[test]
        fn check_signing_and_verifying_matches(keypair in keypair(), msg in ".*") {
            let sig = SubxtSigner::sign(&keypair, msg.as_bytes());

            assert!(verify(
                &sig,
                msg,
                &keypair.public_key())
            );
        }
    }

    /// Test that the dev accounts match those listed in the moonbeam README.
    /// https://github.com/moonbeam-foundation/moonbeam/blob/96cf8898874509d529b03c4da0e07b2787bacb18/README.md
    #[test]
    fn check_dev_accounts_match() {
        let cases = [
            (
                dev::alith(),
                "0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac",
                "0x5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133",
            ),
            (
                dev::baltathar(),
                "0x3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0",
                "0x8075991ce870b93a8870eca0c0f91913d12f47948ca0fd25b49c6fa7cdbeee8b",
            ),
            (
                dev::charleth(),
                "0x798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc",
                "0x0b6e18cafb6ed99687ec547bd28139cafdd2bffe70e6b688025de6b445aa5c5b",
            ),
            (
                dev::dorothy(),
                "0x773539d4Ac0e786233D90A233654ccEE26a613D9",
                "0x39539ab1876910bbf3a223d84a29e28f1cb4e2e456503e7e91ed39b2e7223d68",
            ),
            (
                dev::ethan(),
                "0xFf64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB",
                "0x7dce9bc8babb68fec1409be38c8e1a52650206a7ed90ff956ae8a6d15eeaaef4",
            ),
            (
                dev::faith(),
                "0xC0F0f4ab324C46e55D02D0033343B4Be8A55532d",
                "0xb9d2ea9a615f3165812e8d44de0d24da9bbd164b65c4f0573e1ce2c8dbd9c8df",
            ),
        ];

        for (case_idx, (keypair, exp_account_id, exp_priv_key)) in cases.into_iter().enumerate() {
            let act_account_id = keypair.account_id().to_string();
            let act_priv_key = format!("0x{}", &keypair.0 .0.display_secret());

            assert_eq!(
                exp_account_id, act_account_id,
                "account ID mismatch in {case_idx}"
            );
            assert_eq!(
                exp_priv_key, act_priv_key,
                "private key mismatch in {case_idx}"
            );
        }
    }

    /// Test the same accounts from moonbeam so we know for sure that this implementation is working
    /// https://github.com/moonbeam-foundation/moonbeam/blob/e70ee0d427dfee8987d5a5671a66416ee6ec38aa/primitives/account/src/lib.rs#L217
    #[cfg(test)]
    mod moonbeam_sanity_tests {
        use hex_literal::hex;

        use super::*;

        const KEY_1: [u8; 32] =
            hex!("502f97299c472b88754accd412b7c9a6062ef3186fba0c0388365e1edec24875");
        const KEY_2: [u8; 32] =
            hex!("0f02ba4d7f83e59eaa32eae9c3c4d99b68ce76decade21cdab7ecce8f4aef81a");
        const KEY_3: [u8; 32] =
            hex!("c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470");

        #[test]
        fn test_account_derivation_1() {
            let kp = Keypair::from_seed(KEY_1).expect("valid keypair");
            assert_eq!(
                kp.account_id().to_string(),
                "0x976f8456E4e2034179B284A23C0e0c8f6d3da50c"
            );
        }

        #[test]
        fn test_account_derivation_2() {
            let kp = Keypair::from_seed(KEY_2).expect("valid keypair");
            assert_eq!(
                kp.account_id().to_string(),
                "0x420e9F260B40aF7E49440ceAd3069f8e82A5230f"
            );
        }

        #[test]
        fn test_account_derivation_3() {
            let kp = Keypair::from_seed(KEY_3).expect("valid keypair");
            assert_eq!(
                kp.account_id().to_string(),
                "0x9cce34F7aB185c7ABA1b7C8140d620B4BDA941d6"
            );
        }
    }
}
