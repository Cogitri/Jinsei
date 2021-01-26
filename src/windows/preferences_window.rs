use gdk::subclass::prelude::ObjectSubclass;
use gtk::prelude::*;
use gtk::{gio, glib, CompositeTemplate};

static OPTIMAL_BMI: f32 = 22.5;

mod imp {
    use super::*;
    use crate::{
        core::{i18n, settings::Unitsystem, HealthSettings},
        widgets::{HealthBMILevelBar, HealthSyncListBox},
    };
    use glib::{
        clone,
        subclass::{self, Signal},
    };
    use gtk::subclass::prelude::*;
    use libadwaita::PreferencesRowExt;
    use uom::si::{
        f32::Mass,
        length::{centimeter, inch},
        mass::{kilogram, pound},
        u32::Length,
    };

    #[derive(Debug, CompositeTemplate)]
    #[template(resource = "/dev/Cogitri/Health/ui/preferences_window.ui")]
    pub struct HealthPreferencesWindow {
        pub parent_window: RefCell<Option<gtk::Window>>,
        pub settings: HealthSettings,

        #[template_child]
        pub height_actionrow: TemplateChild<libadwaita::ActionRow>,
        #[template_child]
        pub weightgoal_actionrow: TemplateChild<libadwaita::ActionRow>,
        #[template_child]
        pub age_spin_button: TemplateChild<gtk::SpinButton>,
        #[template_child]
        pub height_spin_button: TemplateChild<gtk::SpinButton>,
        #[template_child]
        pub stepgoal_spin_button: TemplateChild<gtk::set_optimal_weightgoal>,
        #[template_child]
        pub weightgoal_spin_button: TemplateChild<gtk::SpinButton>,
        #[template_child]
        pub unit_imperial_togglebutton: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub unit_metric_togglebutton: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub bmi_levelbar: TemplateChild<HealthBMILevelBar>,
    }

    fn get_spinbutton_value<T>(spin_button: &gtk::SpinButton) -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        spin_button
            .get_text()
            .unwrap()
            .as_str()
            .parse::<T>()
            .unwrap()
    }

    impl ObjectSubclass for HealthPreferencesWindow {
        const NAME: &'static str = "HealthPreferencesWindow";
        type ParentType = libadwaita::ApplicationWindow;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;
        type Type = super::HealthPreferencesWindow;
        type Interfaces = ();

        glib::object_subclass!();

        fn new() -> Self {
            Self {
                settings: HealthSettings::new(),
                bmi_levelbar: TemplateChild::default(),
                setup_first_page: TemplateChild::default(),
                setup_second_page: TemplateChild::default(),
                setup_third_page: TemplateChild::default(),
                setup_fourth_page: TemplateChild::default(),
                setup_done_button: TemplateChild::default(),
                setup_quit_button: TemplateChild::default(),
                setup_next_page_button: TemplateChild::default(),
                setup_previous_page_button: TemplateChild::default(),
                setup_right_stack: TemplateChild::default(),
                setup_left_stack: TemplateChild::default(),
                age_spin_button: TemplateChild::default(),
                height_spin_button: TemplateChild::default(),
                stepgoal_spin_button: TemplateChild::default(),
                weightgoal_spin_button: TemplateChild::default(),
                unit_metric_togglebutton: TemplateChild::default(),
                height_actionrow: TemplateChild::default(),
                weightgoal_actionrow: TemplateChild::default(),
                setup_carousel: TemplateChild::default(),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            HealthSyncListBox::static_type();

            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self::Type>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for HealthPreferencesWindow {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            self.stepgoal_spin_button.set_value(10000.0);
            let provider = gtk::CssProvider::new();
            provider.load_from_resource("/dev/Cogitri/Health/custom.css");
            gtk::StyleContext::add_provider_for_display(
                &obj.get_display(),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );

            self.connect_handlers(obj);
        }

        fn signals() -> &'static [Signal] {
            use once_cell::sync::Lazy;
            static SIGNALS: Lazy<Vec<Signal>> =
                Lazy::new(|| vec![Signal::builder("import-done", &[], glib::Type::Unit).build()]);

            SIGNALS.as_ref()
        }
    }

    impl WidgetImpl for HealthPreferencesWindow {}
    impl WindowImpl for HealthPreferencesWindow {}
    impl libadwaita::subclass::application_window::PreferencesWindowImpl for HealthPreferencesWindow {}
}

glib::wrapper! {
    pub struct HealthPreferencesWindow(ObjectSubclass<imp::HealthPreferencesWindow>)
        @extends gtk::Widget, gtk::Window, libadwaita::PreferencesWindow;
}

impl HealthPreferencesWindow {
    pub fn new() -> Self {
        glib::Object::new(&[()]).expect("Failed to create HealthPreferencesWindow")
    }
}
