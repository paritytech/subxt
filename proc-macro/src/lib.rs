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

#[proc_macro_attribute]
#[proc_macro_error]
pub fn module(args: TokenStream, input: TokenStream) -> TokenStream {
    module::module(args.into(), input.into()).into()
}

decl_derive!(
    [Call] =>
    /// Derive macro that implements [substrate_subxt::Call](../substrate_subxt/trait.Call.html) for your struct
    /// and defines&implements the calls as an extension trait.
    ///
    /// Use the `Call` derive macro in tandem with the [#module](../substrate_subxt/attr.module.html) macro to extend
    ///  your struct to enable calls to substrate and to decode events.
    /// Implements [substrate_subxt::Call](../substrate_subxt/trait.Call.html) and adds an extension trait that
    ///  provides two methods named as your struct.
    ///
    /// Example:
    /// ```rust,ignore
    /// pub struct MyRuntime;
    ///
    /// impl System for MyRuntime { /* … */ }
    /// impl Balances for MyRuntime { /* … */ }
    ///
    /// #[module]
    /// pub trait MyTrait: System + Balances {}
    ///
    /// #[derive(Call)]
    /// pub struct FunStuffCall<T: MyTrait> {
    ///     /// Runtime marker.
    ///     pub _runtime: PhantomData<T>,
    ///     /// The argument passed to the call..
    ///     pub something: Vec<u8>,
    /// }
    /// ```
    ///
    /// When building a [Client](../substrate_subxt/struct.Client.html) parameterised to `MyRuntime`, you have access to
    ///  two new methods: `fun_stuff()` and `fun_stuff_and_watch()` by way of the derived `FunStuffExt` trait. The fields
    ///  of the input struct become arguments to the calls (ignoring the marker field).
    ///
    /// Under the hood the implementation calls [submit()](../substrate_subxt/struct.Client.html#method.submit) and
    /// [watch()](../substrate_subxt/struct.Client.html#method.watch) respectively.
    #[proc_macro_error] call
);
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
