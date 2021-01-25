use crate::views::HealthView;
use gdk::subclass::prelude::*;

mod imp {
    use super::*;
    use crate::core::{i18n, i18n_f, settings::Unitsystem, HealthSettings};
    use crate::model::HealthGraphModelWeight;
    use crate::views::HealthGraphView;
    use chrono::Duration;
    use glib::{subclass, Cast};
    use gtk::{subclass::prelude::*, CompositeTemplate, WidgetExt};
    use std::cell::RefCell;
    use uom::si::{
        length::meter,
        mass::{kilogram, pound},
    };

    #[derive(Debug, CompositeTemplate)]
    pub struct HealthViewWeight {
        settings: HealthSettings,
        weight_graph_view: Option<HealthGraphView>,
        weight_graph_model: RefCell<HealthGraphModelWeight>,
    }

    impl ObjectSubclass for HealthViewWeight {
        const NAME: &'static str = "HealthViewWeight";
        type ParentType = HealthView;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;
        type Type = super::HealthViewWeight;
        type Interfaces = ();

        glib::object_subclass!();

        fn new() -> Self {
            let settings = HealthSettings::new();
            let weight_graph_model = HealthGraphModelWeight::new();

            Self {
                settings,
                weight_graph_view: None,
                weight_graph_model: RefCell::new(weight_graph_model),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            klass.set_template_from_resource("/dev/Cogitri/Health/ui/weight_view.ui");
            Self::bind_template_children(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self::Type>) {
            unsafe {
                // FIXME: This really shouldn't be necessary.
                obj.as_ref().upcast_ref::<HealthView>().init_template();
            }
        }
    }

    impl WidgetImpl for HealthViewWeight {}

    impl ObjectImpl for HealthViewWeight {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }
    }

    impl HealthViewWeight {
        fn update_weightgoal_label(&self, obj: &crate::views::HealthViewWeight) {
            let weightgoal = self.settings.get_user_weightgoal();
            let unitsystem = self.settings.get_unitsystem();
            let (weight_value, translation) = if unitsystem == Unitsystem::Imperial {
                (weightgoal.get::<pound>(), i18n("pounds"))
            } else {
                (weightgoal.get::<kilogram>(), i18n("kilogram"))
            };
            let model = self.weight_graph_model.borrow();
            let goal_label = obj.upcast_ref::<HealthView>().get_goal_label();

            if weight_value > 0.1 && model.is_empty() {
                /* TRANSLATORS: the second {} format strings is the weight unit, e.g. kilogram */
                goal_label.set_text (&i18n_f("Your weight goal is {} {}. Add a first weight measurement to see how close you are to reaching it.",&[&weight_value.to_string(), &translation]));
            } else if weight_value > 0.1 && !model.is_empty() {
                if model.get_last_weight().unwrap() == weightgoal {
                    goal_label.set_text(&i18n("You've reached your weightgoal. Great job!"));
                }

                let unit_diff = model.get_last_weight().unwrap() - weightgoal;
                let diff = if unitsystem == Unitsystem::Imperial {
                    unit_diff.get::<pound>()
                } else {
                    unit_diff.get::<kilogram>()
                };
                /* TRANSLATORS: the second & fourth {} format strings is the weight unit, e.g. kilogram */
                goal_label.set_text(&i18n_f(
                    "{} {} left to reach your weightgoal of {} {}",
                    &[
                        &diff.to_string(),
                        &translation,
                        &weight_value.to_string(),
                        &translation,
                    ],
                ));
            } else {
                goal_label.set_text(&i18n(
                    "No weightgoal set yet. You can set it in Health's preferences.",
                ));
            }
        }

        fn get_bmi(&self) -> String {
            if let Some(last_weight) = self.weight_graph_model.borrow().get_last_weight() {
                let height = self.settings.get_user_height().get::<meter>();
                let bmi = last_weight.get::<kilogram>() as f32 / (height * height) as f32;
                format!("{bmi:.2}", bmi = bmi)
            } else {
                i18n("Unknown BMI")
            }
        }

        pub async fn update(&self, obj: &super::HealthViewWeight) {
            let mut weight_graph_model = self.weight_graph_model.borrow_mut();
            if let Err(e) = weight_graph_model.reload(Duration::days(30)).await {
                glib::g_warning!(
                    crate::config::LOG_DOMAIN,
                    "Failed to reload weight data: {}",
                    e
                );
            }

            let view = obj.upcast_ref::<HealthView>();
            view.set_title(i18n_f("Current BMI: {}", &[&self.get_bmi()]));
            self.update_weightgoal_label(obj);

            if let Some(view) = &self.weight_graph_view {
                view.set_points(weight_graph_model.to_points());
            } else if !weight_graph_model.is_empty() {
                let weight_graph_view = HealthGraphView::new();
                let settings = self.settings.clone();
                weight_graph_view.set_hover_func(Some(Box::new(move |p| {
                    let unit = if settings.get_unitsystem() == Unitsystem::Imperial {
                        "PB"
                    } else {
                        "KG"
                    };

                    return i18n_f(
                        "{} {} on {}",
                        &[
                            &p.value.to_string(),
                            unit,
                            &format!("{}", p.date.format("%x")),
                        ],
                    );
                })));
                let unitgoal = self.settings.get_user_weightgoal();
                let weightgoal = if self.settings.get_unitsystem() == Unitsystem::Imperial {
                    unitgoal.get::<pound>()
                } else {
                    unitgoal.get::<kilogram>()
                };
                weight_graph_view.set_limit(Some(weightgoal));
                weight_graph_view.set_limit_label(Some(i18n("Weightgoal")));

                view.get_scrolled_window()
                    .set_child(Some(&weight_graph_view));
                view.get_stack().set_visible_child_name("data_page");
            }
        }
    }
}

glib::wrapper! {
    pub struct HealthViewWeight(ObjectSubclass<imp::HealthViewWeight>)
        @extends HealthView;
}

impl HealthViewWeight {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create HealthViewWeight")
    }

    pub async fn update(&self) {
        imp::HealthViewWeight::from_instance(self)
            .update(self)
            .await;
    }
}
