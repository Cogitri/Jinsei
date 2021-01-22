use std::convert::TryInto;

use crate::{core::HealthDatabase, views::Point};
use chrono::{DateTime, Duration, FixedOffset};

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

impl HealthGraphModelSteps {
    pub fn new() -> Self {
        Self {
            database: HealthDatabase::new().unwrap(),
            vec: Vec::new(),
        }
    }

    pub fn get_today_step_count(&self) -> Option<u32> {
        let today = chrono::Local::now().date();
        self.vec
            .iter()
            .find(|s| today == s.date.date())
            .map(|s| s.steps)
    }

    pub fn get_streak_count(&self, date: DateTime, step_goal: u32) -> u32 {
        todo!();
    }

    pub async fn reload(&mut self) -> Result<(), glib::Error> {
        //FIXME: Allow reloading for more than 30 days
        self.vec = self
            .database
            .get_steps(
                chrono::Local::now()
                    .checked_sub_signed(Duration::days(30))
                    .unwrap()
                    .into(),
            )
            .await?;
        Ok(())
    }

    pub fn to_points(&self) -> Vec<crate::views::Point> {
        if self.vec.is_empty() {
            return Vec::new();
        }

        let first_date = self.vec.first().unwrap().date;
        let mut last_val = 0;
        let mut ret = Vec::with_capacity(self.vec.len());

        for (i, point) in self.vec.iter().enumerate() {
            for j in i..last_val {
                let date = first_date
                    .clone()
                    .checked_add_signed(Duration::days((i + j).try_into().unwrap()))
                    .unwrap();
                ret.push(Point { date, value: 0.0 });
            }
            ret.push(Point {
                date: point.date,
                value: point.steps as f32,
            });
            last_val = point
                .date
                .signed_duration_since(first_date)
                .num_days()
                .try_into()
                .unwrap();
        }

        ret
    }
}
