use chrono::{DateTime, FixedOffset};

use crate::core::HealthDatabase;
use super::HealthGraphModel;

#[derive(Debug)]
pub struct Steps {
    pub date: DateTime<FixedOffset>,
    pub steps: u32,
}

impl Steps {
    pub fn new(date: DateTime<FixedOffset>, steps: u32) -> Self {
        Self { date, steps }
    }
}


#[derive(Debug)]
pub struct HealthGraphModelSteps {
    database: HealthDatabase,
    vec: Vec<Steps>,
}

impl HealthGraphModel for HealthGraphModelSteps {
    fn to_points() -> Vec<crate::views::Point> {
        todo!()
    }

    fn reload() -> bool {
        todo!()
    }
}

impl HealthGraphModelSteps {
    pub fn new () -> Self {
        Self {
            database: HealthDatabase::new().unwrap(),
            vec: Vec::new(),
        }
    }

    pub fn get_streak_count(step_goal: u32) -> u32 {
        todo!();
    }
}