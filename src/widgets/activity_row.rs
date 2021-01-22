use gtk::prelude::*;
use gtk::{glib, CompositeTemplate};

mod imp {
    use super::*;
    use crate::{
        core::{i18n_f, settings::Unitsystem, HealthSettings},
        model::{Activity, ActivityDataPoints, ActivityInfo},
    };
    use glib::subclass;
    use gtk::subclass::prelude::*;
    use std::cell::RefCell;
    use uom::fmt::DisplayStyle::Abbreviation;
    use uom::si::length::{meter, yard};

    #[derive(Debug, CompositeTemplate)]
    pub struct HealthActivityRow {
        pub activity: RefCell<Option<Activity>>,
        pub settings: HealthSettings,
        #[template_child]
        pub active_minutes_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub activity_date_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub activity_type_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub calories_burned_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub distance_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub heart_rate_average_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub heart_rate_maximum_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub heart_rate_minimum_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub steps_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub details_revealer: TemplateChild<gtk::Revealer>,
        #[template_child]
        pub calories_burned_row: TemplateChild<libadwaita::ActionRow>,
        #[template_child]
        pub distance_row: TemplateChild<libadwaita::ActionRow>,
        #[template_child]
        pub heart_rate_average_row: TemplateChild<libadwaita::ActionRow>,
        #[template_child]
        pub heart_rate_maximum_row: TemplateChild<libadwaita::ActionRow>,
        #[template_child]
        pub heart_rate_minimum_row: TemplateChild<libadwaita::ActionRow>,
        #[template_child]
        pub steps_row: TemplateChild<libadwaita::ActionRow>,
    }

    impl ObjectSubclass for HealthActivityRow {
        const NAME: &'static str = "HealthActivityRow";
        type ParentType = gtk::ListBoxRow;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;
        type Type = super::HealthActivityRow;

        glib::object_subclass!();

        fn new() -> Self {
            Self {
                activity: RefCell::new(None),
                settings: HealthSettings::new(),
                active_minutes_label: TemplateChild::default(),
                activity_date_label: TemplateChild::default(),
                activity_type_label: TemplateChild::default(),
                calories_burned_label: TemplateChild::default(),
                distance_label: TemplateChild::default(),
                heart_rate_average_label: TemplateChild::default(),
                heart_rate_maximum_label: TemplateChild::default(),
                heart_rate_minimum_label: TemplateChild::default(),
                steps_label: TemplateChild::default(),
                details_revealer: TemplateChild::default(),
                calories_burned_row: TemplateChild::default(),
                distance_row: TemplateChild::default(),
                heart_rate_average_row: TemplateChild::default(),
                heart_rate_maximum_row: TemplateChild::default(),
                heart_rate_minimum_row: TemplateChild::default(),
                steps_row: TemplateChild::default(),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            klass.set_template_from_resource("/dev/Cogitri/Health/ui/activity_row.ui");
            Self::bind_template_children(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self::Type>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for HealthActivityRow {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            let gesture_controller = gtk::GestureClick::new();
            gesture_controller.connect_pressed(glib::clone!(@weak obj => move |_,_,_,_| {
                let self_ = imp::HealthActivityRow::from_instance(&obj);
                self_.details_revealer.set_reveal_child(!self_.details_revealer.get_reveal_child());
            }));
        }
    }

    impl WidgetImpl for HealthActivityRow {}
    impl ListBoxRowImpl for HealthActivityRow {}

    impl HealthActivityRow {
        fn set_activity(&self, activity: Activity) {
            let activity_info = ActivityInfo::from(activity.activity_type);

            self.active_minutes_label.set_label(&i18n_f(
                "{} Minutes",
                &[&activity.duration.num_minutes().to_string()],
            ));
            self.activity_date_label
                .set_text(&format!("{}", activity.date.format("%x")));
            self.activity_type_label.set_label(&activity_info.name);

            if activity_info
                .available_data_points
                .contains(ActivityDataPoints::CALORIES_BURNED)
            {
                if let Some(calories_burned) = activity.calories_burned {
                    self.calories_burned_label
                        .set_label(&i18n_f("{} Calories", &[&calories_burned.to_string()]));
                }
            }

            if activity_info
                .available_data_points
                .contains(ActivityDataPoints::HEART_RATE)
            {
                if activity.heart_rate_avg.unwrap_or(0) != 0 {
                    self.heart_rate_average_label
                        .set_text(&activity.heart_rate_avg.unwrap().to_string());
                    self.heart_rate_average_row.set_visible(true);
                }
                if activity.heart_rate_max.unwrap_or(0) != 0 {
                    self.heart_rate_maximum_label
                        .set_text(&activity.heart_rate_max.unwrap().to_string());
                    self.heart_rate_maximum_row.set_visible(true);
                }
                if activity.heart_rate_min.unwrap_or(0) != 0 {
                    self.heart_rate_minimum_label
                        .set_text(&activity.heart_rate_min.unwrap().to_string());
                    self.heart_rate_minimum_row.set_visible(true);
                }
            }

            if activity_info
                .available_data_points
                .contains(ActivityDataPoints::DISTANCE)
            {
                if let Some(distance) = activity.distance {
                    self.distance_row.set_visible(true);

                    if self.settings.get_unitsystem() == Unitsystem::Imperial {
                        self.distance_label.set_label(&format!(
                            "{}",
                            distance.clone().into_format_args(meter, Abbreviation)
                        ));
                    } else {
                        self.distance_label.set_label(&format!(
                            "{}",
                            distance.clone().into_format_args(yard, Abbreviation)
                        ));
                    };
                }
            }
        }
    }
}

glib::wrapper! {
    pub struct HealthActivityRow(ObjectSubclass<imp::HealthActivityRow>)
        @extends gtk::Widget, gtk::ListBoxRow;
}

impl HealthActivityRow {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create HealthActivityRow")
    }
}
