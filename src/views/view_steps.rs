use gdk::subclass::prelude::*;
use crate::views::HealthView;

mod imp {
    use super::*;
    use glib::subclass;
    use gtk::subclass::prelude::*;
    use crate::core::HealthSettings;
    use crate::model::HealthGraphModelSteps;
    use crate::views::HealthGraphView;

    #[derive(Debug)]
    pub struct HealthViewSteps {
        settings: HealthSettings,
        steps_graph_view: Option<HealthGraphView>,
        steps_graph_model: Option<HealthGraphModelSteps>,
    }

    impl ObjectSubclass for HealthViewSteps {
        const NAME: &'static str = "HealthViewSteps";
        type ParentType = gtk::Widget;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;
        type Type = super::HealthViewSteps;

        glib::object_subclass!();

        fn new() -> Self {
            Self {
                settings: HealthSettings::new(),
                steps_graph_view: None,
                steps_graph_model: None,
            }
        }

    }

    impl WidgetImpl for HealthViewSteps {}

    impl ObjectImpl for HealthViewSteps {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }
    }
}

glib::wrapper! {
    pub struct HealthViewSteps(ObjectSubclass<imp::HealthViewSteps>)
        @extends HealthView;
}

impl HealthViewSteps {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create HealthViewSteps")
    }
}