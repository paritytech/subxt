use codec::{Decode, Encode};
use subxt::utils::AccountId32;

#[derive(Encode, Decode, subxt::ext::scale_encode::EncodeAsType, subxt::ext::scale_decode::DecodeAsType, Debug)]
#[encode_as_type(crate_path = "subxt::ext::scale_encode")]
#[decode_as_type(crate_path = "subxt::ext::scale_decode")]
pub struct CustomAddress(u16);

#[derive(Encode, Decode, subxt::ext::scale_encode::EncodeAsType, subxt::ext::scale_decode::DecodeAsType, Debug)]
#[encode_as_type(crate_path = "subxt::ext::scale_encode")]
#[decode_as_type(crate_path = "subxt::ext::scale_decode")]
pub struct Generic<T>(T);

#[derive(Encode, Decode, subxt::ext::scale_encode::EncodeAsType, subxt::ext::scale_decode::DecodeAsType, Debug)]
#[encode_as_type(crate_path = "subxt::ext::scale_encode")]
#[decode_as_type(crate_path = "subxt::ext::scale_decode")]
pub struct Second<T, U>(T, U);

#[derive(Encode, Decode, Debug)]
pub struct DoesntImplEncodeDecodeAsType(u16);

#[subxt::subxt(
    runtime_metadata_path = "../../../../artifacts/polkadot_metadata_small.scale",
    substitute_type(
        path = "sp_runtime::multiaddress::MultiAddress<A, B>",
        // Discarding both params:
        with = "crate::CustomAddress"
    )
)]
pub mod node_runtime {}

#[subxt::subxt(
    runtime_metadata_path = "../../../../artifacts/polkadot_metadata_small.scale",
    substitute_type(
        path = "sp_runtime::multiaddress::MultiAddress<A, B>",
        // Discarding second param:
        with = "crate::Generic<A>"
    )
)]
pub mod node_runtime2 {}

#[subxt::subxt(
    runtime_metadata_path = "../../../../artifacts/polkadot_metadata_small.scale",
    substitute_type(
        path = "sp_runtime::multiaddress::MultiAddress<A, B>",
        // Discarding first param:
        with = "crate::Generic<B>"
    )
)]
pub mod node_runtime3 {}

#[subxt::subxt(
    runtime_metadata_path = "../../../../artifacts/polkadot_metadata_small.scale",
    substitute_type(
        path = "sp_runtime::multiaddress::MultiAddress<A, B>",
        // Swapping params:
        with = "crate::Second<B, A>"
    )
)]
pub mod node_runtime4 {}

#[subxt::subxt(
    runtime_metadata_path = "../../../../artifacts/polkadot_metadata_small.scale",
    substitute_type(
        path = "sp_runtime::multiaddress::MultiAddress",
        // Ignore input params and just use concrete types on output:
        with = "crate::Second<bool, ::std::vec::Vec<u8>>"
    )
)]
pub mod node_runtime5 {}

#[subxt::subxt(
    runtime_metadata_path = "../../../../artifacts/polkadot_metadata_small.scale",
    substitute_type(
        path = "sp_runtime::multiaddress::MultiAddress<A, B>",
        // We can put a static type in, too:
        with = "crate::Second<B, u16>"
    )
)]
pub mod node_runtime6 {}

#[subxt::subxt(
    runtime_metadata_path = "../../../../artifacts/polkadot_metadata_small.scale",
    substitute_type(
        path = "sp_runtime::multiaddress::MultiAddress<A, B>",
        // Check that things can be wrapped in our Static type:
        with = "::subxt::utils::Static<crate::DoesntImplEncodeDecodeAsType>"
    )
)]
pub mod node_runtime7 {}

#[subxt::subxt(
    runtime_metadata_path = "../../../../artifacts/polkadot_metadata_small.scale",
    substitute_type(
        path = "sp_runtime::multiaddress::MultiAddress<A, B>",
        // Recursive type param substitution should work too (swapping out nested A and B):
        with = "::subxt::utils::Static<crate::Second<A, B>>"
    )
)]
pub mod node_runtime8 {}

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
        .transfer(Second((), AccountId32::from([0x01;32])), 123);

    let _ = node_runtime5::tx()
        .balances()
        .transfer(Second(true, vec![1u8, 2u8]), 123);

    let _ = node_runtime6::tx()
        .balances()
        .transfer(Second((), 1234u16), 123);

    let _ = node_runtime7::tx()
        .balances()
        .transfer(subxt::utils::Static(DoesntImplEncodeDecodeAsType(1337)), 123);

    let _ = node_runtime8::tx()
        .balances()
        .transfer(subxt::utils::Static(Second(AccountId32::from([0x01;32]), ())), 123);
}
