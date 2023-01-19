use super::{weekday_strs, LogElement, WEEKDAYS};
use chrono::Datelike;
use itertools::Itertools;
use prettytable::{row, Cell, Row, Table};

pub fn print_logs_table(logs: &Vec<LogElement>) {
    let mut table = Table::new();
    let mut header = row!["Ticket"];
    for weekday_str in &weekday_strs() {
        header.add_cell(Cell::new(weekday_str));
    }
    table.add_row(header);

    let unique_tickets = logs.iter().map(|it| &it.ticket).unique();
    let tickets_days = logs
        .iter()
        .into_group_map_by(|it| (it.date.weekday().num_days_from_monday(), it.ticket.as_str()));

    for ticket in unique_tickets {
        let mut row = Row::empty();
        row.add_cell(Cell::new(ticket));

        for weekday in WEEKDAYS {
            let cell_text = tickets_days
                .get(&(weekday.num_days_from_monday(), ticket))
                .unwrap_or(&vec![])
                .iter()
                .map(|it| format!("{}h {}", it.hours, it.tag.as_deref().unwrap_or("")))
                .join("\n");

            row.add_cell(Cell::new(&cell_text));
        }

        table.add_row(row);
    }

    table.printstd();
}
