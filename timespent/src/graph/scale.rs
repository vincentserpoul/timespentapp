use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Eq, PartialEq, Debug, Deserialize, Serialize, Clone, Copy, TS, Hash)]
#[ts(export)]
pub enum Scale {
    Day,
    Week,
    Month,
    Year,
    All,
}

impl Scale {
    pub fn iterator() -> impl Iterator<Item = Scale> {
        [
            Scale::Day,
            Scale::Week,
            Scale::Month,
            Scale::Year,
            Scale::All,
        ]
        .iter()
        .copied()
    }
}

pub fn naive_date_to_scale_x(date: &NaiveDate, scale: &Scale) -> String {
    match scale {
        Scale::Day => date.to_string(),
        Scale::Week => format!("{}-{}", date.year(), date.iso_week().week()),
        Scale::Month => format!("{}-{}", date.year(), date.month()),
        Scale::Year => format!("{}", date.year()),
        Scale::All => "all".to_string(),
    }
}
