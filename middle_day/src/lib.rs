use chrono::{NaiveDate, Weekday};

pub fn middle_day(year: i32) -> Option<Weekday> {
    let first_day = NaiveDate::from_ymd(year, 1, 1);
    let last_day = NaiveDate::from_ymd(year, 12, 31);

    let total_days = last_day.signed_duration_since(first_day).num_days();
    if total_days % 2 == 0 {
        return None;
    }

    let middle_day = first_day + chrono::Duration::days(total_days / 2);
    Some(middle_day.weekday())
}
