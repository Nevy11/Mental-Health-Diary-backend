use chrono::{Datelike, Local};

/// uses chrono's time module to fetch the current date then extracts the current day from the NaiveDate.
pub fn current_day() -> String {
    let now = Local::now().date_naive();
    let day = now.weekday();
    day.to_string()
}
