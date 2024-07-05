mod components;
mod error_views;
mod templates;

use perseus::prelude::*;
use sycamore::prelude::*;

#[perseus::main(perseus_axum::dflt_server)]
pub fn main<G: Html>() -> PerseusApp<G> {
    PerseusApp::new()
        .template(crate::templates::index::get_template())
        .template(crate::templates::contact::get_template())
        .error_views(crate::error_views::get_error_views())
        .static_alias("/index.css", "dist/output.css")
}
