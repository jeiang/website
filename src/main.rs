#![deny(clippy::all)]
#![warn(clippy::pedantic)]

mod components;
mod pages;
mod router;
mod types;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::router::{root_switch, Route};

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <main>
                <Header />
                <div class="main-content">
                    <Switch<Route> render={root_switch} /> // <- must be child of <BrowserRouter>
                </div>
                <Footer />
            </main>
        </BrowserRouter>
    }
}

fn main() {
    // Max level is overriden by the log crate features
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().render();
}
