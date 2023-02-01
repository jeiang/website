use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer">
            <hr />
            <p>
                {"Made by Aidan Pinard using "}
                <a href="https://rust-lang.org"> {"Rust"} </a>
                {"."}
            </p>
        </footer>
    }
}
