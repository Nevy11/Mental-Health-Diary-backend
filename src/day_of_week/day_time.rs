use chrono::{Datelike, Local};

/// The method returns the current month in u32 format.
/// uses chrono's Local to get the current date; then extracts the current month required
pub fn current_month() -> u32 {
    let now = Local::now().date_naive();
    let month = now.month();
    month
}
