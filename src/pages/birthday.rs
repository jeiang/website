use log::trace;
use yew::prelude::*;
use yew_router::hooks::use_location;

use crate::components::birthday::celebrate::Celebrator;
use crate::components::birthday::customize::{BirthdayCustomizer, BirthdayError};

#[function_component(Birthday)]
pub fn birthday() -> Html {
    if let Some(location) = use_location() {
        trace!("Obtained location: {location:#?}.");
        match location.query_str().trim() {
            "" | "?" => {
                trace!("No query was provided, going to birthday page customizer.");
                html! {
                    <BirthdayCustomizer error={None} />
                }
            },
            c if c.len() == 1 => {
                log::error!("Invalid value provided for query string: \"{c}\"");
                let error = BirthdayError::new("Unexpected query value provided.");
                html! {
                    <BirthdayCustomizer error={error} />
                }
            },
            query => {
                let query = query[1..].to_owned();
                html! {
                    <Celebrator query={query} />
                }
            },
        }
    } else {
        trace!("Unable to get location. Creating default page.");
        html! {
            <BirthdayCustomizer error={None} />
        }
    }
}
