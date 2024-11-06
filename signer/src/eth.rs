// Copyright 2019-2024 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! An ethereum keypair implementation.

use crate::ecdsa;
use alloc::format;
use core::str::FromStr;
use keccak_hash::keccak;
use secp256k1::Message;

use thiserror::Error as DeriveError;

const SECRET_KEY_LENGTH: usize = 32;

/// Bytes representing a private key.
pub type SecretKeyBytes = [u8; SECRET_KEY_LENGTH];

/// The public key for an [`Keypair`] key pair. This is the uncompressed variant of [`ecdsa::PublicKey`].
pub struct PublicKey(pub [u8; 65]);

impl AsRef<[u8]> for PublicKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

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
    /// use subxt_signer::{ bip39::Mnemonic, eth::{ Keypair, DerivationPath } };
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

        Keypair::from_secret_key(private.to_bytes())
    }

    /// Turn a 16, 32 or 64 byte seed into a keypair.
    ///
    /// # Warning
    ///
    /// This will only be secure if the seed is secure!
    pub fn from_seed(seed: &[u8]) -> Result<Self, Error> {
        let private = bip32::XPrv::new(seed).map_err(|_| Error::InvalidSeed)?;
        Keypair::from_secret_key(private.to_bytes())
    }

    /// Turn a 32 byte secret key into a keypair.
    ///
    /// # Warning
    ///
    /// This will only be secure if the secret key is secure!
    pub fn from_secret_key(secret_key: SecretKeyBytes) -> Result<Self, Error> {
        ecdsa::Keypair::from_secret_key(secret_key)
            .map(Self)
            .map_err(|_| Error::InvalidSeed)
    }

    /// Obtain the [`ecdsa::SecretKeyBytes`] of this keypair.
    pub fn secret_key(&self) -> SecretKeyBytes {
        self.0.secret_key()
    }

    /// Obtain the [`eth::PublicKey`] of this keypair.
    pub fn public_key(&self) -> PublicKey {
        let uncompressed = self.0 .0.public_key().serialize_uncompressed();
        PublicKey(uncompressed)
    }

    /// Signs an arbitrary message payload.
    pub fn sign(&self, signer_payload: &[u8]) -> Signature {
        self.sign_prehashed(&keccak(signer_payload).0)
    }

    /// Signs a pre-hashed message.
    pub fn sign_prehashed(&self, message_hash: &[u8; 32]) -> Signature {
        Signature(self.0.sign_prehashed(message_hash).0)
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

/// Verify that some signature for a message was created by the owner of the [`PublicKey`].
///
/// ```rust
/// use subxt_signer::{ bip39::Mnemonic, eth };
///
/// let keypair = eth::dev::alith();
/// let message = b"Hello!";
///
/// let signature = keypair.sign(message);
/// let public_key = keypair.public_key();
/// assert!(eth::verify(&signature, message, &public_key));
/// ```
pub fn verify<M: AsRef<[u8]>>(sig: &Signature, message: M, pubkey: &PublicKey) -> bool {
    let message_hash = keccak(message.as_ref());
    let wrapped =
        Message::from_digest_slice(message_hash.as_bytes()).expect("Message is 32 bytes; qed");
    let Ok(signature) = secp256k1::ecdsa::Signature::from_compact(&sig.as_ref()[..64]) else {
        return false;
    };
    let Ok(pk) = secp256k1::PublicKey::from_slice(&pubkey.0) else {
        return false;
    };

    secp256k1::Secp256k1::verification_only()
        .verify_ecdsa(&wrapped, &signature, &pk)
        .is_ok()
}

/// An error handed back if creating a keypair fails.
#[derive(Debug, PartialEq, DeriveError)]
pub enum Error {
    /// Invalid seed.
    #[error("Invalid seed (was it the wrong length?)")]
    InvalidSeed,
    /// Invalid derivation path.
    #[error("Could not derive from path; some values in the path may have been >= 2^31?")]
    DeriveFromPath,
}

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
    use super::*;
    use subxt_core::config::Config;
    use subxt_core::tx::signer::Signer as SignerT;
    use subxt_core::utils::AccountId20;
    use subxt_core::utils::MultiAddress;

    impl<T: Config> SignerT<T> for Keypair
    where
        T::AccountId: From<PublicKey>,
        T::Address: From<PublicKey>,
        T::Signature: From<Signature>,
    {
        fn account_id(&self) -> T::AccountId {
            self.public_key().into()
        }

        fn address(&self) -> T::Address {
            self.public_key().into()
        }

        fn sign(&self, signer_payload: &[u8]) -> T::Signature {
            self.sign(signer_payload).into()
        }
    }

    impl PublicKey {
        /// Obtains the public address of the account by taking the last 20 bytes
        /// of the Keccak-256 hash of the public key.
        pub fn to_account_id(&self) -> AccountId20 {
            let hash = keccak(&self.0[1..]).0;
            let hash20 = hash[12..].try_into().expect("should be 20 bytes");
            AccountId20(hash20)
        }
        /// A shortcut to obtain a [`MultiAddress`] from a [`PublicKey`].
        /// We often want this type, and using this method avoids any
        /// ambiguous type resolution issues.
        pub fn to_address<T>(self) -> MultiAddress<AccountId20, T> {
            MultiAddress::Address20(self.to_account_id().0)
        }
    }

    impl From<PublicKey> for AccountId20 {
        fn from(value: PublicKey) -> Self {
            value.to_account_id()
        }
    }

    impl<T> From<PublicKey> for MultiAddress<AccountId20, T> {
        fn from(value: PublicKey) -> Self {
            let address: AccountId20 = value.into();
            MultiAddress::Address20(address.0)
        }
    }
}

