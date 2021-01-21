use crate::windows::HealthWindow;
use crate::{config, core::settings::HealthSettings};
use gtk::{gio, glib};

mod imp {
    use std::{cell::RefCell, unimplemented};

    use super::*;
    use glib::{g_warning, subclass};
    use gtk::{subclass::prelude::*, WidgetExt};

    #[derive(Debug)]
    pub struct HealthApplication {
        pub settings: HealthSettings,
        pub window: RefCell<glib::WeakRef<HealthWindow>>,
    }

    impl ObjectSubclass for HealthApplication {
        const NAME: &'static str = "HealthApplication";
        type ParentType = gtk::Application;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;
        type Type = super::HealthApplication;

        glib::object_subclass!();

        fn new() -> Self {
            Self {
                settings: HealthSettings::new(),
                window: RefCell::new(glib::WeakRef::new()),
            }
        }

        fn class_init(_klass: &mut Self::Class) {}

        fn instance_init(_obj: &glib::subclass::InitializingObject<Self::Type>) {}
    }

    impl ObjectImpl for HealthApplication {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }
    }

    impl ApplicationImpl for HealthApplication {
        fn activate(&self, application: &Self::Type) {
            self.parent_activate(application);

            if self.window.borrow().upgrade().is_some() {
                return;
            } else if self.settings.get_did_initial_setup() {
                let window = HealthWindow::new(application);
                window.show();
                self.window.replace(glib::ObjectExt::downgrade(&window));
            } else {
                // create SetupWindow
                unimplemented!();
            }
        }

        fn startup(&self, application: &Self::Type) {
            self.parent_startup(application);
            libadwaita::init();

            if let Some(true) = gtk::Settings::get_default()
                .and_then(|s| s.get_property_gtk_theme_name())
                .map(|s| s.as_str().contains("-dark"))
            {
                g_warning! (config::LOG_DOMAIN, "Using -dark themes (such as Adwaita-dark) is unsupported. Please use your theme in dark-mode instead (e.g. Adwaita:dark instead of Adwaita-dark)");
            }
        }
    }
    impl GtkApplicationImpl for HealthApplication {}
}

glib::wrapper! {
    pub struct HealthApplication(ObjectSubclass<imp::HealthApplication>)
        @extends gio::Application, gtk::Application, @implements gio::ActionMap, gio::ActionGroup;
}

impl HealthApplication {
    pub fn new() -> Self {
        glib::Object::new(&[
            ("application_id", &config::APPLICATION_ID.to_string()),
            ("flags", &gio::ApplicationFlags::FLAGS_NONE),
        ])
        .expect("Failed to create HealthApplication")
    }
}
