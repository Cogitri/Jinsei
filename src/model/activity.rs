use crate::{imp_getter_setter, model::ActivityType};
use chrono::{DateTime, Duration, FixedOffset, Utc};
use gdk::subclass::prelude::ObjectSubclass;
use uom::si::u32::Length;

mod imp {
    use crate::inner_refcell_getter_setter;

    use super::*;
    use glib::subclass;
    use gtk::subclass::prelude::*;
    use std::cell::RefCell;

    #[derive(Debug)]
    pub struct ActivityMut {
        pub activity_type: ActivityType,
        pub calories_burned: Option<u32>,
        pub date: DateTime<FixedOffset>,
        pub distance: Option<Length>,
        pub heart_rate_avg: Option<u32>,
        pub heart_rate_max: Option<u32>,
        pub heart_rate_min: Option<u32>,
        pub duration: Duration,
        pub steps: Option<u32>,
    }

    pub struct Activity {
        inner: RefCell<ActivityMut>,
    }

    impl ObjectSubclass for Activity {
        const NAME: &'static str = "HealthActivity";
        type ParentType = glib::Object;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;
        type Type = super::Activity;
        type Interfaces = ();

        glib::object_subclass!();

        fn new() -> Self {
            Self {
                inner: RefCell::new(ActivityMut {
                    activity_type: ActivityType::OtherSports,
                    calories_burned: None,
                    date: Utc::now().into(),
                    distance: None,
                    heart_rate_avg: None,
                    heart_rate_max: None,
                    heart_rate_min: None,
                    duration: Duration::seconds(0),
                    steps: None,
                }),
            }
        }
    }

    impl ObjectImpl for Activity {}

    impl Activity {
        inner_refcell_getter_setter!(activity_type, ActivityType);
        inner_refcell_getter_setter!(calories_burned, Option<u32>);
        inner_refcell_getter_setter!(date, DateTime<FixedOffset>);
        inner_refcell_getter_setter!(distance, Option<Length>);
        inner_refcell_getter_setter!(heart_rate_avg, Option<u32>);
        inner_refcell_getter_setter!(heart_rate_max, Option<u32>);
        inner_refcell_getter_setter!(heart_rate_min, Option<u32>);
        inner_refcell_getter_setter!(duration, Duration);
        inner_refcell_getter_setter!(steps, Option<u32>);
    }
}

glib::wrapper! {
    pub struct Activity(ObjectSubclass<imp::Activity>);
}

impl Activity {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create Activity")
    }

    fn get_priv(&self) -> &imp::Activity {
        imp::Activity::from_instance(self)
    }

    imp_getter_setter!(activity_type, ActivityType);
    imp_getter_setter!(calories_burned, Option<u32>);
    imp_getter_setter!(date, DateTime<FixedOffset>);
    imp_getter_setter!(distance, Option<Length>);
    imp_getter_setter!(heart_rate_avg, Option<u32>);
    imp_getter_setter!(heart_rate_max, Option<u32>);
    imp_getter_setter!(heart_rate_min, Option<u32>);
    imp_getter_setter!(duration, Duration);
    imp_getter_setter!(steps, Option<u32>);
}
