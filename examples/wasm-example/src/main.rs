//! This is a small WASM app using the Yew UI framework showcasing how to use Subxt's features in a WASM environment.
//!
//! To run the app locally use Trunk, a WASM bundler:
//! ```
//! cargo install --locked trunk
//! ```
//! Run the app locally:
//! ```
//! trunk serve --open
//! ```
//! You need to have a local polkadot/substrate node with it's JSON-RPC HTTP server running at 127.0.0.1:9933 in order for the examples to be working.
//! Also make sure your browser supports WASM.
use futures::{self, FutureExt};

use yew::prelude::*;
mod services;

fn main() {
    yew::Renderer::<SubxtExamplesComponent>::new().render();
}

struct SubxtExamplesComponent {
    operation_title: Option<AttrValue>,
    lines: Vec<AttrValue>,
}

enum Message {
    Error(subxt::Error),
    Reload,
    Line(AttrValue),
    Lines(Vec<AttrValue>),
    ButtonClick(Button),
}

enum Button {
    SubscribeFinalized,
    FetchConstant,
    FetchEvents,
}

impl Component for SubxtExamplesComponent {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        SubxtExamplesComponent {
            lines: Vec::new(),
            operation_title: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Error(err) => {
                self.lines.push(err.to_string().into());
            }
            Message::Reload => {
                let window = web_sys::window().expect("Failed to access the window object");
                window
                    .location()
                    .reload()
                    .expect("Failed to reload the page");
            }
            Message::Line(line) => {
                self.lines.push(line);
            }
            Message::Lines(mut lines) => {
                self.lines.append(&mut lines);
            }
            Message::ButtonClick(button) => match button {
                Button::SubscribeFinalized => {
                    self.operation_title = Some("Subscribe to finalized blocks:".into());
                    let cb: Callback<AttrValue> = ctx.link().callback(Message::Line);
                    ctx.link()
                        .send_future(services::subscribe_to_finalized_blocks(cb).map(|result| {
                            let err = result.unwrap_err();
                            Message::Error(err)
                        }));
                }
                Button::FetchConstant => {
                    self.operation_title =
                        Some("Fetch the constant \"block_length\" of \"System\" pallet:".into());
                    ctx.link()
                        .send_future(services::fetch_constant_block_length().map(|result| {
                            match result {
                                Ok(value) => Message::Line(
                                    format!(
                                    "constant \"block_length\" of \"System\" pallet:\n    {value}"
                                )
                                    .into(),
                                ),
                                Err(err) => Message::Error(err),
                            }
                        }))
                }
                Button::FetchEvents => {
                    self.operation_title = Some("Fetch events:".into());
                    ctx.link()
                        .send_future(services::fetch_events_dynamically().map(
                            |result| match result {
                                Ok(value) => {
                                    Message::Lines(value.into_iter().map(AttrValue::from).collect())
                                }
                                Err(err) => Message::Error(err),
                            },
                        ))
                }
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let reload: Callback<MouseEvent> = ctx.link().callback(|_| Message::Reload);

        let subscribe_finalized = ctx
            .link()
            .callback(|_| Message::ButtonClick(Button::SubscribeFinalized));

        let fetch_constant = ctx
            .link()
            .callback(|_| Message::ButtonClick(Button::FetchConstant));

        let fetch_events = ctx
            .link()
            .callback(|_| Message::ButtonClick(Button::FetchEvents));

        html! {
            <div>
                if let Some(operation_title) = &self.operation_title{
                    <button onclick={reload}>{"🡄 Back"}</button>
                    <h1>{operation_title}</h1>
                    if self.lines.is_empty(){
                        <p>{"Loading..."}</p>
                    }
                    else{

                    }
                    { for self.lines.iter().map(|line| html! {<p> {line} </p>}) }
                }
                else{
                    <>
                        <h1>{"Subxt Examples"}</h1>
                        <button onclick={subscribe_finalized}>{"Example: Subscribe to Finalized blocks"}</button>
                        <button onclick={fetch_constant}>{"Example: Fetch constant value"}</button>
                        <button onclick={fetch_events}>{"Example: Fetch events"}</button>
                    </>
                }
            </div>
        }
    }
}
