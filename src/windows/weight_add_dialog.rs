use gdk::subclass::prelude::ObjectSubclass;
use gtk::prelude::*;
use gtk::{glib, CompositeTemplate};

mod imp {
    use std::cell::RefCell;

    use super::*;
    use crate::{
        core::{i18n, settings::Unitsystem, utils::get_spinbutton_value, HealthSettings},
        widgets::HealthBMILevelBar,
    };
    use glib::{
        clone,
        subclass::{self, Signal},
    };
    use gtk::subclass::prelude::*;
    use libadwaita::PreferencesRowExt;
    use uom::si::{
        f32::Mass,
        length::{centimeter, inch},
        mass::{kilogram, pound},
        u32::Length,
    };

    #[derive(Debug, CompositeTemplate)]
    #[template(resource = "/dev/Cogitri/Health/ui/weight_add_dialog.ui")]
    pub struct HealthWeightAddDialog {
        pub db: HealthDatabase,
        pub settings: HealthSettings,

        #[template_child]
        pub date_selector: TemplateChild<HealthDateSelector>,
        #[template_child]
        pub weight_spin_button: TemplateChild<gtk::SpinButton>,
    }

    impl ObjectSubclass for HealthWeightAddDialog {
        const NAME: &'static str = "HealthWeightAddDialog";
        type ParentType = libadwaita::PreferencesWindow;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;
        type Type = super::HealthWeightAddDialog;
        type Interfaces = ();

        glib::object_subclass!();

        fn new() -> Self {
            Self {
                db: HealthDatabase,
                settings: HealthSettings::new(),
                date_selector: TemplateChild::default(),
                weight_spin_button: TemplateChild::default(),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self::Type>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for HealthWeightAddDialog {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            if self.settings.get_unitsystem() == Unitsystem::Metric {
                self.unit_metric_togglebutton.set_active(true);
                self.height_actionrow
                    .set_title(Some(&i18n("Height in centimeters")));
                self.weightgoal_actionrow
                    .set_title(Some(&i18n("Weightgoal in KG")));
                self.height_spin_button
                    .set_value(self.settings.get_user_height().get::<centimeter>() as f64);
                self.weightgoal_spin_button
                    .set_value(self.settings.get_user_weightgoal().get::<kilogram>() as f64);
            } else {
                self.unit_metric_togglebutton.set_active(true);
                self.height_actionrow
                    .set_title(Some(&i18n("Height in inch")));
                self.weightgoal_actionrow
                    .set_title(Some(&i18n("Weightgoal in pounds")));
                self.height_spin_button
                    .set_value(self.settings.get_user_height().get::<inch>() as f64);
                self.weightgoal_spin_button
                    .set_value(self.settings.get_user_weightgoal().get::<pound>() as f64);
            }

            self.stepgoal_spin_button
                .set_value(self.settings.get_user_stepgoal() as f64);
            self.age_spin_button
                .set_value(self.settings.get_user_age() as f64);

            self.connect_handlers(obj);
        }

        fn signals() -> &'static [Signal] {
            use once_cell::sync::Lazy;
            static SIGNALS: Lazy<Vec<Signal>> =
                Lazy::new(|| vec![Signal::builder("import-done", &[], glib::Type::Unit).build()]);

            SIGNALS.as_ref()
        }
    }

    impl WidgetImpl for HealthWeightAddDialog {}
    impl WindowImpl for HealthWeightAddDialog {}
    impl libadwaita::subclass::window::AdwWindowImpl for HealthWeightAddDialog {}
    impl libadwaita::subclass::preferences_window::PreferencesWindowImpl for HealthWeightAddDialog {}

    impl HealthWeightAddDialog {
        fn connect_handlers(&self, obj: &super::HealthWeightAddDialog) {
            self.age_spin_button
                .connect_changed(clone!(@weak obj => move |_| {
                    let self_ = imp::HealthWeightAddDialog::from_instance(&obj);
                    let val = get_spinbutton_value::<u32>(&self_.age_spin_button);
                    if val != 0 {
                        self_.settings.set_user_age(val);
                    }
                }));

            self.stepgoal_spin_button
                .connect_changed(clone!(@weak obj => move |_| {
                    let self_ = imp::HealthWeightAddDialog::from_instance(&obj);
                    let val = get_spinbutton_value::<u32>(&self_.stepgoal_spin_button);
                    if val != 0 {
                        self_.settings.set_user_stepgoal(val);
                    }
                }));

            self.weightgoal_spin_button
                .connect_changed(clone!(@weak obj => move |_| {
                    let self_ = imp::HealthWeightAddDialog::from_instance(&obj);
                    let val = get_spinbutton_value::<f32>(&self_.weightgoal_spin_button);
                    if val != 0.0 {
                        let weight = if self_.unit_metric_togglebutton.get_active() {
                            Mass::new::<kilogram>(val)
                        } else {
                            Mass::new::<pound>(val)
                        };

                        self_.settings.set_user_weightgoal(weight);
                    }
                }));

            self.height_spin_button
                .connect_changed(clone!(@weak obj => move |_| {
                    let self_ = imp::HealthWeightAddDialog::from_instance(&obj);
                    let val = get_spinbutton_value::<u32>(&self_.height_spin_button);
                    if val != 0 {
                        let height = if self_.unit_metric_togglebutton.get_active() {
                            Length::new::<centimeter>(val)
                        } else {
                            Length::new::<inch>(val)
                        };

                        self_.settings.set_user_height(height);
                    }
                }));

            self.unit_metric_togglebutton.connect_toggled(clone!(@weak obj => move |btn| {
                let self_ = imp::HealthWeightAddDialog::from_instance(&obj);
                if btn.get_active() {
                    self_.settings.set_unitsystem(Unitsystem::Metric);
                    self_.bmi_levelbar.set_unitsystem(Unitsystem::Metric);
                    self_.height_actionrow
                    .set_title(Some(&i18n("Height in centimeters")));
                    self_.weightgoal_actionrow
                        .set_title(Some(&i18n("Weightgoal in KG")));
                    self_.height_spin_button
                        .set_value(Length::new::<inch>(get_spinbutton_value(&self_.height_spin_button)).get::<centimeter>() as f64);
                    self_.weightgoal_spin_button
                        .set_value(Mass::new::<pound>(get_spinbutton_value(&self_.height_spin_button)).get::<kilogram>() as f64);
                } else {
                    self_.settings.set_unitsystem(Unitsystem::Imperial);
                    self_.bmi_levelbar.set_unitsystem(Unitsystem::Imperial);
                    self_.height_actionrow
                    .set_title(Some(&i18n("Height in inch")));
                    self_.weightgoal_actionrow
                        .set_title(Some(&i18n("Weightgoal in pounds")));
                    self_.height_spin_button
                        .set_value(Length::new::<centimeter>(get_spinbutton_value(&self_.height_spin_button)).get::<inch>() as f64);
                    self_.weightgoal_spin_button
                        .set_value(Mass::new::<kilogram>(get_spinbutton_value(&self_.height_spin_button)).get::<pound>() as f64);
                }
            }));
        }
    }
}

glib::wrapper! {
    pub struct HealthWeightAddDialog(ObjectSubclass<imp::HealthWeightAddDialog>)
        @extends gtk::Widget, gtk::Window, libadwaita::PreferencesWindow;
}

impl HealthWeightAddDialog {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create HealthWeightAddDialog")
    }
}
