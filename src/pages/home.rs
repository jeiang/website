use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <h2> {"Who I am"} </h2>
            <p>
                {"My name is Aidan Pinard. I am currently a university student. "}
                {"I'm interested in programming, *nix stuff, and Skyrim. "}
                {"Check out my "}
                <Link<Route> to={Route::About}>
                    {"About page"}
                </Link<Route>>
                {" for more!"}
            </p>
            <br />
            <h2>
                {"Stuff I've used/dabbled in"}
            </h2>
            <ul>
                <li> {"NixOS (currently my main OS) & Nix"} </li>
                <li> {"Rust"} </li>
                <li> {"C# & F#"} </li>
                <li> {"Microcontrollers (ESP & Arduino)"} </li>
                <li> {"Shell Scripting (PowerShell, Bash, Fish)"} </li>
            </ul>
        </>
    }
}
