use anyhow::anyhow;
use futures::FutureExt;

use subxt::{OnlineClient, PolkadotConfig};

use subxt::ext::codec::{Decode, Encode};
use subxt::tx::SubmittableExtrinsic;
use subxt::tx::TxPayload;
use subxt::utils::{AccountId32, MultiSignature};
use subxt::config::DefaultExtrinsicParamsBuilder;

use crate::services::{extension_signature_for_extrinsic, get_accounts, polkadot, Account};
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct SigningExamplesComponent {
    message: String,
    remark_call_bytes: Vec<u8>,
    online_client: Option<OnlineClient<PolkadotConfig>>,
    stage: SigningStage,
}

impl SigningExamplesComponent {
    /// # Panics
    /// panics if self.online_client is None.
    fn set_message(&mut self, message: String) {
        let remark_call = polkadot::tx().system().remark(message.as_bytes().to_vec());
        let online_client = self.online_client.as_ref().unwrap();
        let remark_call_bytes = remark_call
            .encode_call_data(&online_client.metadata())
            .unwrap();
        self.remark_call_bytes = remark_call_bytes;
        self.message = message;
    }
}

pub enum SigningStage {
    Error(String),
    CreatingOnlineClient,
    EnterMessage,
    RequestingAccounts,
    SelectAccount(Vec<Account>),
    Signing(Account),
    SigningSuccess {
        signer_account: Account,
        signature: MultiSignature,
        signed_extrinsic_hex: String,
        submitting_stage: SubmittingStage,
    },
}

pub enum SubmittingStage {
    Initial {
        signed_extrinsic: SubmittableExtrinsic<PolkadotConfig, OnlineClient<PolkadotConfig>>,
    },
    Submitting,
    Success {
        remark_event: polkadot::system::events::ExtrinsicSuccess,
    },
    Error(anyhow::Error),
}

pub enum Message {
    Error(anyhow::Error),
    OnlineClientCreated(OnlineClient<PolkadotConfig>),
    ChangeMessage(String),
    RequestAccounts,
    ReceivedAccounts(Vec<Account>),
    /// usize represents account index in Vec<Account>
    SignWithAccount(usize),
    ReceivedSignature(
        MultiSignature,
        SubmittableExtrinsic<PolkadotConfig, OnlineClient<PolkadotConfig>>,
    ),
    SubmitSigned,
    ExtrinsicFinalized {
        remark_event: polkadot::system::events::ExtrinsicSuccess,
    },
    ExtrinsicFailed(anyhow::Error),
}

