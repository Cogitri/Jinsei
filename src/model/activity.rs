use crate::core::i18n;
use chrono::{DateTime, Duration, FixedOffset, Utc};

bitflags::bitflags! {
    pub struct ActivityDataPoints: u16 {
        const CaloriesBurned = 0b000001;
        const Distance = 0b000010;
        const Duration = 0b000100;
        const HeartRate = 0b001000;
        const StepCount = 0b010000;
    }
}

#[derive(Debug)]
pub struct Activity {
    pub activity_type: ActivityType,
    pub calories_burned: Option<u32>,
    pub date: DateTime<FixedOffset>,
    pub distance: Option<uom::si::u32::Length>,
    pub heart_rate_avg: Option<u32>,
    pub heart_rate_max: Option<u32>,
    pub heart_rate_min: Option<u32>,
    pub duration: Duration,
    pub steps: Option<u32>,
}

impl Default for Activity {
    fn default() -> Self {
        Self {
            activity_type: ActivityType::OtherSports,
            calories_burned: None,
            date: Utc::now().into(),
            distance: None,
            heart_rate_avg: None,
            heart_rate_max: None,
            heart_rate_min: None,
            duration: Duration::seconds(0),
            steps: None,
        }
    }
}

#[derive(Debug, num_derive::FromPrimitive, num_derive::ToPrimitive)]
pub enum ActivityType {
    Basketball,
    Bicycling,
    Boxing,
    Dancing,
    Football,
    Golf,
    Hiking,
    Hockey,
    HorseRiding,
    OtherSports,
    RollerBlading,
    Running,
    Skiing,
    Soccer,
    Softball,
    Swimming,
    Tennis,
    TrackAndField,
    VolleyBall,
    Walking,
}

#[derive(Debug)]
pub struct ActivityInfo {
    pub activity_type: ActivityType,
    pub available_data_points: ActivityDataPoints,
    pub average_calories_burned_per_minute: u32,
    pub name: String,
}

