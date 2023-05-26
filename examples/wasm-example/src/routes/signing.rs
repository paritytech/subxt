
use futures::FutureExt;



use web_sys::HtmlInputElement;
use yew::prelude::*;
use crate::services::{Account, get_accounts, sign_hex_message};

pub struct SigningExamplesComponent {
    message: String,
    stage: SigningStage,
}

pub enum SigningStage {
    Error(String),
    EnterMessage,
    QueryAccounts,
    SelectAccount(Vec<Account>),
    SingMessage(Account),
    SigningSuccess {
        signer_account: Account,
        signature: String,
    },
}

pub enum Message {
    ChangeMessage(String),
    RequestAccounts,
    ReceivedAccounts(Vec<Account>),
    /// usize represents account index in Vec<Account>
    SignWithAccount(usize),
    ReceivedSignature(String),
    Error(anyhow::Error),
}

impl Component for SigningExamplesComponent {
    type Message = Message;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        SigningExamplesComponent {
            message: "Hello".to_string(),
            stage: SigningStage::EnterMessage,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::ReceivedAccounts(accounts) => {
                self.stage = SigningStage::SelectAccount(accounts);
            }
            Message::RequestAccounts => {
                ctx.link().send_future(get_accounts().map(
                    |accounts_or_err| match accounts_or_err {
                        Ok(accounts) => Message::ReceivedAccounts(accounts),
                        Err(err) => Message::Error(err),
                    },
                ));
            }
            Message::Error(err) => self.stage = SigningStage::Error(err.to_string()),
            Message::SignWithAccount(i) => {
                if let SigningStage::SelectAccount(accounts) = &self.stage {
                    let account = accounts.get(i).unwrap();
                    let account_json_string = serde_json::to_string(account).unwrap();
                    self.stage = SigningStage::SingMessage(account.clone());
                    let hex_message = format!("0x{}", hex::encode(self.message.clone()));
                    ctx.link()
                        .send_future(sign_hex_message(hex_message, account_json_string).map(
                            |signature_or_err| match signature_or_err {
                                Ok(signature) => Message::ReceivedSignature(signature),
                                Err(err) => Message::Error(err),
                            },
                        ));
                }
            }
            Message::ReceivedSignature(signature) => {
                if let SigningStage::SingMessage(account) = &self.stage {
                    self.stage = SigningStage::SigningSuccess {
                        signer_account: account.clone(),
                        signature,
                    }
                }
            }
            Message::ChangeMessage(message) => {
                web_sys::console::log_1(&message.clone().into());
                self.message = message
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let message_html: Html = match &self.stage {
            SigningStage::Error(_) | SigningStage::EnterMessage => html!(<></>),
            _ => {
                let hex_message = format!("0x{}", hex::encode(&self.message));
                html!(
                    <div>
                        <div class="mb">
                            <b>{"Message: "}</b> <br/>
                            {&self.message}
                        </div>
                        <div class="mb">
                            <b>{"Hex representation of message:"}</b> <br/>
                            {hex_message}
                        </div>
                    </div>
                )
            }
        };

        let signer_account_html: Html = match &self.stage {
            SigningStage::SingMessage(signer_account)
            | SigningStage::SigningSuccess {
                signer_account,
                signature: _,
            } => {
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
                html!(<div style={"color: red; background: black;"}> {"Error: "} {error_message} </div>)
            }
            SigningStage::EnterMessage => {
                let get_accounts_click = ctx.link().callback(|_| Message::RequestAccounts);
                let hex_message = format!("0x{}", hex::encode(&self.message));
                let on_input = ctx.link().callback(move |event: InputEvent| {
                    let input_element = event.target_dyn_into::<HtmlInputElement>().unwrap();
                    let value = input_element.value();
                    Message::ChangeMessage(value)
                });

                html!(
                    <>
                        <div class="mb">{"Enter a message to be signed:"}</div>
                        <input oninput={on_input} class="mb" value={AttrValue::from(self.message.clone())}/>
                        <div class="mb"><b>{"Hex representation of message:"}</b><br/>{hex_message}</div>
                        <button onclick={get_accounts_click}> {"=> Select an Account for Signing"} </button>
                    </>
                )
            }
            SigningStage::QueryAccounts => {
                html!(<div>{"Querying extensions for accounts..."}</div>)
            }
            SigningStage::SelectAccount(accounts) => {
                if accounts.is_empty() {
                    html!(<div>{"No Web3 extension accounts found. Install Talisman or the Polkadot.js extension and add an account."}</div>)
                } else {
                    html!(
                        <>
                            <div>{"Select an account you want to use for signing:"}</div>
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
            SigningStage::SingMessage(_) => {
                html!(<div>{"Singing message with browser extension..."}</div>)
            }
            SigningStage::SigningSuccess {
                signer_account: _,
                signature,
            } => {
                html!(
                    <div style="overflow-wrap: break-word;">
                        <b>{"Received signature: "}</b><br/>
                        {signature}
                    </div>
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
