use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer">
            <p>
                {"Built by Aidan Pinard using "}
                <a href="https://rust-lang.org"> {"Rust "} </a>
                {"with "}
                <a href="https://yew.rs/"> {"Yew "} </a>
                {"and "}
                <a href="https://trunkrs.dev/"> {"Trunk"} </a>
                {". Domain by "}
                <a href="https://porkbun.com/"> {"Porkbun"} </a>
                {"."}
            </p>
        </footer>
    }
}
