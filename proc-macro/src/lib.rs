extern crate proc_macro;

mod call;
mod event;
mod module;
mod store;
mod test;
mod utils;

use proc_macro::TokenStream;
use synstructure::{
    decl_derive,
    Structure,
};

#[proc_macro_attribute]
pub fn module(args: TokenStream, input: TokenStream) -> TokenStream {
    module::module(args.into(), input.into()).into()
}

decl_derive!([Call] => call);
fn call(s: Structure) -> TokenStream {
    call::call(s).into()
}

decl_derive!([Event] => event);
fn event(s: Structure) -> TokenStream {
    event::event(s).into()
}

decl_derive!([Store, attributes(store)] => store);
fn store(s: Structure) -> TokenStream {
    store::store(s).into()
}

#[proc_macro]
pub fn subxt_test(input: TokenStream) -> TokenStream {
    test::test(input.into()).into()
}
