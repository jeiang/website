use std::fmt::Display;
use std::str::FromStr;

use log::{debug, error, warn};
use once_cell::sync::Lazy;
use time::format_description::FormatItem;
use time::macros::format_description;
use time::Date;

#[derive(Debug, Clone, PartialEq)]
pub struct URLDate {
    date: Date,
    is_valid_year: bool,
}

impl std::ops::Deref for URLDate {
    type Target = Date;

    fn deref(&self) -> &Self::Target {
        &self.date
    }
}

impl URLDate {
    pub fn year(&self) -> Option<i32> {
        if self.is_valid_year {
            Some(self.date.year())
        } else {
            None
        }
    }

    pub fn month(&self) -> time::Month {
        self.date.month()
    }

    pub fn month_u8(&self) -> u8 {
        self.month() as u8
    }

    pub fn day(&self) -> u8 {
        self.date.day()
    }
}

impl Display for URLDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(year) = self.year() {
            write!(f, "{}-{}-{}", year, self.month_u8(), self.day())
        } else {
            write!(f, "{}-{}", self.month_u8(), self.day())
        }
    }
}

impl FromStr for URLDate {
    type Err = time::error::Parse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static YMD_FORMAT: Lazy<&'static [FormatItem<'static>]> =
            Lazy::new(|| format_description!("[year]-[month]-[day]"));

        let ymd_date = Date::parse(s, &YMD_FORMAT);
        let md_date = Date::parse(&format!("0000-{s}"), &YMD_FORMAT);
        match (ymd_date, md_date) {
            (Ok(date), Ok(_)) => {
                warn!("Successfully parsed date in both year-month-day format and month-day format. This should not occur. Defaulting to year-month-day.");
                Ok(Self {
                    date,
                    is_valid_year: true,
                })
            },
            (Ok(date), Err(_)) => {
                debug!("Successfully parsed date in year-month-day format as {date}.");
                Ok(Self {
                    date,
                    is_valid_year: true,
                })
            },
            (Err(_), Ok(date)) => {
                debug!("Successfully parsed date in month-day format as {date}.");
                Ok(Self {
                    date,
                    is_valid_year: false,
                })
            },
            (Err(ymd_err), Err(md_err)) => {
                error!("Failed to parse date. year-month-day format failed because: {ymd_err}. month-day format failed because {md_err}.");
                Err(ymd_err)
            },
        }
    }
}
