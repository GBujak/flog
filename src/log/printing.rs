use super::{LogElement, WEEKDAYS};
use chrono::Datelike;

pub fn print_current_logs(logs: &Vec<LogElement>) {
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
