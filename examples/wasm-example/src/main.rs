use routes::signing::SigningExamplesComponent;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::fetching::FetchingExamplesComponent;
mod routes;
mod services;

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

    fn create(_ctx: &Context<Self>) -> Self {
        SubxtExamplesApp
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
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
