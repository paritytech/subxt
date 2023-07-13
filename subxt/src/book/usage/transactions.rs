// Copyright 2019-2023 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

//! # Transactions
//!
//! A transaction is an extrinsic that's signed (ie it originates from a given address). The purpose
//! of extrinsics is to modify the node storage in a deterministic way, and so being able to submit
//! transactions to a node is one of the core features of Subxt.
//!
//! > Note: the documentation tends to use the terms _extrinsic_ and _transaction_ interchangeably;
//! > An extrinsic is some data that can be added to a block, and is either signed (a _transaction_)
//! > or unsigned (an _inherent_). Subxt can construct either, but overwhelmingly you'll need to
//! > sign the payload you'd like to submit.
//!
//! Submitting a transaction to a node consists of the following steps:
//!
//! 1. [Constructing a transaction payload to submit](#constructing-a-transaction-payload).
//! 2. [Signing it](#signing-it).
//! 3. [Submitting it (optionally with some additional parameters)](#submitting-it).
//!
//! We'll look at each of these steps in turn.
//!
//! ## Constructing a transaction payload
//!
//! We can use the statically generated interface to build transaction payloads:
//!
//! ```rust,no_run
//! #[subxt::subxt(runtime_metadata_path = "../artifacts/polkadot_metadata_small.scale")]
//! pub mod polkadot {}
//!
//! let remark = "Hello there".as_bytes().to_vec();
//! let tx_payload = polkadot::tx().system().remark(remark);
//! ```
//!
//! > If you're not sure what types to import and use to build a given payload, you can use the
//! > `subxt` CLI tool to generate the interface by using something like `subxt codegen | rustfmt >
//! > interface.rs`, to see what types and things are available (or even just to use directly
//! > instead of the [`#[subxt]`](crate::subxt) macro).
//!
//! Alternately, we can dynamically construct a transaction payload. This will not be type checked or
//! validated until it's submitted:
//!
//! ```rust,no_run
//! use subxt::dynamic::Value;
//!
//! let tx_payload = subxt::dynamic::tx("System", "remark", vec![
//!     Value::from_bytes("Hello there")
//! ]);
//! ```
//!
//! The [`crate::dynamic::Value`] type is a dynamic type much like a `serde_json::Value` but instead
//! represents any type of data that can be SCALE encoded or decoded. It can be serialized,
//! deserialized and parsed from/to strings.
//!
//! A valid transaction payload is just something that implements the [`crate::tx::TxPayload`] trait;
//! you can implement this trait on your own custom types if the built-in ones are not suitable for
//! your needs.
//!
//! ## Signing it
//!
//! You'll normally need to sign an extrinsic to prove that it originated from an account that you
//! control. To do this, you will typically first create a [`crate::tx::Signer`] instance, which tells
//! Subxt who the extrinsic is from, and takes care of signing the relevant details to prove this.
//!
//! There are two main ways to create a compatible signer instance:
//! 1. The `subxt_signer` crate provides a WASM compatible implementation of [`crate::tx::Signer`]
//! for chains which require sr25519 signatures (requires the `subxt` feature to be enabled).
//! 2. Alternately, Subxt can use instances of Substrate's [`sp_core::Pair`] to sign things by wrapping
//! them in a [`crate::tx::PairSigner`] (requires the `substrate-compat` feature to be enabled).
//!
//! Going for 1 leads to fewer dependencies being imported and WASM compatibility out of the box via
//! the `web` feature flag. Going for 2 is useful if you're already using the Substrate dependencies or
//! need additional signing algorithms that `subxt_signer` doesn't support, and don't care about WASM
//! compatibility.
//!
//! Let's see how to use each of these approaches:
//!
//! ```rust
//! use subxt::config::PolkadotConfig;
//! use std::str::FromStr;
//!
//! //// 1. Use a `subxt_signer` impl:
//! use subxt_signer::{ SecretUri, sr25519 };
//!
//! // Get hold of a `Signer` for a test account:
//! let alice = sr25519::dev::alice();
//!
//! // Or generate a keypair, here from an SURI:
//! let uri = SecretUri::from_str("vessel ladder alter error federal sibling chat ability sun glass valve picture/0/1///Password")
//!     .expect("valid URI");
//! let keypair = sr25519::Keypair::from_uri(&uri)
//!     .expect("valid keypair");
//!
//! //// 2. Use the corresponding `sp_core::Pair` impl:
//! use subxt::tx::PairSigner;
//! use sp_core::Pair;
//!
//! // Get hold of a `Signer` for a test account:
//! let alice = sp_keyring::AccountKeyring::Alice.pair();
//! let alice = PairSigner::<PolkadotConfig,_>::new(alice);
//!
//! // Or generate a keypair, here from an SURI:
//! let keypair = sp_core::sr25519::Pair::from_string("vessel ladder alter error federal sibling chat ability sun glass valve picture/0/1///Password", None)
//!     .expect("valid URI");
//! let keypair = PairSigner::<PolkadotConfig,_>::new(keypair);
//! ```
//!
//! See the `subxt_signer::sr25519::Keypair` or the [`sp_core::Pair`] docs for more ways to construct
//! and work with key pairs.
//!
//! If this isn't suitable/available, you can either implement [`crate::tx::Signer`] yourself to use
//! custom signing logic, or you can use some external signing logic, like so:
//!
//! ```rust,no_run
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! use subxt::client::OnlineClient;
//! use subxt::config::PolkadotConfig;
//! use subxt::dynamic::Value;
//!
//! // Create client:
//! let client = OnlineClient::<PolkadotConfig>::new().await?;
//!
//! // Create a dummy tx payload to sign:
//! let payload = subxt::dynamic::tx("System", "remark", vec![
//!     Value::from_bytes("Hello there")
//! ]);
//!
//! // Construct the tx but don't sign it. You need to provide the nonce
//! // here, or can use `create_partial_signed` to fetch the correct nonce.
//! let partial_tx = client.tx().create_partial_signed_with_nonce(
//!     &payload,
//!     0,
//!     Default::default()
//! )?;
//!
//! // Fetch the payload that needs to be signed:
//! let signer_payload = partial_tx.signer_payload();
//!
//! // ... At this point, we can hand off the `signer_payload` to be signed externally.
//! // Ultimately we need to be given back a `signature` (or really, anything
//! // that can be SCALE encoded) and an `address`:
//! let signature;
//! let address;
//! # use subxt::tx::Signer;
//! # let signer = subxt_signer::sr25519::dev::alice();
//! # signature = signer.sign(&signer_payload).into();
//! # address = signer.public_key().to_address();
//!
//! // Now we can build an tx, which one can call `submit` or `submit_and_watch`
//! // on to submit to a node and optionally watch the status.
//! let tx = partial_tx.sign_with_address_and_signature(
//!     &address,
//!     &signature
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## Submitting it
//!
//! Once we have signed the transaction, we need to submit it.
//!
//! ### The high level API
//!
//! The highest level approach to doing this is to call
//! [`crate::tx::TxClient::sign_and_submit_then_watch_default`]. This hands back a
//! [`crate::tx::TxProgress`] struct which will monitor the transaction status. We can then call
//! [`crate::tx::TxProgress::wait_for_finalized_success()`] to wait for this transaction to make it
//! into a finalized block, check for an `ExtrinsicSuccess` event, and then hand back the events for
//! inspection. This looks like:
//!
//! ```rust,ignore
#![doc = include_str!("../../../examples/tx_basic.rs")]
//! ```
//!
//! ### Providing transaction parameters
//!
//! If you'd like to provide parameters (such as mortality) to the transaction, you can use
//! [`crate::tx::TxClient::sign_and_submit_then_watch`] instead:
//!
//! ```rust,ignore
#![doc = include_str!("../../../examples/tx_with_params.rs")]
//! ```
//!
//! This example doesn't wait for the transaction to be included in a block; it just submits it and
//! hopes for the best!
//!
//! ### Custom handling of transaction status updates
//!
//! If you'd like more control or visibility over exactly which status updates are being emitted for
//! the transaction, you can monitor them as they are emitted and react however you choose:
//!
//! ```rust,ignore
#![doc = include_str!("../../../examples/tx_status_stream.rs")]
//! ```
//!
//! Take a look at the API docs for [`crate::tx::TxProgress`], [`crate::tx::TxStatus`] and
//! [`crate::tx::TxInBlock`] for more options.
//!
