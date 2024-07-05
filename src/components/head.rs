use perseus::prelude::*;
use sycamore::prelude::*;

pub fn StylesheetHead(cx: Scope) -> View<SsrNode> {
    view! { cx,
        link(rel="stylesheet", href="index.css") {}
        link(rel="stylesheet", href="https://fonts.googleapis.com/css2?family=Fira+Mono&family=Inter&family=Merriweather:wght@700&display=swap") {}
        link(rel="preconnect", href="https://fonts.googleapis.com") {}
        link(rel="preconnect", href="https://fonts.gstatic.com", crossorigin=true) {}
    }
}
