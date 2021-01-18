use crate::window::JinseiWindow;
use crate::{config, core::settings::JinseiSettings};
use gtk::{gio, glib};

mod imp {
    use std::unimplemented;

    use super::*;
    use glib::subclass;
    use gtk::{subclass::prelude::*, WidgetExt};

    #[derive(Debug)]
    pub struct JinseiApplication {
        pub settings: JinseiSettings,
        pub window: Option<JinseiWindow>,
    }

    impl ObjectSubclass for JinseiApplication {
        const NAME: &'static str = "JinseiApplication";
        type ParentType = gtk::Application;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;
        type Type = super::JinseiApplication;

        glib::object_subclass!();

        fn new() -> Self {
            Self {
                settings: JinseiSettings::new(),
                window: None,
            }
        }

        fn class_init(_klass: &mut Self::Class) {}

        fn instance_init(_obj: &glib::subclass::InitializingObject<Self::Type>) {}
    }

    impl ObjectImpl for JinseiApplication {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }
    }

    impl ApplicationImpl for JinseiApplication {
        fn activate(&self, application: &Self::Type) {
            self.parent_activate(application);

            if self.window.is_some() {
                return;
            } else if self.settings.get_did_initial_setup() {
                let window = JinseiWindow::new(application);
                window.show();
            //self.window = Some(window);
            } else {
                // create SetupWindow
                unimplemented!();
            }
        }
    }
    impl GtkApplicationImpl for JinseiApplication {}
}

glib::wrapper! {
    pub struct JinseiApplication(ObjectSubclass<imp::JinseiApplication>)
        @extends gio::Application, gtk::Application, @implements gio::ActionMap, gio::ActionGroup;
}

impl JinseiApplication {
    pub fn new() -> Self {
        glib::Object::new(&[
            ("application_id", &config::APPLICATION_ID.to_string()),
            ("flags", &gio::ApplicationFlags::FLAGS_NONE),
        ])
        .expect("Failed to create JinseiApplication")
    }
}
