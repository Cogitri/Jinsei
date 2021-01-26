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
    #[template(resource = "/dev/Cogitri/Health/ui/activity_view.ui")]
    pub struct HealthViewActivity {
        settings: HealthSettings,
        activity_model: HealthModelActivity,
        activities_list_box: gtk::ListBox,
        clamp: libadwaita::Clamp,
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
            let activities_list_box = gtk::ListBoxBuilder::new()
                .can_focus(false)
                .selection_mode(gtk::SelectionMode::None)
                .build();
            activities_list_box.add_css_class("content");
            let clamp_builder = libadwaita::ClampBuilder::new()
                .maximum_size(800)
                .tightening_threshold(600)
                .valign(gtk::Align::Center)
                .vexpand(true)
                .hexpand(true)
                .margin_end(6)
                .margin_bottom(6)
                .margin_start(6)
                .margin_top(6)
                .child(&activities_list_box);

            Self {
                settings: HealthSettings::new(),
                activity_model: HealthModelActivity::new(),
                activities_list_box,
                clamp: clamp_builder.build(),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
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
            scrolled_window.set_child(Some(&self.clamp));
            scrolled_window.set_property_vscrollbar_policy(gtk::PolicyType::Automatic);

            self.activities_list_box
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

            if !self.activity_model.is_empty() {
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