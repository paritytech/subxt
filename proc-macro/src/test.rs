use crate::utils;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

mod kw {
    use syn::custom_keyword;

    custom_keyword!(name);
    custom_keyword!(runtime);
    custom_keyword!(account);
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
        Ok(Self {
            brace: syn::braced!(content in input),
            items: content.parse_terminated(I::parse)?,
        })
    }
}

type ItemTest = Items<TestItem>;

#[derive(Debug)]
enum TestItem {
    Name(Item<kw::name, syn::Ident>),
    Runtime(Item<kw::runtime, syn::Type>),
    Account(Item<kw::account, syn::Ident>),
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
    Assert(Item<kw::assert, syn::Expr>),
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
    runtime: syn::Type,
    account: syn::Ident,
    steps: Vec<Step>,
}

impl From<ItemTest> for Test {
    fn from(test: ItemTest) -> Self {
        let mut name = None;
        let mut runtime = None;
        let mut account = None;
        let mut steps = vec![];
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
                TestItem::Step(item) => {
                    steps.push(item.value.into());
                }
            }
        }
        Self {
            name: name.expect("No name specified"),
            runtime: runtime.expect("No runtime specified"),
            account: account.unwrap_or_else(|| format_ident!("Alice")),
            steps,
        }
    }
}

impl From<Test> for TokenStream {
    fn from(test: Test) -> Self {
        let env_logger = utils::use_crate("env_logger");
        let futures = utils::use_crate("futures");
        let sp_keyring = utils::use_crate("sp-keyring");
        let subxt = utils::use_crate("substrate-subxt");
        let Test { name, runtime, account, steps } = test;
        let step = steps.into_iter().map(TokenStream::from);
        quote! {
            #[async_std::test]
            #[ignore]
            async fn #name() {
                use #futures::future::FutureExt;
                #env_logger::init();

                let client = #subxt::ClientBuilder::<#runtime>::new().build().await.unwrap();
                let xt = client.xt(#sp_keyring::AccountKeyring::#account.pair(), None).await.unwrap();
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

                #({
                    #step
                })*
            }
        }
    }
}

struct Step {
    state_name: Vec<syn::Ident>,
    state: Vec<syn::Expr>,
    state_param: Vec<syn::Ident>,
    call: syn::Expr,
    event_name: Vec<syn::Path>,
    event: Vec<syn::Expr>,
    assert: syn::Expr,
}

impl From<ItemStep> for Step {
    fn from(step: ItemStep) -> Self {
        let mut state_name = vec![];
        let mut state = vec![];
        let mut call = None;
        let mut event_name = vec![];
        let mut event = vec![];
        let mut assert = None;

        for step_item in step.items {
            match step_item {
                StepItem::State(item) => {
                    for item in item.value.items {
                        state_name.push(item.key);
                        state.push(item.value);
                    }
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

        let state_param = (b'A'..b'Z')
            .map(|c| format_ident!("{}", (c as char).to_string()))
            .take(state_name.len())
            .collect::<Vec<_>>();

        Self {
            state_name,
            state,
            state_param,
            call: call.expect("Step requires a call."),
            event_name,
            event,
            assert: assert.expect("Step requires assert."),
        }
    }
}

impl From<Step> for TokenStream {
    fn from(step: Step) -> Self {
        let futures = utils::use_crate("futures");
        let Step { state_name, state, state_param, call, event_name, event, assert } = step;
        quote! {
            struct State<#(#state_param),*> {
                #(#state_name: #state_param,)*
            }

            let pre = {
                let (#(#state_name,)*) = #futures::join!(
                    #(
                        client
                            .fetch(#state, None)
                            .map(|r| r.unwrap().unwrap())
                    ),*
                );
                State { #(#state_name),* }
            };

            #[allow(unused)]
            let result = xt
                .watch()
                .submit(#call)
                .await
                .unwrap();

            #(
                assert_eq!(
                    result.find_event::<#event_name<_>>().unwrap(),
                    Some(#event)
                );
            )*

            let post = {
                let (#(#state_name,)*) = #futures::join!(
                    #(
                        client
                            .fetch(#state, None)
                            .map(|r| r.unwrap().unwrap())
                    ),*
                );
                State { #(#state_name),* }
            };

            #assert
        }
    }
}

fn struct_name(expr: &syn::Expr) -> syn::Path {
    if let syn::Expr::Struct(syn::ExprStruct { path, .. }) = expr {
        return path.clone();
    } else {
        panic!("not a struct");
    }
}

pub fn test(input: TokenStream) -> TokenStream {
    let item_test: ItemTest = syn::parse2(input).unwrap();
    Test::from(item_test).into()
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
                use futures::future::FutureExt;

                env_logger::init();
                let client = substrate_subxt::ClientBuilder::<KusamaRuntime>::new().build().await.unwrap();
                let xt = client.xt(sp_keyring::AccountKeyring::Alice.pair(), None).await.unwrap();
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
                        let (alice, bob,) = futures::join!(
                            client
                                .fetch(AccountStore { account_id: &alice }, None)
                                .map(|r| r.unwrap().unwrap()),
                            client
                                .fetch(AccountStore { account_id: &bob }, None)
                                .map(|r| r.unwrap().unwrap())
                        );
                        State { alice, bob }
                    };

                    #[allow(unused)]
                    let result = xt
                        .watch()
                        .submit(TransferCall {
                            to: &bob,
                            amount: 10_000,
                        })
                        .await
                        .unwrap();

                    assert_eq!(
                        result.find_event::<TransferEvent<_>>().unwrap(),
                        Some(TransferEvent {
                            from: alice.clone(),
                            to: bob.clone(),
                            amount: 10_000,
                        })
                    );

                    let post = {
                        let (alice, bob,) = futures::join!(
                            client
                                .fetch(AccountStore { account_id: &alice }, None)
                                .map(|r| r.unwrap().unwrap()),
                            client
                                .fetch(AccountStore { account_id: &bob }, None)
                                .map(|r| r.unwrap().unwrap())
                        );
                        State { alice, bob }
                    };

                    {
                        assert_eq!(pre.alice.free, post.alice.free - 10_000);
                        assert_eq!(pre.bob.free, post.bob.free + 10_000);
                    }
                }
            }
        };
        let result = test(input);
        utils::assert_proc_macro(result, expected);
    }
}
