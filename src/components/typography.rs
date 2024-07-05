use perseus::prelude::*;
use sycamore::prelude::*;

pub enum HeadingVariant {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

#[component(inline_props)]
pub fn Heading<'a, G: Html>(
    cx: Scope<'a>,
    children: Children<'a, G>,
    variant: HeadingVariant,
) -> View<G> {
    let children = children.call(cx);
    match variant {
        HeadingVariant::H1 => view! { cx,
            h1(class = "text-4xl lg:text-5xl font-serif tracking-wide py-2") {
                (children)
            }
        },
        HeadingVariant::H2 => view! { cx,
            h2(class = "text-3xl lg:text-4xl font-serif tracking-wide py-2") {
                (children)
            }
        },
        HeadingVariant::H3 => view! { cx,
            h3(class = "text-2xl lg:text-3xl font-serif tracking-wide py-2") {
                (children)
            }
        },
        HeadingVariant::H4 => view! { cx,
            h4(class = "text-xl lg:text-2xl font-serif py-1") {
                (children)
            }
        },
        HeadingVariant::H5 => view! { cx,
            h5(class = "text-lg lg:text-xl font-serif py-1") {
                (children)
            }
        },
        HeadingVariant::H6 => view! { cx,
            h5(class = "text-base lg:text-lg font-serif py-1") {
                (children)
            }
        },
    }
}
