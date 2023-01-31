use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

// TODO: Add a customize page for birthday page here

#[function_component(Navbar)]
pub fn navbar() -> Html {
    html! {
        <nav class="navbar">
            <h1> {"aidanpinard.co"} </h1>
            <Link<Route> to={Route::Home}>
                <button>
                    {"Home"}
                </button>
            </Link<Route>>
            <Link<Route> to={Route::About}>
                <button>
                    {"About"}
                </button>
            </Link<Route>>
            <Link<Route> to={Route::Contact}>
                <button>
                    {"Contact Me"}
                </button>
            </Link<Route>>
        </nav>
    }
}
