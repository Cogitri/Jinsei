use crate::views::HealthView;
use futures_util::FutureExt;
use gdk::subclass::prelude::*;

mod imp {
    use super::*;
    use crate::core::{i18n, i18n_f, HealthSettings};
    use crate::model::HealthGraphModelSteps;
    use crate::views::HealthGraphView;
    use glib::{subclass, Cast, ObjectExt};
    use gtk::subclass::prelude::*;

    #[derive(Debug)]
    pub struct HealthViewSteps {
        settings: HealthSettings,
        steps_graph_view: HealthGraphView,
        steps_graph_model: HealthGraphModelSteps,
    }

    impl ObjectSubclass for HealthViewSteps {
        const NAME: &'static str = "HealthViewSteps";
        type ParentType = gtk::Widget;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;
        type Type = super::HealthViewSteps;

        glib::object_subclass!();

        fn new() -> Self {
            let settings = HealthSettings::new();
            let steps_graph_model = HealthGraphModelSteps::new();
            let steps_graph_view = HealthGraphView::new();

            steps_graph_view.set_hover_func(Some(Box::new(|p| {
                return i18n_f(
                    "{} steps on {}",
                    &[&p.value.to_string(), &format!("{}", p.date.format("%x"))],
                );
            })));
            steps_graph_view.set_limit(Some(settings.get_user_stepgoal() as f32));
            steps_graph_view.set_limit_label(Some(i18n("Stepgoal")));

            Self {
                settings,
                steps_graph_view,
                steps_graph_model,
            }
        }
    }

    impl WidgetImpl for HealthViewSteps {}

    impl ObjectImpl for HealthViewSteps {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }
    }

    impl HealthViewSteps {
        pub async fn update(&mut self, obj: super::HealthViewSteps) {
            if let Err(e) = self.steps_graph_model.reload().await {
                glib::g_warning!(
                    crate::config::LOG_DOMAIN,
                    "Failed to reload step data: {}",
                    e
                );
            }

            let view = obj.upcast::<HealthView>();
            view.set_title(i18n_f(
                "Today's steps: {}",
                &[&self
                    .steps_graph_model
                    .get_today_step_count()
                    .unwrap_or(0)
                    .to_string()],
            ));
            match (self.steps_graph_model.get_today_step_count()) {}
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
