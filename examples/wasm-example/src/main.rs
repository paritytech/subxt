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

use routes::signing::SigningExamplesComponent;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::fetching::FetchingExamplesComponent;
mod services;
mod routes;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/fetching")]
    Fetching,
    #[at("/signing")]
    Signing,
    #[not_found]
    #[at("/")]
    Home,
}

fn main() {
    yew::Renderer::<SubxtExamplesApp>::new().render();
}

struct SubxtExamplesApp;

impl Component for SubxtExamplesApp {
    type Message = ();

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        SubxtExamplesApp
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        }
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Fetching => {
            html! { <FetchingExamplesComponent/> }
        }
        Route::Signing => html! { <SigningExamplesComponent/> },
        Route::Home => {
            html! {
            <div>
                <h1>{"Welcome to the Subxt WASM examples!"}</h1>
                <a href="/signing"> <button>{"Signing Examples"} </button></a>
                <a href="/fetching"> <button>{"Fetching and Subscribing Examples"}</button></a>
            </div> }
        }
    }
}
