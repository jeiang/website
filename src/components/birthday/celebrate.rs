use std::str::FromStr;

use log::{debug, trace, warn};
use time::Date;
use yew::prelude::*;

use crate::types::query_params::QueryParams;
use crate::types::url_date::URLDate;

// TODO: Say happy belated birthday up to 1 month after birthday
// TODO: If one field fails to parse, should redirect to an error page.

// TODO: Make fallible
fn date_now() -> Date {
    let current_time = js_sys::Date::new_0();
    let year: i32 = match current_time.get_full_year().try_into() {
        Ok(year) => year,
        Err(err) => {
            warn!("Failed to convert year from u32 to i32 because: {err}. Performing lossy conversion.");
            current_time.get_full_year() as i32
        },
    };
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
        m => {
            warn!(
                "Invalid month was provided by `new Date()`: ({m}). Defaulting to December (12)."
            );
            time::Month::December
        },
    };
    let day: u8 = match current_time.get_date().try_into() {
        Ok(day) => day,
        Err(err) => {
            warn!(
                "Failed to convert day from u32 to u8 because: {err}. Performing lossy conversion."
            );
            current_time.get_date() as u8
        },
    };

    // FIXME: this can break. not expected however.
    let date = Date::from_calendar_date(year, month, day);
    let date = date.unwrap();
    let json_time = current_time.to_json();
    trace!("Current local time is {json_time}. This was converted to {date}.");
    date
}

#[derive(Debug, Clone, PartialEq)]
enum Name {
    First(String),
    Last(String),
    Full { first: String, last: String },
}

#[derive(Debug, Default, Clone, PartialEq)]
struct BirthdayInfo {
    to: Option<Name>,
    from: Option<Name>,
    message: Option<String>,
    date_of_birth: Option<URLDate>,
}

impl TryFrom<QueryParams> for BirthdayInfo {
    // NOTE: this does not "fail", it will return nothing if all fields are none
    type Error = ();

    fn try_from(value: QueryParams) -> Result<Self, Self::Error> {
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
        let birthday_info = Self {
            to,
            from,
            message,
            date_of_birth,
        };

        // NOTE: check if all fields are none
        if birthday_info == BirthdayInfo::default() {
            warn!("None of the required paramters were found in the query.");
            Err(())
        } else {
            trace!("Parsed query parameters as {birthday_info:#?}");
            Ok(birthday_info)
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Properties)]
pub struct CelebratorProps {
    pub query: String,
}

#[function_component(Celebrator)]
pub fn create_birthday_content(params: &CelebratorProps) -> Html {
    let current_date = date_now();
    // FIXME: Redirect to default page with an error if either of the following two returns Err
    let params: QueryParams = serde_urlencoded::from_str(&params.query).unwrap();
    let BirthdayInfo {
        to,
        from,
        message,
        date_of_birth,
    } = BirthdayInfo::try_from(params).unwrap_or_default();

    let is_birthday = if let Some(date) = &date_of_birth {
        let is_birthday =
            (current_date.month() == date.month()) && (current_date.day() == date.day());
        debug!(
            "Date of birth was provided, calculated that today is {}the birthday.",
            if is_birthday { "" } else { "not " }
        );
        is_birthday
    } else {
        debug!("");
        true
    };

    let age = if let Some(Some(birth_year)) = date_of_birth.as_ref().map(|d| d.year()) {
        if is_birthday {
            debug!(
                "Calculating age given that year of birth was provided, and today is the birthday."
            );
            // If current date is valid, this should not break
            Some(current_date.year() - birth_year)
        } else {
            debug!("Today is not the birthday, skipping age calc.");
            None
        }
    } else {
        debug!("No date of birth provided.");
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
            Some(Name::First(first)) => {
                debug!("Detected only first name.");
                html! {
                    <h2>{ format!("Happy birthday, {first}!") }</h2>
                }
            },
            Some(Name::Last(last)) => {
                debug!("Detected only last name.");
                html! {
                    <h2>{ format!("Happy birthday, {last}!") }</h2>
                }
            },
            Some(Name::Full { first, last }) => {
                debug!("Detected full name.");
                html! {
                    <h2>{ format!("Happy birthday, {first} {last}!") }</h2>
                }
            },
            None => {
                debug!("No name detected.");
                html! {
                    <h2>{ format!("Happy birthday!") }</h2>
                }
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
                (Some(message), Some(name)) => {
                    html! {
                        <p>
                            { format!("{name} wanted to tell you on your special day: \"") }
                            <em>{ message }</em>
                            { "\"." }
                        </p>
                    }
                },
                (None, Some(name)) => {
                    html! {
                        <p>{ format!("{name} wanted to tell you to have an amazing day!") }</p>
                    }
                },
                (Some(message), None) => {
                    html! {
                        <p>
                            { format!("A special someone wanted to tell you: \"") }
                            <em>{ message }</em>
                            { "\"." }
                        </p>
                    }
                },
                (None, None) => {
                    html! {
                        <p>{ "We hope you have a great day! And cheers to the years ahead!" }</p>
                    }
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
