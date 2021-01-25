use crate::views::HealthView;
use gdk::subclass::prelude::*;

mod imp {
    use super::*;
    use crate::{
        core::HealthSettings,
        model::{Activity, HealthModelActivity},
        widgets::HealthActivityRow,
    };
    use chrono::Duration;
    use glib::{subclass, Cast};
    use gtk::{subclass::prelude::*, CompositeTemplate, WidgetExt};

    #[derive(Debug, CompositeTemplate)]
    pub struct HealthViewActivity {
        settings: HealthSettings,
        activity_model: HealthModelActivity,
        #[template_child]
        activies_list_box: TemplateChild<gtk::ListBox>,
        #[template_child]
        clamp: TemplateChild<libadwaita::Clamp>,
    }

    impl ObjectSubclass for HealthViewActivity {
        const NAME: &'static str = "HealthViewActivity";
        type ParentType = HealthView;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;
        type Type = super::HealthViewActivity;
        type Interfaces = ();

        glib::object_subclass!();

        fn new() -> Self {
            Self {
                settings: HealthSettings::new(),
                activity_model: HealthModelActivity::new(),
                activies_list_box: TemplateChild::default(),
                clamp: TemplateChild::default(),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            klass.set_template_from_resource("/dev/Cogitri/Health/ui/activity_view.ui");
            Self::bind_template_children(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self::Type>) {
            unsafe {
                // FIXME: This really shouldn't be necessary.
                obj.as_ref().upcast_ref::<HealthView>().init_template();
            }
        }
    }

    impl WidgetImpl for HealthViewActivity {}

    impl ObjectImpl for HealthViewActivity {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            let scrolled_window = obj.upcast_ref::<HealthView>().get_scrolled_window();
            scrolled_window.set_child(Some(&self.clamp.get()));
            scrolled_window.set_property_vscrollbar_policy(gtk::PolicyType::Automatic);

            self.activies_list_box
                .bind_model(Some(&self.activity_model), |o| {
                    let row = HealthActivityRow::new();
                    row.set_activity(o.clone().downcast::<Activity>().unwrap());
                    row.upcast()
                });
        }
    }

    impl HealthViewActivity {
        pub async fn update(&self, obj: &super::HealthViewActivity) {
            if let Err(e) = self.activity_model.reload(Duration::days(30)).await {
                glib::g_warning!(
                    crate::config::LOG_DOMAIN,
                    "Failed to reload activity data: {}",
                    e
                );
            }

            if self.activity_model.is_empty() {
                obj.upcast_ref::<HealthView>()
                    .get_stack()
                    .set_visible_child_name("data_page");
            }
        }
    }
}

glib::wrapper! {
    pub struct HealthViewActivity(ObjectSubclass<imp::HealthViewActivity>)
        @extends HealthView;
}

impl HealthViewActivity {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create HealthViewActivity")
    }

    pub async fn update(&self) {
        imp::HealthViewActivity::from_instance(self)
            .update(self)
            .await;
    }
}
