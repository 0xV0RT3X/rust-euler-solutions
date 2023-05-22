use std::time::Instant;
use chrono::{NaiveDate, Weekday, Datelike};

fn count_sundays(from: &NaiveDate, to: &NaiveDate) -> u32 {
    let mut current_date = *from;
    let mut sundays = 0;

    while current_date <= *to {
        if current_date.day() == 1 && current_date.weekday() == Weekday::Sun {
            sundays += 1;
        }

        current_date = current_date.succ_opt().unwrap();
    }

    sundays
}

fn main() {
    let start = Instant::now();
    let from = NaiveDate::from_ymd_opt(1901, 1, 1).unwrap();
    let to = NaiveDate::from_ymd_opt(2000, 12, 31).unwrap();
    println!("Solution: {}", count_sundays(&from, &to));
    let end = Instant::now();

    println!("Elapsed time: {:?}", end - start);
}
