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

    #[derive(Debug, CompositeTemplate)]
    pub struct HealthActivityTypeRow {
        #[template_child]
        pub activity_type_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub selected_image: TemplateChild<gtk::Image>,
    }

    impl ObjectSubclass for HealthActivityTypeRow {
        const NAME: &'static str = "HealthActivityTypeRow";
        type ParentType = gtk::ListBoxRow;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;
        type Type = super::HealthActivityTypeRow;

        glib::object_subclass!();

        fn new() -> Self {
            Self {
                activity_type_label: TemplateChild::default(),
                selected_image: TemplateChild::default(),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            klass.set_template_from_resource("/dev/Cogitri/Health/ui/activity_type_row.ui");
            Self::bind_template_children(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self::Type>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for HealthActivityTypeRow {}
    impl WidgetImpl for HealthActivityTypeRow {}
    impl ListBoxRowImpl for HealthActivityTypeRow {}

    impl HealthActivityTypeRow {
        pub fn set_selected(&self, value: bool) {
            self.selected_image.set_visible(value);
        }

        pub fn set_label(&self, value: &str) {
            self.activity_type_label.set_text(value);
        }
    }

}

glib::wrapper! {
    pub struct HealthActivityTypeRow(ObjectSubclass<imp::HealthActivityTypeRow>)
        @extends gtk::Widget, gtk::ListBoxRow;
}

impl HealthActivityTypeRow  {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create HealthActivityTypeRow")
    }
}
