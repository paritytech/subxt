// Copyright 2019-2020 Parity Technologies (UK) Ltd.
// This file is part of substrate-subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with substrate-subxt.  If not, see <http://www.gnu.org/licenses/>.

extern crate proc_macro;

mod call;
mod event;
mod module;
mod store;
mod test;
mod utils;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use synstructure::{
    decl_derive,
    Structure,
};

/// Register type sizes for [EventsDecoder](struct.EventsDecoder.html) and sets the `MODULE`.
///
/// The `module` macro registers the type sizes of the associated types of a trait so that [EventsDecoder](struct.EventsDecoder.html)
/// can decode events of that type when received from Substrate. It also sets the `MODULE` constant
/// to the name of the trait (must match the name of the Substrate pallet) that enables the [Call](), [Event]() and [Store]() macros to work.
///
/// If you do not want an associated type to be registered, likely because you never expect it as part of a response payload to be decoded, use `#[module(ignore)]` on the type.
///
/// Example:
///
/// ```ignore
/// #[module]
/// pub trait Herd: Husbandry {
/// 	type Hooves: HoofCounter;
/// 	type Wool: WoollyAnimal;
/// 	#[module(ignore)]
/// 	type Digestion: EnergyProducer + std::fmt::Debug;
/// }
/// ```
///
/// The above will produce the following code:
///
/// ```ignore
/// pub trait Herd: Husbandry {
///     type Hooves: HoofCounter;
///     type Wool: WoollyAnimal;
///     #[module(ignore)]
///     type Digestion: EnergyProducer + std::fmt::Debug;
/// }
///
/// const MODULE: &str = "Herd";
///
/// // `EventsDecoder` extension trait.
/// pub trait HerdEventsDecoder {
///     // Registers this modules types.
///     fn with_herd(&mut self);
/// }
///
/// impl<T: Herd> HerdEventsDecoder for
///     substrate_subxt::EventsDecoder<T>
/// {
///     fn with_herd(&mut self) {
///         self.with_husbandry();
///         self.register_type_size::<T::Hooves>("Hooves");
///         self.register_type_size::<T::Wool>("Wool");
///     }
/// }
/// ```
///
/// The following type sizes are registered by default: `bool, u8, u32, AccountId, AccountIndex,
/// AuthorityId, AuthorityIndex, AuthorityWeight, BlockNumber, DispatchInfo, Hash, Kind,
/// MemberCount, PhantomData, PropIndex, ProposalIndex, ReferendumIndex, SessionIndex, VoteThreshold`
#[proc_macro_attribute]
#[proc_macro_error]
pub fn module(args: TokenStream, input: TokenStream) -> TokenStream {
    module::module(args.into(), input.into()).into()
}

decl_derive!([Call] => #[proc_macro_error] call);
fn call(s: Structure) -> TokenStream {
    call::call(s).into()
}

decl_derive!([Event] => #[proc_macro_error] event);
fn event(s: Structure) -> TokenStream {
    event::event(s).into()
}

decl_derive!([Store, attributes(store)] => #[proc_macro_error] store);
fn store(s: Structure) -> TokenStream {
    store::store(s).into()
}

#[proc_macro]
#[proc_macro_error]
pub fn subxt_test(input: TokenStream) -> TokenStream {
    test::test(input.into()).into()
}
