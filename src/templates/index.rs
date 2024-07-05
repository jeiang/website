use perseus::prelude::*;
use sycamore::prelude::*;

use crate::components::head::StylesheetHead;
use crate::components::link::Link;
use crate::components::page_wrapper::PageWrapper;
use crate::components::typography::{Heading, HeadingVariant};

fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        PageWrapper {
            Heading(variant = HeadingVariant::H2) { "Who I am" }
            p {
                "I'm Aidan Pinard. Currently, I'm a developer at "
                Link(href = "https://www.cibcfcib.com/") { "CIBC Caribbean" }
                ". I'm also a student at UWI, and I will be soon graduating with a BSc in Electrical and Computer Engineering. "
                "I'm passionate about technology, specifically Nix/NixOS, *nix, and recently F1."
            }
            Heading(variant = HeadingVariant::H2) { "What I use" }

            p {
                "I'm currently dabbling in a lot of things, but some of the things I am most interested in are:"
            }

            ul(class = "list-disc list-inside") {
                li { "Nix/NixOS (currently used for my main desktop and webserver)" }
                li { "Rust/Zig/Go (I use them occasionally for hobby projects)" }
                li { "Microcontrollers (ESPs and Arduinos)" }
                li { "F1 (I was recently introduced to it, I'm closely following the races)" }
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
    Template::build("index").view(index_page).head(head).build()
}
