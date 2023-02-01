use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

// TODO: Add a customize page for birthday page here

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <h1>{ "Aidan Pinard" }</h1>
            <nav>
                <Link<Route> to={Route::Home}>
                    <button>{ "Home" }</button>
                </Link<Route>>
                <span class="spacer"></span>
                <Link<Route> to={Route::About}>
                    <button>{ "About" }</button>
                </Link<Route>>
                <span class="spacer"></span>
                <Link<Route> to={Route::Contact}>
                    <button>{ "Contact" }</button>
                </Link<Route>>
            </nav>
        </header>
    }
}
