use perseus::prelude::*;
use sycamore::prelude::*;

use crate::components::typography::{Heading, HeadingVariant};

const BUTTON_CLASS: &str = "border-2 rounded-md border-primary dark:border-secondary hover:bg-primary hover:text-secondary dark:hover:bg-secondary dark:hover:text-primary px-2 bg-clip-border";

#[component(inline_props)]
pub fn PageWrapper<'a, G: Html>(cx: Scope<'a>, children: Children<'a, G>) -> View<G> {
    let children = children.call(cx);
    view! { cx,
        div(class = "h-dvh w-dvw p-8 flex flex-col justify-center items-center text-base lg:text-lg text-primary bg-secondary dark:bg-primary dark:text-secondary") {
            header(class = "w-full max-w-2xl flex flex-col items-left") {
                Heading(variant = HeadingVariant::H2) { "Aidan Pinard" }
                nav(class = "w-full flex flex-row space-x-4 py-4") {
                    a(class = BUTTON_CLASS, href = "/") { "Home" }
                    a(class = BUTTON_CLASS, href = "/about") { "About" }
                    a(class = BUTTON_CLASS, href = "/contact") { "Contact" }
                }
            }
            main(class = "w-full max-w-2xl flex-grow overflow-y-auto") {
                (children)
            }
            footer(class = "text-center pt-4 w-full max-w-2xl") {
                hr(class = "h-0.5 bg-primary dark:bg-secondary") {}
                p(class = "pt-4") { "Designed and made by Aidan Pinard - © 2024" }
            }
        }
    }
}
