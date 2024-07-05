use perseus::prelude::*;
use sycamore::prelude::*;

#[component(inline_props)]
pub fn Link<'a, G: Html>(cx: Scope<'a>, href: &'a str, children: Children<'a, G>) -> View<G> {
    let children = children.call(cx);
    view! { cx,
        a(class = "underline", href = href) {(children)}
    }
}