impl From<ActivityType> for ActivityInfo {
    fn from(activity_type: ActivityType) -> Self {
        match activity_type {
            ActivityType::Basketball => ActivityInfo::new(
                ActivityType::Basketball,
                ActivityDataPoints::CaloriesBurned
                    | ActivityDataPoints::Duration
                    | ActivityDataPoints::HeartRate,
                6,
                i18n("Basketball"),
            ),
            ActivityType::Bicycling => ActivityInfo::new(
                ActivityType::Bicycling,
                ActivityDataPoints::CaloriesBurned
                    | ActivityDataPoints::Duration
                    | ActivityDataPoints::HeartRate
                    | ActivityDataPoints::Distance,
                10,
                i18n("Bicycling"),
            ),
            ActivityType::Boxing => ActivityInfo::new(
                ActivityType::Boxing,
                ActivityDataPoints::CaloriesBurned
                    | ActivityDataPoints::Duration
                    | ActivityDataPoints::HeartRate,
                7,
                i18n("Boxing"),
            ),
            ActivityType::Dancing => ActivityInfo::new(
                ActivityType::Dancing,
                ActivityDataPoints::CaloriesBurned
                    | ActivityDataPoints::Duration
                    | ActivityDataPoints::HeartRate,
                8,
                i18n("Dancing"),
            ),
            ActivityType::Football => ActivityInfo::new(
                ActivityType::Football,
                ActivityDataPoints::CaloriesBurned
                    | ActivityDataPoints::Duration
                    | ActivityDataPoints::HeartRate,
                3,
                i18n("Football"),
            ),
            ActivityType::Golf => ActivityInfo::new(
                ActivityType::Golf,
                ActivityDataPoints::CaloriesBurned | ActivityDataPoints::Duration,
                4,
                i18n("Golf"),
            ),
            ActivityType::Hiking => ActivityInfo::new(
                ActivityType::Hiking,
                ActivityDataPoints::CaloriesBurned
                    | ActivityDataPoints::Duration
                    | ActivityDataPoints::HeartRate
                    | ActivityDataPoints::StepCount
                    | ActivityDataPoints::Distance,
                8,
                i18n("Hiking"),
            ),
            ActivityType::Hockey => ActivityInfo::new(
                ActivityType::Hockey,
                ActivityDataPoints::CaloriesBurned
                    | ActivityDataPoints::Duration
                    | ActivityDataPoints::HeartRate,
                10,
                i18n("Hockey"),
            ),
            ActivityType::HorseRiding => ActivityInfo::new(
                ActivityType::HorseRiding,
                ActivityDataPoints::CaloriesBurned
                    | ActivityDataPoints::Duration
                    | ActivityDataPoints::HeartRate
                    | ActivityDataPoints::Distance,
                5,
                i18n("Horse Riding"),
            ),
            ActivityType::OtherSports => ActivityInfo::new(
                ActivityType::OtherSports,
                ActivityDataPoints::CaloriesBurned
                    | ActivityDataPoints::Duration
                    | ActivityDataPoints::HeartRate,
                9,
                i18n("Other Sports"),
            ),
            ActivityType::RollerBlading => ActivityInfo::new(
                ActivityType::RollerBlading,
                ActivityDataPoints::CaloriesBurned
                    | ActivityDataPoints::Duration
                    | ActivityDataPoints::HeartRate
                    | ActivityDataPoints::Distance,
                10,
                i18n("Rollerblading"),
            ),
            ActivityType::Running => ActivityInfo::new(
                ActivityType::Running,
                ActivityDataPoints::CaloriesBurned
                    | ActivityDataPoints::Duration
                    | ActivityDataPoints::HeartRate
                    | ActivityDataPoints::Distance
                    | ActivityDataPoints::StepCount,
                15,
                i18n("Running"),
            ),
            ActivityType::Skiing => ActivityInfo::new(
                ActivityType::Skiing,
                ActivityDataPoints::CaloriesBurned
                    | ActivityDataPoints::Duration
                    | ActivityDataPoints::HeartRate
                    | ActivityDataPoints::Distance,
                12,
                i18n("Skiing"),
            ),
            ActivityType::Soccer => ActivityInfo::new(
                ActivityType::Soccer,
                ActivityDataPoints::CaloriesBurned
                    | ActivityDataPoints::Duration
                    | ActivityDataPoints::HeartRate,
                8,
                i18n("Soccer"),
            ),
            ActivityType::Softball => ActivityInfo::new(
                ActivityType::Softball,
                ActivityDataPoints::CaloriesBurned
                    | ActivityDataPoints::Duration
                    | ActivityDataPoints::HeartRate,
                5,
                i18n("Softball"),
            ),
            ActivityType::Swimming => ActivityInfo::new(
                ActivityType::Swimming,
                ActivityDataPoints::CaloriesBurned
                    | ActivityDataPoints::Duration
                    | ActivityDataPoints::HeartRate
                    | ActivityDataPoints::Distance,
                12,
                i18n("Swimming"),
            ),
            ActivityType::Tennis => ActivityInfo::new(
                ActivityType::Tennis,
                ActivityDataPoints::CaloriesBurned
                    | ActivityDataPoints::Duration
                    | ActivityDataPoints::HeartRate,
                6,
                i18n("Tennis"),
            ),
            ActivityType::TrackAndField => ActivityInfo::new(
                ActivityType::TrackAndField,
                ActivityDataPoints::CaloriesBurned
                    | ActivityDataPoints::Duration
                    | ActivityDataPoints::HeartRate,
                5,
                i18n("Track And Field"),
            ),
            ActivityType::VolleyBall => ActivityInfo::new(
                ActivityType::VolleyBall,
                ActivityDataPoints::CaloriesBurned
                    | ActivityDataPoints::Duration
                    | ActivityDataPoints::HeartRate
                    | ActivityDataPoints::Distance
                    | ActivityDataPoints::StepCount,
                5,
                i18n("Volleyball"),
            ),
            ActivityType::Walking => ActivityInfo::new(
                ActivityType::Walking,
                ActivityDataPoints::CaloriesBurned
                    | ActivityDataPoints::Duration
                    | ActivityDataPoints::HeartRate
                    | ActivityDataPoints::Distance
                    | ActivityDataPoints::StepCount,
                5,
                i18n("Walking"),
            ),
        }
    }
}

impl ActivityInfo {
    pub fn new(
        activity_type: ActivityType,
        available_data_points: ActivityDataPoints,
        average_calories_burned_per_minute: u32,
        name: String,
    ) -> Self {
        Self {
            activity_type,
            available_data_points,
            average_calories_burned_per_minute,
            name,
        }
    }
}
