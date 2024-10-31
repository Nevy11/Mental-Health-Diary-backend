use chrono::{Datelike, Local};

/// The method returns the current year in i32 format.
pub fn current_year() -> i32 {
    let now = Local::now().date_naive();
    let year = now.year();
    year
}
