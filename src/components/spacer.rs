use perseus::prelude::*;
use sycamore::prelude::*;

pub enum Spacing {
    None,
    Small,
    Medium,
    Large,
    ExtraLarge,
}

#[component(inline_props)]
pub fn Spacer<'a, G: Html>(cx: Scope<'a>, spacing: Spacing) -> View<G> {
    let class = match spacing {
        Spacing::None => "",
        Spacing::Small => "p-1",
        Spacing::Medium => "p-2",
        Spacing::Large => "p-4",
        Spacing::ExtraLarge => "p-8",
    };
    view! { cx,
        div(class = class) {}
    }
}