#[cfg(test)]
#[cfg(feature = "subxt")]
mod test {
    use bip39::Mnemonic;
    use proptest::prelude::*;
    use secp256k1::Secp256k1;
    use subxt_core::utils::AccountId20;

    use subxt_core::{config::*, tx::signer::Signer as SignerT, utils::H256};

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
                Keypair(ecdsa::Keypair::from_secret_key(private.to_bytes()).expect("valid ecdsa keypair"))
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

            assert_eq!(SubxtSigner::account_id(&keypair), keypair.public_key().to_account_id());
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
            let account_id_derived_from_pk: AccountId20 = keypair.public_key().to_account_id();
            assert_eq!(account_id_derived_from_pk, account_id);
            assert_eq!(keypair.public_key().to_account_id(), account_id);

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
            let act_account_id = keypair.public_key().to_account_id().checksum();
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

    // This is a part of the test set linked in BIP39 and copied from https://github.com/trezor/python-mnemonic/blob/f5a975ab10c035596d65d854d21164266ffed284/vectors.json.
    // The passphrase is always TREZOR. We check that keys generated with the mnemonic (and no derivation path) line up with the seeds given.
    #[test]
    fn check_basic_bip39_compliance() {
        let mnemonics_and_seeds = [
            (
                "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
                "c55257c360c07c72029aebc1b53c05ed0362ada38ead3e3e9efa3708e53495531f09a6987599d18264c1e1c92f2cf141630c7a3c4ab7c81b2f001698e7463b04",
            ),
            (
                "legal winner thank year wave sausage worth useful legal winner thank yellow",
                "2e8905819b8723fe2c1d161860e5ee1830318dbf49a83bd451cfb8440c28bd6fa457fe1296106559a3c80937a1c1069be3a3a5bd381ee6260e8d9739fce1f607",
            ),
            (
                "letter advice cage absurd amount doctor acoustic avoid letter advice cage above",
                "d71de856f81a8acc65e6fc851a38d4d7ec216fd0796d0a6827a3ad6ed5511a30fa280f12eb2e47ed2ac03b5c462a0358d18d69fe4f985ec81778c1b370b652a8",
            ),
            (
                "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong",
                "ac27495480225222079d7be181583751e86f571027b0497b5b5d11218e0a8a13332572917f0f8e5a589620c6f15b11c61dee327651a14c34e18231052e48c069",
            ),
            (
                "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon agent",
                "035895f2f481b1b0f01fcf8c289c794660b289981a78f8106447707fdd9666ca06da5a9a565181599b79f53b844d8a71dd9f439c52a3d7b3e8a79c906ac845fa",
            ),
            (
                "legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth useful legal will",
                "f2b94508732bcbacbcc020faefecfc89feafa6649a5491b8c952cede496c214a0c7b3c392d168748f2d4a612bada0753b52a1c7ac53c1e93abd5c6320b9e95dd",
            ),
            (
                "letter advice cage absurd amount doctor acoustic avoid letter advice cage absurd amount doctor acoustic avoid letter always",
                "107d7c02a5aa6f38c58083ff74f04c607c2d2c0ecc55501dadd72d025b751bc27fe913ffb796f841c49b1d33b610cf0e91d3aa239027f5e99fe4ce9e5088cd65",
            ),
            (
                "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo when",
                "0cd6e5d827bb62eb8fc1e262254223817fd068a74b5b449cc2f667c3f1f985a76379b43348d952e2265b4cd129090758b3e3c2c49103b5051aac2eaeb890a528",
            ),
            (
                "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art",
                "bda85446c68413707090a52022edd26a1c9462295029f2e60cd7c4f2bbd3097170af7a4d73245cafa9c3cca8d561a7c3de6f5d4a10be8ed2a5e608d68f92fcc8",
            ),
            (
                "legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth title",
                "bc09fca1804f7e69da93c2f2028eb238c227f2e9dda30cd63699232578480a4021b146ad717fbb7e451ce9eb835f43620bf5c514db0f8add49f5d121449d3e87",
            ),
            (
                "letter advice cage absurd amount doctor acoustic avoid letter advice cage absurd amount doctor acoustic avoid letter advice cage absurd amount doctor acoustic bless",
                "c0c519bd0e91a2ed54357d9d1ebef6f5af218a153624cf4f2da911a0ed8f7a09e2ef61af0aca007096df430022f7a2b6fb91661a9589097069720d015e4e982f",
            ),
            (
                "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo vote",
                "dd48c104698c30cfe2b6142103248622fb7bb0ff692eebb00089b32d22484e1613912f0a5b694407be899ffd31ed3992c456cdf60f5d4564b8ba3f05a69890ad",
            ),
            (
                "ozone drill grab fiber curtain grace pudding thank cruise elder eight picnic",
                "274ddc525802f7c828d8ef7ddbcdc5304e87ac3535913611fbbfa986d0c9e5476c91689f9c8a54fd55bd38606aa6a8595ad213d4c9c9f9aca3fb217069a41028",
            ),
            (
                "gravity machine north sort system female filter attitude volume fold club stay feature office ecology stable narrow fog",
                "628c3827a8823298ee685db84f55caa34b5cc195a778e52d45f59bcf75aba68e4d7590e101dc414bc1bbd5737666fbbef35d1f1903953b66624f910feef245ac",
            ),
            (
                "hamster diagram private dutch cause delay private meat slide toddler razor book happy fancy gospel tennis maple dilemma loan word shrug inflict delay length",
                "64c87cde7e12ecf6704ab95bb1408bef047c22db4cc7491c4271d170a1b213d20b385bc1588d9c7b38f1b39d415665b8a9030c9ec653d75e65f847d8fc1fc440",
            ),
            (
                "scheme spot photo card baby mountain device kick cradle pact join borrow",
                "ea725895aaae8d4c1cf682c1bfd2d358d52ed9f0f0591131b559e2724bb234fca05aa9c02c57407e04ee9dc3b454aa63fbff483a8b11de949624b9f1831a9612",
            ),
            (
                "horn tenant knee talent sponsor spell gate clip pulse soap slush warm silver nephew swap uncle crack brave",
                "fd579828af3da1d32544ce4db5c73d53fc8acc4ddb1e3b251a31179cdb71e853c56d2fcb11aed39898ce6c34b10b5382772db8796e52837b54468aeb312cfc3d",
            ),
            (
                "panda eyebrow bullet gorilla call smoke muffin taste mesh discover soft ostrich alcohol speed nation flash devote level hobby quick inner drive ghost inside",
                "72be8e052fc4919d2adf28d5306b5474b0069df35b02303de8c1729c9538dbb6fc2d731d5f832193cd9fb6aeecbc469594a70e3dd50811b5067f3b88b28c3e8d",
            ),
            (
                "cat swing flag economy stadium alone churn speed unique patch report train",
                "deb5f45449e615feff5640f2e49f933ff51895de3b4381832b3139941c57b59205a42480c52175b6efcffaa58a2503887c1e8b363a707256bdd2b587b46541f5",
            ),
            (
                "light rule cinnamon wrap drastic word pride squirrel upgrade then income fatal apart sustain crack supply proud access",
                "4cbdff1ca2db800fd61cae72a57475fdc6bab03e441fd63f96dabd1f183ef5b782925f00105f318309a7e9c3ea6967c7801e46c8a58082674c860a37b93eda02",
            ),
            (
                "all hour make first leader extend hole alien behind guard gospel lava path output census museum junior mass reopen famous sing advance salt reform",
                "26e975ec644423f4a4c4f4215ef09b4bd7ef924e85d1d17c4cf3f136c2863cf6df0a475045652c57eb5fb41513ca2a2d67722b77e954b4b3fc11f7590449191d",
            ),
            (
                "vessel ladder alter error federal sibling chat ability sun glass valve picture",
                "2aaa9242daafcee6aa9d7269f17d4efe271e1b9a529178d7dc139cd18747090bf9d60295d0ce74309a78852a9caadf0af48aae1c6253839624076224374bc63f",
            ),
            (
                "scissors invite lock maple supreme raw rapid void congress muscle digital elegant little brisk hair mango congress clump",
                "7b4a10be9d98e6cba265566db7f136718e1398c71cb581e1b2f464cac1ceedf4f3e274dc270003c670ad8d02c4558b2f8e39edea2775c9e232c7cb798b069e88",
            ),
            (
                "void come effort suffer camp survey warrior heavy shoot primary clutch crush open amazing screen patrol group space point ten exist slush involve unfold",
                "01f5bced59dec48e362f2c45b5de68b9fd6c92c6634f44d6d40aab69056506f0e35524a518034ddc1192e1dacd32c1ed3eaa3c3b131c88ed8e7e54c49a5d0998",
            )
        ];

        for (idx, (m, s)) in mnemonics_and_seeds.into_iter().enumerate() {
            let m = Mnemonic::parse(m).expect("mnemonic should be valid");
            let pair1 = Keypair::from_phrase(&m, Some("TREZOR"), DerivationPath::empty()).unwrap();
            let s = hex::decode(s).expect("seed hex should be valid");
            let pair2 = Keypair::from_seed(&s).unwrap();

            assert_eq!(pair1, pair2, "pair1 and pair2 at index {idx} don't match");
        }
    }

    /// Test the same accounts from moonbeam so we know for sure that this implementation is working
    /// https://github.com/moonbeam-foundation/moonbeam/blob/e70ee0d427dfee8987d5a5671a66416ee6ec38aa/primitives/account/src/lib.rs#L217
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
            let kp = Keypair::from_secret_key(KEY_1).expect("valid keypair");
            assert_eq!(
                kp.public_key().to_account_id().checksum(),
                "0x976f8456E4e2034179B284A23C0e0c8f6d3da50c"
            );
        }

        #[test]
        fn test_account_derivation_2() {
            let kp = Keypair::from_secret_key(KEY_2).expect("valid keypair");
            assert_eq!(
                kp.public_key().to_account_id().checksum(),
                "0x420e9F260B40aF7E49440ceAd3069f8e82A5230f"
            );
        }

        #[test]
        fn test_account_derivation_3() {
            let kp = Keypair::from_secret_key(KEY_3).expect("valid keypair");
            assert_eq!(
                kp.public_key().to_account_id().checksum(),
                "0x9cce34F7aB185c7ABA1b7C8140d620B4BDA941d6"
            );
        }
    }
}
