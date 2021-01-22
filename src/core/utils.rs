use chrono::{DateTime, Duration, Local, Utc};

pub fn date_delta_local(delta: i64) -> DateTime<Local> {
    if delta < 0 {
        Local::now()
            .checked_sub_signed(Duration::days(delta * -1))
            .unwrap()
    } else {
        Local::now()
            .checked_add_signed(Duration::days(delta))
            .unwrap()
    }
}

pub fn date_delta_utc(delta: i64) -> DateTime<Utc> {
    if delta < 0 {
        Utc::now()
            .checked_sub_signed(Duration::days(delta * -1))
            .unwrap()
    } else {
        Utc::now()
            .checked_add_signed(Duration::days(delta))
            .unwrap()
    }
}
