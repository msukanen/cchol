use chrono::{Datelike, Local};

pub fn is_april_fools() -> bool {
    let now = Local::now();
    now.month() == 4 && now.day() == 1
}