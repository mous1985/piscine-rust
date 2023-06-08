extern crate chrono;
use chrono::{Datelike, NaiveDate};

pub type wd = chrono::Weekday;

pub fn middle_day(year: i32) -> Option<wd> {
    let last_day = NaiveDate::from_ymd_opt(year, 12, 31)?;

    let days_in_year = last_day.ordinal();

    if days_in_year % 2 == 0 {
        None
    } else {
        let middle_day = NaiveDate::from_ymd_opt(year, 1, 1)?.checked_add_signed(chrono::Duration::days(days_in_year as i64 / 2))?;
        Some(middle_day.weekday())
    }
}
