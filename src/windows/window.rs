use crate::views::{HealthViewSteps, HealthViewWeight};
use gdk::subclass::prelude::ObjectSubclass;
use gtk::prelude::*;
use gtk::{gio, glib, CompositeTemplate};

mod imp {
    use super::*;
    use crate::core::{HealthDatabase, HealthSettings};
    use crate::views::HealthView;
    use glib::{clone, signal::Inhibit, subclass, SourceId};
    use gtk::subclass::prelude::*;
    use std::cell::RefCell;
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum ViewMode {
        ACTIVITIES,
        STEPS,
        WEIGHT,
    }

    #[derive(Debug)]
    pub struct HealthWindowMut {
        current_height: i32,
        current_width: i32,
        current_view: ViewMode,
        sync_source_id: Option<SourceId>,
    }

    #[derive(Debug, CompositeTemplate)]
    pub struct HealthWindow {
        pub db: HealthDatabase,
        pub inner: RefCell<HealthWindowMut>,
        pub settings: HealthSettings,
        pub views: HashMap<ViewMode, HealthView>,

        pub add_data_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub error_infobar: TemplateChild<gtk::InfoBar>,
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
            views.insert(ViewMode::WEIGHT, HealthViewWeight::new().upcast());
            views.insert(ViewMode::STEPS, HealthViewSteps::new().upcast());

            Self {
                db: HealthDatabase::new().unwrap(),
                inner: RefCell::new(HealthWindowMut {
                    current_height: 0,
                    current_width: 0,
                    current_view: ViewMode::STEPS,
                    sync_source_id: None,
                }),
                settings: HealthSettings::new(),
                views,
                add_data_button: TemplateChild::default(),
                error_infobar: TemplateChild::default(),
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
                    &view.get_view_title().unwrap(),
                );
                page.set_icon_name(&view.get_icon_name().unwrap());
            }

            self.connect_handlers(obj);
        }
    }

    impl HealthWindow {
        pub fn show_error(&self, err_msg: &str) {
            glib::g_warning!(crate::config::LOG_DOMAIN, "{}", err_msg);
            self.error_label.set_text(err_msg);
            self.error_infobar.set_revealed(true);
        }

        fn connect_handlers(&self, obj: &super::HealthWindow) {
            self.error_infobar.connect_response(|bar, response| {
                if response == gtk::ResponseType::Close {
                    bar.set_revealed(false);
                }
            });
            self.stack
                .connect_property_visible_child_notify(clone!(@weak obj => move |s| {
                    let child_name = s.get_visible_child_name().map(|s| s.to_string());
                    let self_ = HealthWindow::from_instance(&obj);

                    if child_name == self_.views.get(&ViewMode::STEPS).and_then(|s| s.get_name()).map(|s| s.to_string()) {
                        self_.inner.borrow_mut().current_view = ViewMode::STEPS;
                    } else if child_name == self_.views.get(&ViewMode::WEIGHT).and_then(|s| s.get_name()).map(|s| s.to_string()) {
                        self_.inner.borrow_mut().current_view = ViewMode::WEIGHT;
                    }
                }));
            /*
            self.add_data_button
            .connect_clicked(clone!(@weak obj => move |_| {
                let self_ = HealthWindow::from_instance(&obj);

                todo!();
            }));
            */

            obj.connect_property_default_height_notify(move |w| {
                HealthWindow::from_instance(w)
                    .inner
                    .borrow_mut()
                    .current_height = w.get_property_default_height();
            });
            obj.connect_property_default_width_notify(move |w| {
                HealthWindow::from_instance(w)
                    .inner
                    .borrow_mut()
                    .current_width = w.get_property_default_width();
            });
            obj.connect_close_request(|w| {
                let self_ = HealthWindow::from_instance(w);
                let mut inner = self_.inner.borrow_mut();

                self_
                    .settings
                    .set_window_is_maximized(w.get_property_maximized());
                self_.settings.set_window_height(inner.current_height);
                self_.settings.set_window_width(inner.current_width);

                if let Some(source_id) = inner.sync_source_id.take() {
                    glib::source_remove(source_id);
                }

                Inhibit(false)
            });
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

    pub fn update(&self) {
        for (mode, view) in &imp::HealthWindow::from_instance(self).views {
            match mode {
                imp::ViewMode::STEPS => {
                    let v = view.downcast_ref::<HealthViewSteps>().unwrap();
                    let fut = v.update();
                    //FIXME: don't block here!
                    glib::MainContext::default().block_on(fut);
                }
                imp::ViewMode::WEIGHT => {
                    let v = view.downcast_ref::<HealthViewWeight>().unwrap();
                    let fut = v.update();
                    //FIXME: don't block here!
                    glib::MainContext::default().block_on(fut);
                }
                _ => unimplemented!(),
            }
        }
    }
}
