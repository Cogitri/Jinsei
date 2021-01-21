use chrono::{DateTime, FixedOffset};
pub struct Weight {
    pub date: DateTime<FixedOffset>,
    pub weight: u32,
}
