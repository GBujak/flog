use chrono::{NaiveDate, Weekday};
use itertools::Itertools;
use serde::Serialize;

mod inquires;
mod log_buffer;
mod printing;

pub use inquires::inquire_log;

const WEEKDAYS: &'static [Weekday] = &[
    Weekday::Mon,
    Weekday::Tue,
    Weekday::Wed,
    Weekday::Thu,
    Weekday::Fri,
    Weekday::Sat,
    Weekday::Sun,
];

fn weekday_strs() -> Vec<String> {
    WEEKDAYS.into_iter().map(Weekday::to_string).collect_vec()
}

pub struct LogElement {
    pub ticket: String,
    pub tag: Option<String>,
    pub hours: u8,
    pub date: NaiveDate,
}

impl LogElement {
    pub fn to_serializable(self) -> LogElementSerializable {
        LogElementSerializable {
            ticket: self.ticket,
            tag: self.tag,
            hours: self.hours,
            date: self.date.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct LogElementSerializable {
    ticket: String,
    tag: Option<String>,
    hours: u8,
    date: String,
}
