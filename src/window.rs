use gtk::prelude::*;
use gtk::{gio, glib, CompositeTemplate};

mod imp {
    use super::*;
    use glib::subclass;
    use gtk::subclass::prelude::*;

    #[derive(Debug, CompositeTemplate)]
    pub struct JinseiWindow {
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
    }

    impl ObjectSubclass for JinseiWindow {
        const NAME: &'static str = "JinseiWindow";
        type ParentType = libadwaita::ApplicationWindow;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;
        type Type = super::JinseiWindow;

        glib::object_subclass!();

        fn new() -> Self {
            Self {
                label: TemplateChild::default(),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            klass.set_template_from_resource("/dev/Cogitri/Jinsei/ui/window.ui");
            Self::bind_template_children(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self::Type>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for JinseiWindow {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }
    }

    impl WidgetImpl for JinseiWindow {}
    impl WindowImpl for JinseiWindow {}
    impl ApplicationWindowImpl for JinseiWindow {}
    impl libadwaita::subclass::application_window::AdwApplicationWindowImpl for JinseiWindow {}
}

glib::wrapper! {
    pub struct JinseiWindow(ObjectSubclass<imp::JinseiWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, libadwaita::ApplicationWindow, @implements gio::ActionMap, gio::ActionGroup;
}

impl JinseiWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(&[("application", app)]).expect("Failed to create JinseiWindow")
    }
}
