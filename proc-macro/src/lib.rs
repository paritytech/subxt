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
