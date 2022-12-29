use std::str::FromStr;

use crate::{
    configuration::{Configuration, TagConfiguration},
    repo::branch_collecting::RepoBranch,
};
use anyhow::{anyhow, Result};
use chrono::{Datelike, Days, NaiveDate, Utc, Weekday};
use serde::Serialize;

const WEEKDAYS: &'static [Weekday] = &[
    Weekday::Mon,
    Weekday::Tue,
    Weekday::Wed,
    Weekday::Thu,
    Weekday::Fri,
    Weekday::Sat,
    Weekday::Sun,
];

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

pub fn inquire_log(config: &Configuration, branches: &Vec<RepoBranch>) -> Result<Vec<LogElement>> {
    let mut result = Vec::<LogElement>::new();

    let mut day = inquire::DateSelect::new("Select week to log (day of week doesn't matter)")
        .with_vim_mode(true)
        .with_default(Utc::now().date_naive())
        .with_week_start(Weekday::Mon)
        .prompt()?;

    while day.weekday() != Weekday::Mon {
        day = day
            .checked_sub_days(Days::new(1))
            .ok_or(anyhow!("could not get monday"))?;
    }

    loop {
        let days_of_week = inquire_days_of_week()?;

        let untagged_hours =
            inquire::CustomType::<u8>::new("How much untagged work per day?").prompt()?;

        let ticket = inquire::Text::new("Which ticket to log for?")
            .with_default(&config.default_project)
            .prompt()?;

        let branch =
            inquire::Select::new("Which branch to log work on?", branches.clone()).prompt()?;

        let tag = inquire_tag_name(&branch, config)?;

        for day_of_week in days_of_week {
            for (hours, tag) in [(untagged_hours, None), (8 - untagged_hours, tag.clone())] {
                if hours <= 0 {
                    continue;
                }

                result.push(LogElement {
                    ticket: ticket.clone(),
                    tag,
                    hours,
                    date: day
                        .checked_add_days(Days::new(day_of_week.num_days_from_monday() as u64))
                        .unwrap(),
                })
            }
        }

        print_current_logs(&result);

        let should_continue = inquire::Confirm::new("Create more logs for this week?")
            .with_default(true)
            .prompt()?;
        if !should_continue {
            break;
        }
    }

    Ok(result)
}

fn inquire_days_of_week() -> Result<Vec<Weekday>> {
    let weekdays_strs = WEEKDAYS.iter().map(|it| it.to_string()).collect::<Vec<_>>();
    Ok(
        inquire::MultiSelect::new("Select days with the same worklog.", weekdays_strs)
            .with_vim_mode(true)
            .prompt()?
            .iter()
            .map(|it| Weekday::from_str(it).unwrap())
            .collect::<Vec<_>>(),
    )
}

fn inquire_tag_name(branch: &RepoBranch, config: &Configuration) -> Result<Option<String>> {
    let tag = inquire::Text::new("Override tag name:")
        .with_default(&make_tag(&branch.branch_name, &config.tag_configuration))
        .prompt()?;

    Ok(if tag.len() == 0 { None } else { Some(tag) })
}

fn make_tag(branch_name: &str, tag_configuration: &TagConfiguration) -> String {
    if let Some(tag_body) = branch_name
        .split(&tag_configuration.separator)
        .skip(tag_configuration.element_index)
        .next()
    {
        format!("{} {}", tag_configuration.prefix, tag_body)
    } else {
        format!("{} {}", tag_configuration.prefix, branch_name)
    }
}

fn print_current_logs(logs: &Vec<LogElement>) {
    println!("=== Current time logs: ===");

    for weekday in WEEKDAYS {
        let mut logs_for_weekday = logs
            .iter()
            .filter(|it| &it.date.weekday() == weekday)
            .collect::<Vec<_>>();

        logs_for_weekday.sort_by_key(|it| it.hours);
        logs_for_weekday.reverse();

        println!("{}", weekday.to_string());
        for log in logs_for_weekday {
            println!(
                "\t{}h : [{}] {}",
                log.hours,
                log.ticket,
                log.tag.as_ref().unwrap_or(&String::new())
            );
        }
    }
}
