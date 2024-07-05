use perseus::errors::ClientError;
use perseus::prelude::*;
use sycamore::prelude::*;

use crate::components::head::StylesheetHead;
use crate::components::page_wrapper::PageWrapper;
use crate::components::typography::{Heading, HeadingVariant};

pub fn get_error_views<G: Html>() -> ErrorViews<G> {
    ErrorViews::new(|cx, err, _err_info, _err_pos| match err {
        ClientError::Panic(_) => (
            view! { cx,
                title { "Aidan Pinard - Critical Error" }
                StylesheetHead()
            },
            view! { cx,
                PageWrapper {
                    Heading(variant = HeadingVariant::H2) { "Error" }
                    p { "An unexpected error has occurred. Please try again later." }
                }
            },
        ),
        ClientError::ServerError { status, message } => match status {
            404 => (
                view! { cx,
                    title { "Aidan Pinard - Not Found" }
                    StylesheetHead()
                },
                view! { cx,
                    PageWrapper {
                        Heading(variant = HeadingVariant::H2) { "Error - Page Not Found" }
                        p { "The page you're looking for doesn't exist." }
                    }
                },
            ),
            _ if (400..500).contains(&status) => (
                view! { cx,
                    title { "Aidan Pinard - Client Error" }
                    StylesheetHead()
                },
                view! { cx,
                    PageWrapper {
                        Heading(variant = HeadingVariant::H2) { "Client Error" }
                        p { "Something was wrong with the last request, please try reloading the page." }
                        code { (message) }
                    }
                },
            ),
            _ => (
                view! { cx,
                    title { "Aidan Pinard - Server Error" }
                    StylesheetHead()
                },
                view! { cx,
                    PageWrapper {
                        Heading(variant = HeadingVariant::H2) { "Server Error" }
                        p { "An error occurred on the server. Please try again later." }
                        p { (message) }
                    }
                },
            ),
        },
        ClientError::FetchError(_) => (
            view! { cx,
                title { "Aidan Pinard - Network Error" }
                StylesheetHead()
            },
            view! { cx,
                PageWrapper {
                    Heading(variant = HeadingVariant::H2) { "Network Error" }
                    p { "A network error has occurred. Please check if you have internet connectivity (if you do, try reloading)." }
                }
            },
        ),
        _ => (
            view! { cx,
                title { "Aidan Pinard - Error" }
                StylesheetHead()
            },
            view! { cx,
                PageWrapper {
                    Heading(variant = HeadingVariant::H2) { "Error" }
                    p { "An unexpected error has occurred. Please try again later." }
                }
            },
        ),
    })
}
