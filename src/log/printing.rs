use super::{weekday_strs, LogElement};
use chrono::Datelike;
use itertools::Itertools;
use prettytable::{row, Cell, Table};

pub fn print_logs_table(mut logs: Vec<LogElement>) {
    let mut table = Table::new();
    let mut header = row![""];
    for weekday_str in &weekday_strs() {
        header.add_cell(Cell::new(weekday_str));
    }
    table.add_row(header);

    logs.sort_by_key(|it| (it.ticket.clone(), it.date.weekday().number_from_monday()));

    for (ticket, logs) in logs.iter().group_by(|it| &it.ticket).into_iter() {
        let mut row = row![ticket];
        for (_weekday, logs_for_weekday) in logs
            .into_iter()
            .group_by(|it| it.date.weekday())
            .into_iter()
        {
            row.add_cell(Cell::new(
                &logs_for_weekday
                    .into_iter()
                    .map(|it| format!("{}h {}", it.hours, it.tag.clone().unwrap_or("".into())))
                    .join("\n"),
            ))
        }
        table.add_row(row);
    }

    table.printstd();
}
