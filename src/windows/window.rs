use gtk::prelude::*;
use gtk::{gio, glib, CompositeTemplate};

mod imp {
    use super::*;
    use crate::core::{HealthDatabase, HealthSettings};
    use crate::views::{HealthView, HealthViewSteps};
    use glib::subclass;
    use gtk::subclass::prelude::*;
    use std::cell::RefCell;
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum ViewMode {
        ACTIVITIES,
        STEPS,
        WEIGHT,
    }

    #[derive(Debug, CompositeTemplate)]
    pub struct HealthWindow {
        pub current_view: RefCell<ViewMode>,
        pub db: HealthDatabase,
        pub settings: HealthSettings,
        pub views: HashMap<ViewMode, HealthView>,

        #[template_child]
        pub error_bar: TemplateChild<gtk::InfoBar>,
        #[template_child]
        pub error_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub primary_menu_popover: TemplateChild<gtk::Popover>,
        #[template_child]
        pub stack: TemplateChild<gtk::Stack>,
    }

    impl ObjectSubclass for HealthWindow {
        const NAME: &'static str = "HealthWindow";
        type ParentType = libadwaita::ApplicationWindow;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;
        type Type = super::HealthWindow;

        glib::object_subclass!();

        fn new() -> Self {
            let mut views = HashMap::new();
            views.insert(ViewMode::STEPS, HealthViewSteps::new().upcast());

            Self {
                current_view: RefCell::new(ViewMode::STEPS),
                db: HealthDatabase::new().unwrap(),
                settings: HealthSettings::new(),
                views,
                error_bar: TemplateChild::default(),
                error_label: TemplateChild::default(),
                primary_menu_popover: TemplateChild::default(),
                stack: TemplateChild::default(),
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

            if crate::config::APPLICATION_ID.ends_with("Devel") {
                obj.get_style_context().add_class("devel");

                // When in devel mode our application ID is different so we have to manually add the icon theme
                gtk::IconTheme::get_for_display(&obj.get_display())
                    .map(|i| i.add_resource_path("/dev/Cogitri/Health/icons"));
            }

            let provider = gtk::CssProvider::new();
            provider.load_from_resource("/dev/Cogitri/Health/custom.css");
            gtk::StyleContext::add_provider_for_display(
                &obj.get_display(),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );

            for (_, view) in &self.views {
                let page = self.stack.add_titled(
                    view,
                    view.get_name().map(|s| s.to_string()).as_deref(),
                    &view.get_view_title(),
                );
                page.set_icon_name(&view.get_icon_name());
            }
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