impl Component for SigningExamplesComponent {
    type Message = Message;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(OnlineClient::<PolkadotConfig>::new().map(|res| {
            match res {
                Ok(online_client) => Message::OnlineClientCreated(online_client),
                Err(err) => Message::Error(anyhow!("Online Client could not be created. Make sure you have a local node running:\n{err}")),
            }
        }));
        SigningExamplesComponent {
            message: "".to_string(),
            stage: SigningStage::CreatingOnlineClient,
            online_client: None,
            remark_call_bytes: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::OnlineClientCreated(online_client) => {
                self.online_client = Some(online_client);
                self.stage = SigningStage::EnterMessage;
                self.set_message("Hello".into());
            }
            Message::ChangeMessage(message) => {
                self.set_message(message);
            }
            Message::RequestAccounts => {
                self.stage = SigningStage::RequestingAccounts;
                ctx.link().send_future(get_accounts().map(
                    |accounts_or_err| match accounts_or_err {
                        Ok(accounts) => Message::ReceivedAccounts(accounts),
                        Err(err) => Message::Error(err),
                    },
                ));
            }
            Message::ReceivedAccounts(accounts) => {
                self.stage = SigningStage::SelectAccount(accounts);
            }
            Message::Error(err) => self.stage = SigningStage::Error(err.to_string()),
            Message::SignWithAccount(i) => {
                if let SigningStage::SelectAccount(accounts) = &self.stage {
                    let account = accounts.get(i).unwrap();
                    let account_address = account.address.clone();
                    let account_source = account.source.clone();
                    let account_id: AccountId32 = account_address.parse().unwrap();

                    self.stage = SigningStage::Signing(account.clone());

                    let remark_call = polkadot::tx()
                        .system()
                        .remark(self.message.as_bytes().to_vec());

                    let api = self.online_client.as_ref().unwrap().clone();

                    ctx.link()
                        .send_future(
                            async move {
                                let Ok(account_nonce) = api.tx().account_nonce(&account_id).await else {
                                    return Message::Error(anyhow!("Fetching account nonce failed"));
                                };

                                let Ok(call_data) = api.tx().call_data(&remark_call) else {
                                    return Message::Error(anyhow!("could not encode call data"));
                                };

                                let Ok(signature) = extension_signature_for_extrinsic(&call_data, &api, account_nonce, account_source, account_address).await else {
                                    return Message::Error(anyhow!("Signing via extension failed"));
                                };

                                let Ok(multi_signature) = MultiSignature::decode(&mut &signature[..]) else {
                                    return Message::Error(anyhow!("MultiSignature Decoding"));
                                };

                                let params = DefaultExtrinsicParamsBuilder::new().nonce(account_nonce).build();
                                let Ok(partial_signed) = api.tx().create_partial_signed_offline(&remark_call, params) else {
                                    return Message::Error(anyhow!("PartialExtrinsic creation failed"));
                                };

                                // Apply the signature
                                let signed_extrinsic = partial_signed.sign_with_address_and_signature(&account_id.into(), &multi_signature);

                                // check the TX validity (to debug in the js console if the extrinsic would work)
                                let dry_res = signed_extrinsic.validate().await;
                                web_sys::console::log_1(&format!("Validation Result: {:?}", dry_res).into());

                                // return the signature and signed extrinsic
                                Message::ReceivedSignature(multi_signature, signed_extrinsic)
                            }
                        );
                }
            }
            Message::ReceivedSignature(signature, signed_extrinsic) => {
                if let SigningStage::Signing(account) = &self.stage {
                    let signed_extrinsic_hex =
                        format!("0x{}", hex::encode(signed_extrinsic.encoded()));
                    self.stage = SigningStage::SigningSuccess {
                        signer_account: account.clone(),
                        signature,
                        signed_extrinsic_hex,
                        submitting_stage: SubmittingStage::Initial { signed_extrinsic },
                    }
                }
            }
            Message::SubmitSigned => {
                if let SigningStage::SigningSuccess {
                    submitting_stage: submitting_stage @ SubmittingStage::Initial { .. },
                    ..
                } = &mut self.stage
                {
                    let SubmittingStage::Initial { signed_extrinsic } = std::mem::replace(submitting_stage, SubmittingStage::Submitting) else {
                        panic!("unreachable")
                    };

                    ctx.link().send_future(async move {
                        match submit_wait_finalized_and_get_extrinsic_success_event(
                            signed_extrinsic,
                        )
                        .await
                        {
                            Ok(remark_event) => Message::ExtrinsicFinalized { remark_event },
                            Err(err) => Message::ExtrinsicFailed(err),
                        }
                    });
                }
            }
            Message::ExtrinsicFinalized { remark_event } => {
                if let SigningStage::SigningSuccess {
                    submitting_stage, ..
                } = &mut self.stage
                {
                    *submitting_stage = SubmittingStage::Success { remark_event }
                }
            }
            Message::ExtrinsicFailed(err) => {
                if let SigningStage::SigningSuccess {
                    submitting_stage, ..
                } = &mut self.stage
                {
                    *submitting_stage = SubmittingStage::Error(err)
                }
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let message_as_hex_html = || {
            html!(
                <div class="mb">
                    <b>{"Hex representation of \"remark\" call in \"System\" pallet:"}</b> <br/>
                    {format!("0x{}", hex::encode(&self.remark_call_bytes))}
                </div>
            )
        };

        let message_html: Html = match &self.stage {
            SigningStage::Error(_)
            | SigningStage::EnterMessage
            | SigningStage::CreatingOnlineClient => html!(<></>),
            _ => {
                let _remark_call = polkadot::tx()
                    .system()
                    .remark(self.message.as_bytes().to_vec());
                html!(
                    <div>
                        <div class="mb">
                            <b>{"Message: "}</b> <br/>
                            {&self.message}
                        </div>
                        {message_as_hex_html()}
                    </div>
                )
            }
        };

        let signer_account_html: Html = match &self.stage {
            SigningStage::Signing(signer_account)
            | SigningStage::SigningSuccess { signer_account, .. } => {
                html!(
                    <div class="mb">
                            <b>{"Account used for signing: "}</b> <br/>
                            {"Extension: "}{&signer_account.source} <br/>
                            {"Name: "}{&signer_account.name} <br/>
                            {"Address: "}{&signer_account.address} <br/>
                    </div>
                )
            }
            _ => html!(<></>),
        };

        let stage_html: Html = match &self.stage {
            SigningStage::Error(error_message) => {
                html!(<div class="error"> {"Error: "} {error_message} </div>)
            }
            SigningStage::CreatingOnlineClient => {
                html!(
                    <div>
                        <b>{"Creating Online Client..."}</b>
                    </div>
                )
            }
            SigningStage::EnterMessage => {
                let get_accounts_click = ctx.link().callback(|_| Message::RequestAccounts);
                let on_input = ctx.link().callback(move |event: InputEvent| {
                    let input_element = event.target_dyn_into::<HtmlInputElement>().unwrap();
                    let value = input_element.value();
                    Message::ChangeMessage(value)
                });

                html!(
                    <>
                        <div class="mb"><b>{"Enter a message for the \"remark\" call in the \"System\" pallet:"}</b></div>
                        <input oninput={on_input} class="mb" value={AttrValue::from(self.message.clone())}/>
                        {message_as_hex_html()}
                        <button onclick={get_accounts_click}> {"=> Select an Account for Signing"} </button>
                    </>
                )
            }
            SigningStage::RequestingAccounts => {
                html!(<div>{"Querying extensions for accounts..."}</div>)
            }
            SigningStage::SelectAccount(accounts) => {
                if accounts.is_empty() {
                    html!(<div>{"No Web3 extension accounts found. Install Talisman or the Polkadot.js extension and add an account."}</div>)
                } else {
                    html!(
                        <>
                            <div class="mb"><b>{"Select an account you want to use for signing:"}</b></div>
                            { for accounts.iter().enumerate().map(|(i, account)| {
                                let sign_with_account = ctx.link().callback(move |_| Message::SignWithAccount(i));
                                html! {
                                    <button onclick={sign_with_account}>
                                        {&account.source} {" | "} {&account.name}<br/>
                                        <small>{&account.address}</small>
                                    </button>
                                }
                            }) }
                        </>
                    )
                }
            }
            SigningStage::Signing(_) => {
                html!(<div>{"Singing message with browser extension..."}</div>)
            }
            SigningStage::SigningSuccess {
                signature,
                signed_extrinsic_hex,
                submitting_stage,
                ..
            } => {
                let submitting_stage_html = match submitting_stage {
                    SubmittingStage::Initial { .. } => {
                        let submit_extrinsic_click =
                            ctx.link().callback(move |_| Message::SubmitSigned);
                        html!(<button onclick={submit_extrinsic_click}> {"=> Submit the signed extrinsic"} </button>)
                    }
                    SubmittingStage::Submitting => {
                        html!(<div class="loading"><b>{"Submitting Extrinsic... (please wait a few seconds)"}</b></div>)
                    }
                    SubmittingStage::Success { remark_event } => {
                        html!(<div style="overflow-wrap: break-word;"> <b>{"Successfully submitted Extrinsic. Event:"}</b> <br/> {format!("{:?}", remark_event)} </div>)
                    }
                    SubmittingStage::Error(err) => {
                        html!(<div class="error"> {"Error: "} {err.to_string()} </div>)
                    }
                };

                html!(
                    <>
                        <div style="overflow-wrap: break-word;" class="mb">
                            <b>{"Received signature: "}</b><br/>
                            {hex::encode(signature.encode())}
                        </div>
                        <div style="overflow-wrap: break-word;" class="mb">
                            <b>{"Hex representation of signed extrinsic: "}</b> <br/>
                            {signed_extrinsic_hex}
                        </div>
                        {submitting_stage_html}
                    </>
                )
            }
        };

        html! {
            <div>
                <a href="/"> <button>{"<= Back"}</button></a>
                <h1>{"Subxt Signing Example"}</h1>
                {message_html}
                {signer_account_html}
                {stage_html}
            </div>
        }
    }
}

async fn submit_wait_finalized_and_get_extrinsic_success_event(
    extrinsic: SubmittableExtrinsic<PolkadotConfig, OnlineClient<PolkadotConfig>>,
) -> Result<polkadot::system::events::ExtrinsicSuccess, anyhow::Error> {
    let events = extrinsic
        .submit_and_watch()
        .await?
        .wait_for_finalized_success()
        .await?;

    let events_str = format!("{:?}", &events);
    web_sys::console::log_1(&events_str.into());
    for event in events.find::<polkadot::system::events::ExtrinsicSuccess>() {
        web_sys::console::log_1(&format!("{:?}", event).into());
    }

    let success = events.find_first::<polkadot::system::events::ExtrinsicSuccess>()?;
    success.ok_or(anyhow!("ExtrinsicSuccess not found in events"))
}
