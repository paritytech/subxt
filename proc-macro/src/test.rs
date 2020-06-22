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

use crate::utils;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::{
    format_ident,
    quote,
};
use syn::{
    parse::{
        Parse,
        ParseStream,
    },
    punctuated::Punctuated,
};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(name);
    custom_keyword!(runtime);
    custom_keyword!(account);
    custom_keyword!(prelude);
    custom_keyword!(step);
    custom_keyword!(state);
    custom_keyword!(call);
    custom_keyword!(event);
    custom_keyword!(assert);
}

#[derive(Debug)]
struct Item<K, V> {
    key: K,
    colon: syn::token::Colon,
    value: V,
}

impl<K: Parse, V: Parse> Parse for Item<K, V> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            key: input.parse()?,
            colon: input.parse()?,
            value: input.parse()?,
        })
    }
}

#[derive(Debug)]
struct Items<I> {
    brace: syn::token::Brace,
    items: Punctuated<I, syn::token::Comma>,
}

impl<I: Parse> Parse for Items<I> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let brace = syn::braced!(content in input);
        let items = content.parse_terminated(I::parse)?;
        Ok(Self { brace, items })
    }
}

type ItemTest = Items<TestItem>;

#[derive(Debug)]
enum TestItem {
    Name(Item<kw::name, syn::Ident>),
    Runtime(Item<kw::runtime, Box<syn::Type>>),
    Account(Item<kw::account, syn::Ident>),
    State(Item<kw::state, ItemState>),
    Prelude(Item<kw::prelude, syn::Block>),
    Step(Item<kw::step, ItemStep>),
}

impl Parse for TestItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(kw::name) {
            Ok(TestItem::Name(input.parse()?))
        } else if input.peek(kw::runtime) {
            Ok(TestItem::Runtime(input.parse()?))
        } else if input.peek(kw::account) {
            Ok(TestItem::Account(input.parse()?))
        } else if input.peek(kw::state) {
            Ok(TestItem::State(input.parse()?))
        } else if input.peek(kw::prelude) {
            Ok(TestItem::Prelude(input.parse()?))
        } else {
            Ok(TestItem::Step(input.parse()?))
        }
    }
}

type ItemStep = Items<StepItem>;

#[derive(Debug)]
enum StepItem {
    State(Item<kw::state, ItemState>),
    Call(Item<kw::call, syn::Expr>),
    Event(Item<kw::event, syn::Expr>),
    Assert(Item<kw::assert, syn::Block>),
}

impl Parse for StepItem {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(kw::state) {
            Ok(StepItem::State(input.parse()?))
        } else if input.peek(kw::call) {
            Ok(StepItem::Call(input.parse()?))
        } else if input.peek(kw::event) {
            Ok(StepItem::Event(input.parse()?))
        } else {
            Ok(StepItem::Assert(input.parse()?))
        }
    }
}

type ItemState = Items<StateItem>;
type StateItem = Item<syn::Ident, syn::Expr>;

struct Test {
    name: syn::Ident,
    runtime: Box<syn::Type>,
    account: syn::Ident,
    state: Option<State>,
    prelude: Option<syn::Block>,
    steps: Vec<Step>,
}

