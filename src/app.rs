use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <h1>{ "Hello World!" }</h1>
            <p>{ "This is a test to ensure that the webpage is looking ok!" }</p>
        </main>
    }
}
