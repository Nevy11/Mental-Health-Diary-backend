use chrono::{Local, NaiveDate};

pub fn date() -> NaiveDate {
    let now = Local::now().date_naive();
    now
}
