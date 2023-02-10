use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

const IMG_404: &str = "https://files.catbox.moe/cmsfhg.webp";

#[function_component(Error404NotFound)]
pub fn not_found() -> Html {
    let not_found_page = use_memo(
        |_| {
            html! {
                <section>
                    <h1>{"This page does not exist!" }</h1>
                    <h3>{ "You have entered the shadow realm." }</h3>
                    <br />
                    <img src={IMG_404} alt="Look's like you're going to the shadow realm, Jimbo" />
                    <br />
                    <p>
                        { "Do you want to head back to " }
                        <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                        { "?" }
                    </p>
                </section>
            }
        },
        (),
    );
    (*not_found_page).clone()
}
