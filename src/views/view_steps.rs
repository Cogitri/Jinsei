use crate::views::HealthView;
use futures_util::FutureExt;
use gdk::subclass::prelude::*;

mod imp {
    use super::*;
    use crate::core::{i18n, i18n_f, HealthSettings};
    use crate::model::HealthGraphModelSteps;
    use crate::views::HealthGraphView;
    use chrono::Duration;
    use glib::{subclass, Cast};
    use gtk::subclass::prelude::*;

    #[derive(Debug)]
    pub struct HealthViewSteps {
        settings: HealthSettings,
        steps_graph_view: Option<HealthGraphView>,
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

            Self {
                settings,
                steps_graph_view: None,
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
            if let Err(e) = self.steps_graph_model.reload(Duration::days(30)).await {
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

            let goal_label = view.get_goal_label();
            let streak_count = self
                .steps_graph_model
                .get_streak_count_today(self.settings.get_user_stepgoal());

            match streak_count {
                0 => {
                    let previous_streak = self
                        .steps_graph_model
                        .get_streak_count_yesterday(self.settings.get_user_stepgoal());
                    if previous_streak == 0 {
                        goal_label.set_text (&i18n("No streak yet. Reach your stepgoal for multiple days to start a streak!"));
                    } else {
                        goal_label.set_text (&i18n_f("You're on a streak for {} days. Reach your stepgoal today to continue it!", &[&previous_streak.to_string()]));
                    }
                }
                1 => goal_label.set_text(&i18n(
                    "You've reached your stepgoal today. Keep going to start a streak!",
                )),
                _ => goal_label.set_text(&i18n_f(
                    "You're on a streak for {} days. Good job!",
                    &[&streak_count.to_string()],
                )),
            }

            if let Some(view) = &self.steps_graph_view {
                view.set_points(self.steps_graph_model.to_points());
            } else if !self.steps_graph_model.is_empty() {
                let steps_graph_view = HealthGraphView::new();

                steps_graph_view.set_hover_func(Some(Box::new(|p| {
                    return i18n_f(
                        "{} steps on {}",
                        &[&p.value.to_string(), &format!("{}", p.date.format("%x"))],
                    );
                })));
                steps_graph_view.set_limit(Some(self.settings.get_user_stepgoal() as f32));
                steps_graph_view.set_limit_label(Some(i18n("Stepgoal")));

                view.get_scrolled_window()
                    .set_child(Some(&steps_graph_view));
                view.get_stack().set_visible_child_name("data_page");
            }
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
