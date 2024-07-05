use perseus::prelude::*;
use sycamore::prelude::*;

use crate::components::head::StylesheetHead;
use crate::components::link::Link;
use crate::components::page_wrapper::PageWrapper;
use crate::components::typography::{Heading, HeadingVariant};

fn contact_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        PageWrapper {
            Heading(variant = HeadingVariant::H2) { "Contact me!" }
            // TODO: add a ntfy.sh post endpoint here
            p { "You can contact me (or see what I'm up to) via:" }

            ul(class = "list-disc list-inside") {
                li { Link(href = "https://anilist.co/user/jeiang") {"AniList"} }
                li { Link(href = "https://discordapp.com/users/343517984222085134") {"Discord"} }
                li { Link(href = "mailto:aidan@aidanpinard.co") {"Email"} }
                li { Link(href = "https://github.com/jeiang/") {"Github"} }
                li { Link(href = "https://app.roll20.net/users/6153276/jeiang") {"Roll20"} }
                li { Link(href = "https://twitter.com/jeiang_") {"Twitter"} }
                li { Link(href = "https://www.youtube.com/@jeiang") {"YouTube"} }
            }
        }
    }
}

#[engine_only_fn]
fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Aidan Pinard - Home" }
        StylesheetHead()
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::build("contact")
        .view(contact_page)
        .head(head)
        .build()
}
