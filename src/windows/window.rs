use gtk::prelude::*;
use gtk::{gio, glib, CompositeTemplate};

mod imp {
    use super::*;
    use glib::subclass;
    use gtk::subclass::prelude::*;

    #[derive(Debug, CompositeTemplate)]
    pub struct HealthWindow {
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
    }

    impl ObjectSubclass for HealthWindow {
        const NAME: &'static str = "HealthWindow";
        type ParentType = libadwaita::ApplicationWindow;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;
        type Type = super::HealthWindow;

        glib::object_subclass!();

        fn new() -> Self {
            Self {
                label: TemplateChild::default(),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            klass.set_template_from_resource("/dev/Cogitri/Health/ui/window.ui");
            Self::bind_template_children(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self::Type>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for HealthWindow {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }
    }

    impl WidgetImpl for HealthWindow {}
    impl WindowImpl for HealthWindow {}
    impl ApplicationWindowImpl for HealthWindow {}
    impl libadwaita::subclass::application_window::AdwApplicationWindowImpl for HealthWindow {}
}

glib::wrapper! {
    pub struct HealthWindow(ObjectSubclass<imp::HealthWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, libadwaita::ApplicationWindow, @implements gio::ActionMap, gio::ActionGroup;
}

impl HealthWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(&[("application", app)]).expect("Failed to create HealthWindow")
    }
}
