use std::fmt::Display;
use std::str::FromStr;

use log::trace;
use once_cell::sync::Lazy;
use time::format_description::FormatItem;
use time::macros::{date, format_description};
use time::{Date, Duration, Instant};
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct YMDDate(Date);

impl Display for YMDDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

static YMD_FORMAT: Lazy<&'static [FormatItem<'static>]> =
    Lazy::new(|| format_description!("[year]-[month]-[day]"));

impl FromStr for YMDDate {
    type Err = time::error::Parse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date = Date::parse(s, &YMD_FORMAT)?;
        Ok(Self(date))
    }
}

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct BirthdayProps {
    pub first_name: String,
    pub last_name: String,
    pub birthday: YMDDate,
}

fn from_unix_milliseconds(ms: f64) -> Date {
    let duration = Duration::milliseconds(ms as i64);
    let date = date!(1970 - 01 - 01) + duration;
    trace!(target: "Birthday", "{ms} was converted to a duration of {duration}. The real date was estimated to be {date}.");
    date
}

#[function_component(Birthday)]
pub fn birthday(props: &BirthdayProps) -> Html {
    let BirthdayProps {
        first_name,
        last_name,
        birthday,
    } = props;
    let birthday = birthday.0;

    let current_time = js_sys::Date::now();
    trace!(target: "Birthday", "Current time is {} ms after unix epoch", current_time);
    let current_date = from_unix_milliseconds(current_time);
    trace!(target: "Birthday", "Current date is estimated to be {}", current_date);

    // If current date is valid, this should not break
    let birthday_this_year = birthday.replace_year(current_date.year()).unwrap();
    trace!(target: "Birthday", "Birthday this year should be on {}", birthday_this_year);

    let is_birthday = birthday_this_year == current_date;
    let age = current_date.year() - birthday.year();
    trace!(target: "Birthday", "Today is birthday: {}\nAge:{}", is_birthday, age);

    html! {
        <div class="center-text">
        if is_birthday {
            <h2>{ format!("Happy Birthday, {}!", first_name) }</h2>
            <p>{ format!("Congrats on becoming {} years old today!", age) }</p>
            <img alt="Happy birthday" src="https://source.unsplash.com/random/?birthday" />
        } else {
            <h2>{ format!("Today is not your birthday...") }</h2>
            <p>{ "Wait for your actual birthday for a surprise!" }</p>
        }
        </div>
    }
}
