use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::about::About;
use crate::pages::birthday::{Birthday, YMDDate};
use crate::pages::contact::Contact;
use crate::pages::home::Home;
use crate::pages::not_found::Error404NotFound;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
    // Need to add a date option
    #[at("/happy_birthday/:birthday/:first_name/:last_name/")]
    Birthday {
        first_name: String,
        last_name: String,
        birthday: YMDDate,
    },
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::About => html! { <About /> },
        Route::Contact => html! { <Contact /> },
        Route::Birthday {
            first_name,
            last_name,
            birthday,
        } => {
            html! { <Birthday first_name={first_name} last_name={last_name} birthday={birthday} /> }
        },
        Route::NotFound => html! { <Error404NotFound /> },
    }
}
