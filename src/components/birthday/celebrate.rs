use std::str::FromStr;

use log::trace;
use time::Date;
use yew::prelude::*;

use crate::types::query_params::QueryParams;
use crate::types::url_date::URLDate;

// TODO: Say happy belated birthday up to 1 month after birthday
// TODO: Default page should provide a creator
// TODO: If one field fails to parse, should redirect to an error page.

// BUG: Many assumptions about conversions & unwraps
// TODO: Make fallible
fn date_now() -> Date {
    let current_time = js_sys::Date::new_0();
    let year = current_time.get_full_year() as i32;
    let month = match current_time.get_month() {
        0 => time::Month::January,
        1 => time::Month::February,
        2 => time::Month::March,
        3 => time::Month::April,
        4 => time::Month::May,
        5 => time::Month::June,
        6 => time::Month::July,
        7 => time::Month::August,
        8 => time::Month::September,
        9 => time::Month::October,
        10 => time::Month::November,
        11 => time::Month::December,
        _ => panic!("Expected month to be a number between 0 and 11"),
    };
    let day = current_time.get_date() as u8;

    let date = Date::from_calendar_date(year, month, day).unwrap();
    trace!(target: "Birthday", "Current local time is {current_time:?}. This was converted to {date}.");
    date
}

#[derive(Debug, Clone, PartialEq)]
enum Name {
    First(String),
    Last(String),
    Full { first: String, last: String },
}

#[derive(Debug, Default, Clone)]
struct BirthdayInfo {
    to: Option<Name>,
    from: Option<Name>,
    message: Option<String>,
    date_of_birth: Option<URLDate>,
}

impl From<QueryParams> for BirthdayInfo {
    fn from(value: QueryParams) -> Self {
        // TODO: clean this up
        let to_first_name = value
            .get_nonempty_str("to_first_name")
            .map(|s| s.to_string());

        let to_last_name = value
            .get_nonempty_str("to_last_name")
            .map(|s| s.to_string());

        let from_first_name = value
            .get_nonempty_str("from_first_name")
            .map(|s| s.to_string());

        let from_last_name = value
            .get_nonempty_str("from_last_name")
            .map(|s| s.to_string());

        let message = value.get_nonempty_str("message").map(|s| s.to_string());

        let date_of_birth = value
            .get("date")
            .map(|date_str| URLDate::from_str(date_str).ok())
            .flatten();
        let to = match (to_first_name, to_last_name) {
            (None, None) => None,
            (None, Some(last)) => Some(Name::Last(last)),
            (Some(first), None) => Some(Name::First(first)),
            (Some(first), Some(last)) => Some(Name::Full { first, last }),
        };
        let from = match (from_first_name, from_last_name) {
            (None, None) => None,
            (None, Some(last)) => Some(Name::Last(last)),
            (Some(first), None) => Some(Name::First(first)),
            (Some(first), Some(last)) => Some(Name::Full { first, last }),
        };
        let info = Self {
            to,
            from,
            message,
            date_of_birth,
        };
        trace!(target: "Birthday", "Parsed params as {info:#?}");
        info
    }
}

#[derive(Debug, Default, Clone, PartialEq, Properties)]
pub struct CelebratorProps {
    pub query: String,
}

#[function_component(Celebrator)]
pub fn create_birthday_content(params: &CelebratorProps) -> Html {
    let current_date = date_now();
    // TODO: if fail, error page
    // TODO: handle unwrap
    let params: QueryParams = serde_urlencoded::from_str(&params.query).unwrap();
    let BirthdayInfo {
        to,
        from,
        message,
        date_of_birth,
    } = BirthdayInfo::from(params);

    let is_birthday = if let Some(date) = &date_of_birth {
        (current_date.month() == date.month()) && (current_date.day() == date.day())
    } else {
        true
    };

    let age = if let Some(Some(birth_year)) = date_of_birth.as_ref().map(|d| d.year()) {
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
            Some(Name::First(first)) => html! {
                <h2>{ format!("Happy birthday, {first}!") }</h2>
            },
            Some(Name::Last(last)) => html! {
                <h2>{ format!("Happy birthday, {last}!") }</h2>
            },
            Some(Name::Full { first, last }) => html! {
                <h2>{ format!("Happy birthday, {first} {last}!") }</h2>
            },
            None => html! {
                <h2>{ format!("Happy birthday!") }</h2>
            },
        },
        to,
    );

    let custom_message = use_memo(
        |(message, name)| {
            let full_name = match name {
                Some(Name::First(name)) | Some(Name::Last(name)) => Some(name.into()),
                Some(Name::Full { first, last }) => Some(format!("{} {}", first, last)),
                None => None,
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
        (message, from),
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
