use core::marker::PhantomData;

use codec::{Decode, Encode};

use subxt::utils::AccountId32;

#[derive(Encode, Decode, Debug)]
pub struct CustomAddress(u16);
#[derive(Encode, Decode, Debug)]
pub struct Generic<T>(T);
#[derive(Encode, Decode, Debug)]
pub struct Second<T, U>(U, PhantomData<T>);

#[subxt::subxt(
    runtime_metadata_path = "../../../artifacts/polkadot_metadata.scale",
    substitute_type(
        type = "sp_runtime::multiaddress::MultiAddress<A, B>",
        with = "crate::CustomAddress"
    )
)]
pub mod node_runtime {}

#[subxt::subxt(
    runtime_metadata_path = "../../../artifacts/polkadot_metadata.scale",
    substitute_type(
        type = "sp_runtime::multiaddress::MultiAddress<A, B>",
        with = "crate::Generic<A>"
    )
)]
pub mod node_runtime2 {}

#[subxt::subxt(
    runtime_metadata_path = "../../../artifacts/polkadot_metadata.scale",
    substitute_type(
        type = "sp_runtime::multiaddress::MultiAddress<A, B>",
        with = "crate::Generic<B>"
    )
)]
pub mod node_runtime3 {}

#[subxt::subxt(
    runtime_metadata_path = "../../../artifacts/polkadot_metadata.scale",
    substitute_type(
        type = "sp_runtime::multiaddress::MultiAddress<A, B>",
        with = "crate::Second<B, A>"
    )
)]
pub mod node_runtime4 {}

fn main() {
    // We assume Polkadot's config of MultiAddress<AccountId32, ()> here
    let _ = node_runtime::tx()
        .balances()
        .transfer(CustomAddress(1337), 123);

    let _ = node_runtime2::tx()
        .balances()
        .transfer(Generic(AccountId32::from([0x01;32])), 123);

    let _ = node_runtime3::tx()
        .balances()
        .transfer(Generic(()), 123);

    let _ = node_runtime4::tx()
        .balances()
        .transfer(Second(AccountId32::from([0x01;32]), PhantomData), 123);
}
