#![deny(clippy::all)]
#![warn(clippy::pedantic)]

mod components;
mod pages;
mod router;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::footer::Footer;
use crate::components::navbar::Navbar;
use crate::router::{switch, Route};

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Navbar />
            <main class="content">
                <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
            </main>
            <Footer />
        </BrowserRouter>
    }
}

fn main() {
    // Max level is overriden by the log crate features
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().render();
}
