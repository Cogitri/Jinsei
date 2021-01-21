use chrono::{DateTime, FixedOffset};
pub struct Weight {
    pub date: DateTime<FixedOffset>,
    pub weight: uom::si::f32::Mass,
}

impl Weight {
    pub fn new(date: DateTime<FixedOffset>, weight: uom::si::f32::Mass) -> Self {
        Self {
            date,
            weight,
        }
    }
}