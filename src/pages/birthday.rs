use log::trace;
use serde::{Deserialize, Serialize};
use time::macros::date;
use time::{Date, Duration};
use yew::prelude::*;
use yew_router::hooks::use_location;

use crate::types::url_date::URLDate;

fn from_unix_milliseconds(ms: f64) -> Date {
    let duration = Duration::milliseconds(ms as i64);
    let date = date!(1970 - 01 - 01) + duration;
    trace!(target: "Birthday", "{ms} was converted to a duration of {duration}. The real date was estimated to be {date}.");
    date
}

fn date_now() -> Date {
    let current_time = js_sys::Date::now();
    from_unix_milliseconds(current_time)
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct BirthdayInfo {
    // Workaround since serde_urlencoded does not work
    to_first_name: Option<String>,
    to_last_name: Option<String>,
    from_first_name: Option<String>,
    from_last_name: Option<String>,
    date: Option<URLDate>,
    message: Option<String>,
}

#[function_component(Birthday)]
pub fn birthday() -> Html {
    let params = if let Some(location) = use_location() {
        trace!(target: "Birthday", "Obtained location: {location:#?}.");
        match location.query() {
            Ok(query) => {
                trace!(target: "Birthday", "Parsed query as: {query:#?}.");
                query
            },
            Err(err) => {
                trace!(target: "Birthday", "Failed to parse query because: {err}.");
                BirthdayInfo::default()
                // TODO: change this to show an error page
            },
        }
    } else {
        trace!(target: "Birthday", "Unable to get location. Creating default page.");
        BirthdayInfo::default()
    };

    let current_date = date_now();
    let BirthdayInfo {
        to_first_name,
        to_last_name,
        from_first_name,
        from_last_name,
        date,
        message,
    } = params;

    let is_birthday = if let Some(date) = &date {
        (current_date.month() == date.month()) && (current_date.day() == date.day())
    } else {
        true
    };

    let age = if let Some(Some(birth_year)) = date.as_ref().map(|d| d.year()) {
        if is_birthday {
            // If current date is valid, this should not break
            Some(current_date.year() - birth_year)
        } else {
            None
        }
    } else {
        None
    };

    let birthday_image = use_memo(
        |_| {
            html! {
                <>
                    <img alt="Happy birthday" src="https://source.unsplash.com/random/?birthday" />
                </>
            }
        },
        (),
    );

    let not_birthday_message = use_memo(
        |_| {
            html! {
                <>
                    <h2>{ "Today is not your birthday..." }</h2>
                    <p>{ "Wait for your actual birthday for a surprise!" }</p>
                </>
            }
        },
        (),
    );

    let birthday_greeting = use_memo(
        |name| match name {
            (Some(first_name), Some(last_name)) => html! {
                <h2>{ format!("Happy birthday, {first_name} {last_name}!") }</h2>
            },
            (Some(first_name), None) => html! {
                <h2>{ format!("Happy birthday, {first_name}!") }</h2>
            },
            (None, Some(last_name)) => html! {
                <h2>{ format!("Happy birthday, {last_name}!") }</h2>
            },
            (None, None) => html! {
                <h2>{ format!("Happy birthday!") }</h2>
            },
        },
        (to_first_name, to_last_name),
    );

    let custom_message = use_memo(
        |(message, from_first_name, from_last_name)| {
            let full_name = match (from_first_name, from_last_name) {
                (Some(first_name), Some(last_name)) => Some(format!("{first_name} {last_name}")),
                (Some(name), None) | (None, Some(name)) => Some(name.into()),
                (None, None) => None,
            };
            match (message, full_name) {
                (Some(message), Some(name)) => html! {
                    <p>
                        { format!("{name} wanted to tell you on your special day: \"") }
                        <em>{ message }</em>
                        { "\"." }
                    </p>
                },
                (None, Some(name)) => html! {
                    <p>{ format!("{name} wanted to tell you to have an amazing day!") }</p>
                },
                (Some(message), None) => html! {
                    <p>
                        { format!("A special someone wanted to tell you: \"") }
                        <em>{ message }</em>
                        { "\"." }
                    </p>
                },
                (None, None) => html! {
                    <p>{ "We hope you have a great day! And cheers to the years ahead!" }</p>
                },
            }
        },
        (message, from_first_name, from_last_name),
    );

    let age_congratulation = use_memo(
        |age| {
            html! {
                if let Some(age) = age {
                    <p>{ format!("Congrats on becoming {} years old today!", age) }</p>
                }
            }
        },
        age,
    );

    // TODO: add a button leading to back to a customize page

    html! {
        <div class="center-text">
            if is_birthday {
                {(*birthday_greeting).clone()}
                {(*age_congratulation).clone()}
                {(*birthday_image).clone()}
                {(*custom_message).clone()}
            } else {
                {(*not_birthday_message).clone()}
            }
        </div>
    }
}
