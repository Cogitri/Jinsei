use chrono::{DateTime, FixedOffset};
use gio::prelude::*;
use gio::Settings;

pub enum Unitsystem {
    Imperal,
    Metric,
}

#[derive(Debug)]
pub struct JinseiSettings {
    settings: Settings,
}

macro_rules! settings_getter_setter {
    ($type:ty, $name:ident, $key:literal) => {
        paste::item! {
            pub fn [< get_ $name >] (&self) -> $type {
                self.settings.get::<$type>($key)
            }
        }
        paste::item! {
            pub fn [< set_ $name >] (&self, value: $type) {
                self.settings.set::<$type>($key, &value).unwrap();
            }
        }
        paste::item! {
            pub fn [< connect_ $name _changed >]<F: Fn(&Settings, &str) + 'static>(&self, f: F) -> glib::SignalHandlerId {
                self.settings.connect_changed(f)
            }
        }
    };
}

impl JinseiSettings {
    pub fn new() -> Self {
        Self {
            settings: Settings::new("dev.Cogitri.Jinsei"),
        }
    }

    pub fn get_recent_activity_types(&self) -> Vec<String> {
        self.settings
            .get_strv("recent-activity-types")
            .iter()
            .map(|s| s.to_string())
            .collect()
    }

    pub fn set_recent_activity_types(&self, value: &[&str]) {
        self.settings
            .set_strv("recent-activity-types", value)
            .unwrap();
    }

    pub fn get_timestamp_last_sync_google_fit(&self) -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc3339(
            self.settings
                .get_string("timestamp-last-sync-google-fit")
                .unwrap()
                .as_str(),
        )
        .unwrap()
    }

    pub fn set_timestamp_last_sync_google_fit(&self, value: DateTime<FixedOffset>) {
        self.settings
            .set_string("timestamp-last-sync-google-fit", &value.to_rfc3339())
            .unwrap();
    }

    pub fn get_unitsystem(&self) -> Unitsystem {
        match self.settings.get_enum("unitsystem") {
            0 => Unitsystem::Imperal,
            1 => Unitsystem::Metric,
            _ => std::unreachable!(),
        }
    }

    pub fn set_unitsystem(&self, value: Unitsystem) {
        self.settings.set_enum("unitsystem", value as i32).unwrap();
    }

    settings_getter_setter!(u32, current_view_id, "current-view-id");
    settings_getter_setter!(bool, did_initial_setup, "did-initial-setup");
    settings_getter_setter!(
        bool,
        sync_provider_setup_google_fit,
        "sync-provider-setup-google-fit"
    );
    settings_getter_setter!(u32, user_age, "user-age");
    settings_getter_setter!(u32, user_height, "user-height");
    settings_getter_setter!(u32, user_stepgoal, "user-stepgoal");
    settings_getter_setter!(u32, user_weight, "user-weight");
    settings_getter_setter!(i32, window_height, "window-height");
    settings_getter_setter!(bool, window_is_maximized, "window-is-maximized");
    settings_getter_setter!(i32, window_width, "window-width");
}