impl From<ItemTest> for Test {
    fn from(test: ItemTest) -> Self {
        let mut name = None;
        let mut runtime = None;
        let mut account = None;
        let mut state = None;
        let mut prelude = None;
        let mut steps = vec![];

        let span = test.brace.span;
        for test_item in test.items {
            match test_item {
                TestItem::Name(item) => {
                    name = Some(item.value);
                }
                TestItem::Runtime(item) => {
                    runtime = Some(item.value);
                }
                TestItem::Account(item) => {
                    account = Some(item.value);
                }
                TestItem::State(item) => {
                    state = Some(item.value.into());
                }
                TestItem::Prelude(item) => {
                    prelude = Some(item.value);
                }
                TestItem::Step(item) => {
                    steps.push(item.value.into());
                }
            }
        }
        let subxt = utils::use_crate("substrate-subxt");
        let runtime = runtime
            .unwrap_or_else(|| syn::parse2(quote!(#subxt::DefaultNodeRuntime)).unwrap());
        Self {
            name: name.unwrap_or_else(|| abort!(span, "No name specified")),
            account: account.unwrap_or_else(|| format_ident!("Alice")),
            runtime,
            state,
            prelude,
            steps,
        }
    }
}

impl Test {
    fn into_tokens(self) -> TokenStream {
        let subxt = utils::use_crate("substrate-subxt");
        let sp_keyring = utils::use_crate("sp-keyring");
        let env_logger = utils::opt_crate("env_logger")
            .map(|env_logger| quote!(#env_logger::try_init().ok();));
        let Test {
            name,
            runtime,
            account,
            state,
            prelude,
            steps,
        } = self;
        let prelude = prelude.map(|block| block.stmts).unwrap_or_default();
        let step = steps
            .into_iter()
            .map(|step| step.into_tokens(state.as_ref()));
        quote! {
            #[async_std::test]
            #[ignore]
            async fn #name() {
                #env_logger
                let client = #subxt::ClientBuilder::<#runtime>::new()
                    .build().await.unwrap();
                let signer = #subxt::PairSigner::new(#sp_keyring::AccountKeyring::#account.pair());

                #[allow(unused)]
                let alice = #sp_keyring::AccountKeyring::Alice.to_account_id();
                #[allow(unused)]
                let bob = #sp_keyring::AccountKeyring::Bob.to_account_id();
                #[allow(unused)]
                let charlie = #sp_keyring::AccountKeyring::Charlie.to_account_id();
                #[allow(unused)]
                let dave = #sp_keyring::AccountKeyring::Dave.to_account_id();
                #[allow(unused)]
                let eve = #sp_keyring::AccountKeyring::Eve.to_account_id();
                #[allow(unused)]
                let ferdie = #sp_keyring::AccountKeyring::Ferdie.to_account_id();

                #(#prelude)*

                #({
                    #step
                })*
            }
        }
    }
}

struct Step {
    state: Option<State>,
    call: syn::Expr,
    event_name: Vec<syn::Path>,
    event: Vec<syn::Expr>,
    assert: Option<syn::Block>,
}

impl From<ItemStep> for Step {
    fn from(step: ItemStep) -> Self {
        let mut state = None;
        let mut call = None;
        let mut event_name = vec![];
        let mut event = vec![];
        let mut assert = None;

        let span = step.brace.span;
        for step_item in step.items {
            match step_item {
                StepItem::State(item) => {
                    state = Some(item.value.into());
                }
                StepItem::Call(item) => {
                    call = Some(item.value);
                }
                StepItem::Event(item) => {
                    event_name.push(struct_name(&item.value));
                    event.push(item.value);
                }
                StepItem::Assert(item) => {
                    assert = Some(item.value);
                }
            }
        }

        Self {
            state,
            call: call.unwrap_or_else(|| abort!(span, "Step requires a call.")),
            event_name,
            event,
            assert,
        }
    }
}

impl Step {
    fn into_tokens(self, test_state: Option<&State>) -> TokenStream {
        let Step {
            state,
            call,
            event_name,
            event,
            assert,
        } = self;
        let (pre, post) = state
            .as_ref()
            .or(test_state)
            .map(|state| {
                let State {
                    state_name,
                    state,
                    state_param,
                } = state;
                let state_struct = quote! {
                    struct State<#(#state_param),*> {
                        #(#state_name: #state_param,)*
                    }
                };
                let build_struct = quote! {
                    #(
                        let #state_name = client.fetch_or_default(#state, None).await.unwrap();
                    )*
                    State { #(#state_name),* }
                };
                let pre = quote! {
                    #state_struct
                    let pre = {
                        #build_struct
                    };
                };
                let post = quote! {
                    let post = {
                        #build_struct
                    };
                };
                (pre, post)
            })
            .unwrap_or_default();
        let expect_event = event_name.iter().map(|event| {
            format!(
                "failed to find event {}",
                utils::path_to_ident(event).to_string()
            )
        });
        let assert = assert.map(|block| block.stmts).unwrap_or_default();
        quote! {
            #pre

            #[allow(unused)]
            let result = client
                .watch(#call, &signer)
                .await
                .unwrap();

            #(
                let event = result.find_event::<#event_name<_>>().unwrap().expect(#expect_event);
                assert_eq!(event, #event);
            )*

            #post

            #(#assert)*
        }
    }
}

struct State {
    state_name: Vec<syn::Ident>,
    state: Vec<syn::Expr>,
    state_param: Vec<syn::Ident>,
}

impl From<ItemState> for State {
    fn from(item_state: ItemState) -> Self {
        let mut state_name = vec![];
        let mut state = vec![];
        for item in item_state.items {
            state_name.push(item.key);
            state.push(item.value);
        }
        let state_param = (b'A'..b'Z')
            .map(|c| format_ident!("{}", (c as char).to_string()))
            .take(state_name.len())
            .collect::<Vec<_>>();
        Self {
            state_name,
            state,
            state_param,
        }
    }
}

fn struct_name(expr: &syn::Expr) -> syn::Path {
    if let syn::Expr::Struct(syn::ExprStruct { path, .. }) = expr {
        path.clone()
    } else {
        abort!(expr, "Expected a struct");
    }
}

pub fn test(input: TokenStream) -> TokenStream {
    let item_test: ItemTest =
        syn::parse2(input).map_err(|err| abort!("{}", err)).unwrap();
    Test::from(item_test).into_tokens()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transfer_test_case() {
        let input = quote! {{
            name: test_transfer_balance,
            runtime: KusamaRuntime,
            account: Alice,
            step: {
                state: {
                    alice: AccountStore { account_id: &alice },
                    bob: AccountStore { account_id: &bob },
                },
                call: TransferCall {
                    to: &bob,
                    amount: 10_000,
                },
                event: TransferEvent {
                    from: alice.clone(),
                    to: bob.clone(),
                    amount: 10_000,
                },
                assert: {
                    assert_eq!(pre.alice.free, post.alice.free - 10_000);
                    assert_eq!(pre.bob.free, post.bob.free + 10_000);
                },
            },
        }};
        let expected = quote! {
            #[async_std::test]
            #[ignore]
            async fn test_transfer_balance() {
                env_logger::try_init().ok();
                let client = substrate_subxt::ClientBuilder::<KusamaRuntime>::new().build().await.unwrap();
                let signer = substrate_subxt::PairSigner::new(sp_keyring::AccountKeyring::Alice.pair());
                #[allow(unused)]
                let alice = sp_keyring::AccountKeyring::Alice.to_account_id();
                #[allow(unused)]
                let bob = sp_keyring::AccountKeyring::Bob.to_account_id();
                #[allow(unused)]
                let charlie = sp_keyring::AccountKeyring::Charlie.to_account_id();
                #[allow(unused)]
                let dave = sp_keyring::AccountKeyring::Dave.to_account_id();
                #[allow(unused)]
                let eve = sp_keyring::AccountKeyring::Eve.to_account_id();
                #[allow(unused)]
                let ferdie = sp_keyring::AccountKeyring::Ferdie.to_account_id();

                {
                    struct State<A, B> {
                        alice: A,
                        bob: B,
                    }

                    let pre = {
                        let alice = client
                            .fetch_or_default(AccountStore { account_id: &alice }, None)
                            .await
                            .unwrap();
                        let bob = client
                            .fetch_or_default(AccountStore { account_id: &bob }, None)
                            .await
                            .unwrap();
                        State { alice, bob }
                    };

                    #[allow(unused)]
                    let result = client
                        .watch(TransferCall {
                            to: &bob,
                            amount: 10_000,
                        }, &signer)
                        .await
                        .unwrap();

                    let event = result.find_event::<TransferEvent<_>>()
                        .unwrap()
                        .expect("failed to find event TransferEvent");
                    assert_eq!(
                        event,
                        TransferEvent {
                            from: alice.clone(),
                            to: bob.clone(),
                            amount: 10_000,
                        }
                    );

                    let post = {
                        let alice = client
                            .fetch_or_default(AccountStore { account_id: &alice }, None)
                            .await
                            .unwrap();
                        let bob = client
                            .fetch_or_default(AccountStore { account_id: &bob }, None)
                            .await
                            .unwrap();
                        State { alice, bob }
                    };

                    assert_eq!(pre.alice.free, post.alice.free - 10_000);
                    assert_eq!(pre.bob.free, post.bob.free + 10_000);
                }
            }
        };
        let result = test(input);
        utils::assert_proc_macro(result, expected);
    }
}
