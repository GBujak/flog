use std::fmt::Display;
use std::str::FromStr;

use super::printing::print_logs_table;
use super::LogElement;
use super::{log_buffer::LogBuffer, weekday_strs};
use crate::{
    configuration::{Configuration, TagConfiguration},
    repo::branch_collecting::RepoBranch,
};
use anyhow::{anyhow, Result};
use chrono::{Datelike, Days, Utc, Weekday};

enum UserAction {
    ContinueAdding,
    Done,
    ClearDays,
}

impl Display for UserAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ContinueAdding => "Continue adding",
                Self::Done => "Finish",
                Self::ClearDays => "Clear Days",
            }
        )
    }
}

impl UserAction {
    fn iter() -> impl Iterator<Item = Self> {
        [Self::ContinueAdding, Self::Done, Self::ClearDays].into_iter()
    }
}

pub fn inquire_log(config: &Configuration, branches: &Vec<RepoBranch>) -> Result<Vec<LogElement>> {
    let mut log_buffer = LogBuffer::new();

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

    'main_loop: loop {
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
            log_buffer.log_on_day(day_of_week, &ticket, &tag.clone())
        }

        print_logs_table(log_buffer.to_serializable(&day, untagged_hours, &ticket));

        match inquire_action()? {
            UserAction::ContinueAdding => {}
            UserAction::ClearDays => {
                inquire_and_clear_days(&mut log_buffer)?;
                print_logs_table(log_buffer.to_serializable(&day, untagged_hours, &ticket));
            }
            UserAction::Done => {
                break 'main_loop Ok(log_buffer.to_serializable(&day, untagged_hours, &ticket))
            }
        }
    }
}

fn inquire_days_of_week() -> Result<Vec<Weekday>> {
    Ok(
        inquire::MultiSelect::new("Select days with the same worklog.", weekday_strs())
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

fn inquire_action() -> Result<UserAction> {
    Ok(inquire::Select::new("What to do next?", UserAction::iter().collect()).prompt()?)
}

fn inquire_and_clear_days(log_buffer: &mut LogBuffer) -> Result<()> {
    let days = inquire_days_of_week()?;
    for day in days {
        log_buffer.clear_day(day);
    }
    Ok(())
}
