use crate::properties_setter_getter;
use gdk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};
use gtk::{prelude::*, subclass::prelude::*};

mod imp {
    use super::*;
    use glib::subclass::{self, Property};
    use std::cell::RefCell;

    #[derive(Debug, CompositeTemplate)]
    pub struct HealthView {
        #[template_child]
        pub empty_icon: TemplateChild<gtk::Image>,
        #[template_child]
        pub goal_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub scrolled_window: TemplateChild<gtk::ScrolledWindow>,
        #[template_child]
        pub subtitle_empty_view_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub title_empty_view_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub title_label: TemplateChild<gtk::Label>,
        pub view_title: RefCell<String>,
    }

    static PROPERTIES: [Property; 4] = [
        Property("empty-subtitle", |name| {
            glib::ParamSpec::string(
                name,
                "empty-subtitle",
                "empty-subtitle",
                None,
                glib::ParamFlags::READWRITE,
            )
        }),
        Property("icon-name", |name| {
            glib::ParamSpec::string(
                name,
                "icon-name",
                "icon-name",
                None,
                glib::ParamFlags::READWRITE,
            )
        }),
        Property("title", |name| {
            glib::ParamSpec::string(name, "title", "title", None, glib::ParamFlags::READWRITE)
        }),
        Property("view-title", |name| {
            glib::ParamSpec::string(
                name,
                "view-title",
                "view-title",
                None,
                glib::ParamFlags::READWRITE,
            )
        }),
    ];

    impl ObjectSubclass for HealthView {
        const NAME: &'static str = "HealthView";
        type ParentType = gtk::Widget;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;
        type Type = super::HealthView;

        glib::object_subclass!();

        fn new() -> Self {
            Self {
                empty_icon: TemplateChild::default(),
                goal_label: TemplateChild::default(),
                scrolled_window: TemplateChild::default(),
                subtitle_empty_view_label: TemplateChild::default(),
                stack: TemplateChild::default(),
                title_empty_view_label: TemplateChild::default(),
                title_label: TemplateChild::default(),
                view_title: RefCell::new(String::new()),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            klass.set_layout_manager_type::<gtk::BinLayout>();
            klass.install_properties(&PROPERTIES);
            klass.set_template_from_resource("/dev/Cogitri/Health/ui/view.ui");
            Self::bind_template_children(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self::Type>) {
            obj.init_template();
        }
    }

    impl WidgetImpl for HealthView {}

    impl ObjectImpl for HealthView {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }

        fn dispose(&self, obj: &Self::Type) {
            while let Some(child) = obj.get_first_child() {
                child.unparent();
            }
        }

        fn set_property(&self, _obj: &Self::Type, id: usize, value: &glib::Value) {
            let prop = &PROPERTIES[id];

            match *prop {
                Property("empty-subtitle", ..) => self
                    .subtitle_empty_view_label
                    .set_label(value.get().unwrap().unwrap_or("")),
                Property("icon-name", ..) => {
                    self.empty_icon.set_property_icon_name(value.get().unwrap())
                }
                Property("title", ..) => self
                    .title_label
                    .set_label(value.get().unwrap().unwrap_or("")),
                Property("view-title", ..) => {
                    self.view_title
                        .replace(value.get().unwrap().unwrap_or("".to_string()));
                }
                _ => unimplemented!(),
            }
        }

        fn get_property(&self, _obj: &Self::Type, id: usize) -> glib::Value {
            let prop = &PROPERTIES[id];

            match *prop {
                Property("empty-subtitle", ..) => {
                    self.subtitle_empty_view_label.get_label().to_value()
                }
                Property("icon-name", ..) => self.empty_icon.get_icon_name().to_value(),
                Property("title", ..) => self.title_label.get_label().to_value(),
                Property("view-title", ..) => self.view_title.borrow().to_value(),
                _ => unimplemented!(),
            }
        }
    }
}

glib::wrapper! {
    pub struct HealthView(ObjectSubclass<imp::HealthView>)
        @extends gtk::Widget;
}

impl HealthView {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create HealthView")
    }

    pub fn get_goal_label(&self) -> gtk::Label {
        imp::HealthView::from_instance(self).goal_label.get()
    }

    pub fn get_stack(&self) -> gtk::Stack {
        imp::HealthView::from_instance(self).stack.get()
    }

    pub fn get_scrolled_window(&self) -> gtk::ScrolledWindow {
        imp::HealthView::from_instance(self).scrolled_window.get()
    }

    properties_setter_getter!("empty-title", String);
    properties_setter_getter!("icon-name", String);
    properties_setter_getter!("title", String);
    properties_setter_getter!("view-title", String);
}

unsafe impl<T: WidgetImpl> IsSubclassable<T> for HealthView {
    fn override_vfuncs(class: &mut glib::Class<Self>) {
        <gtk::Widget as IsSubclassable<T>>::override_vfuncs(class);
    }
}
