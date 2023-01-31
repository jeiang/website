use log::info;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::about::About;
use crate::pages::birthday::Birthday;
use crate::pages::contact::Contact;
use crate::pages::home::Home;
use crate::pages::not_found::Error404NotFound;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
    #[at("/happy_birthday")]
    Birthday,
    #[at("/404")]
    NotFound,
}

pub fn root_switch(route: Route) -> Html {
    info!(target: "Router", "Navigating to {route:#?} at {path}", path = route.to_path());
    match route {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::Contact => html! { <Contact /> },
        Route::Birthday => html! { <Birthday /> },
        Route::NotFound => html! { <Error404NotFound /> },
    }
}
