use anyhow::Context;
use chrono::{Days, NaiveDate, Weekday};
use std::collections::HashMap;

use super::LogElement;

pub struct LogBuffer {
    buffer: HashMap<Weekday, Vec<BufElement>>,
}

struct BufElement {
    pub ticket: String,
    pub tag: Option<String>,
}

impl LogBuffer {
    pub fn new() -> Self {
        LogBuffer {
            buffer: HashMap::new(),
        }
    }

    pub fn clear_day(&mut self, weekday: Weekday) {
        self.get_mut_vec(weekday).clear();
    }

    pub fn log_on_day(&mut self, weekday: Weekday, ticket: &str, tag: &Option<String>) {
        self.get_mut_vec(weekday).push(BufElement {
            ticket: ticket.into(),
            tag: tag.clone(),
        })
    }

    pub fn to_serializable(
        &self,
        week_start: &NaiveDate,
        untagged_per_day: u8,
        untagged_ticket: &str,
    ) -> Vec<LogElement> {
        let mut result = Vec::new();

        for (weekday, buf_elements) in self.buffer.iter() {
            let date = Self::week_start_and_weekday_to_date(week_start, *weekday);

            let mut tmp_result = buf_elements
                .into_iter()
                .map(|it| LogElement {
                    ticket: it.ticket.clone(),
                    tag: it.tag.clone(),
                    hours: 0,
                    date: date.clone(),
                })
                .collect::<Vec<_>>();

            let mut time_left = 8_u8 - untagged_per_day;

            for index in (0..tmp_result.len()).cycle() {
                tmp_result[index].hours += 1;
                time_left -= 1;

                if time_left == 0 {
                    break;
                }
            }

            if !tmp_result.iter().any(|it| it.tag.is_none()) {
                tmp_result.push(LogElement {
                    ticket: untagged_ticket.into(),
                    tag: None,
                    hours: untagged_per_day,
                    date,
                })
            }

            result.append(&mut tmp_result)
        }

        result
    }

    fn get_mut_vec(&mut self, weekday: Weekday) -> &mut Vec<BufElement> {
        self.buffer.entry(weekday).or_insert(Vec::with_capacity(8))
    }

    fn week_start_and_weekday_to_date(week_start: &NaiveDate, weekday: Weekday) -> NaiveDate {
        week_start
            .checked_add_days(Days::new(weekday.num_days_from_monday() as u64))
            .context("Date out of range.")
            .unwrap()
    }
}
