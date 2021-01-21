use crate::model::{Activity, Steps, Weight};
use chrono::{DateTime, FixedOffset};

pub struct HealthDatabase {}

impl HealthDatabase {
    pub fn new() -> Result<Self, ()> {
        todo!();
    }

    async fn get_activities(date: Option<DateTime<FixedOffset>>) -> Result<Vec<Activity>, ()> {
        todo!();
    }

    async fn get_steps(date: Option<DateTime<FixedOffset>>) -> Result<Vec<Steps>, ()> {
        todo!();
    }

    async fn get_weights(date: Option<DateTime<FixedOffset>>) -> Result<Vec<Weight>, ()> {
        todo!();
    }

    async fn reset() -> Result<(), ()> {
        todo!();
    }

    async fn save_activity(activity: Activity) -> Result<(), ()> {
        todo!();
    }

    async fn save_weight(weight: Weight) -> Result<(), ()> {
        todo!();
    }
}
