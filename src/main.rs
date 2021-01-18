use gettextrs::*;
use gtk::prelude::*;
use gtk::prelude::{ApplicationExt, ApplicationExtManual};

mod config;
mod core;
mod window;
use crate::window::JinseiWindow;

fn main() {
    gtk::init().expect("Failed to initialize GTK.");
    libadwaita::init();

    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain("Jinsei", config::LOCALEDIR);
    textdomain("Jinsei");

    let res = gio::Resource::load(config::PKGDATADIR.to_owned() + "/jinsei.gresource")
        .expect("Could not load resources");
    gio::resources_register(&res);

    let app = gtk::Application::new(Some("dev.Cogitri.Jinsei"), Default::default()).unwrap();
    app.connect_activate(move |app| {
        let window = JinseiWindow::new(app);
        window.show();
    });

    let ret = app.run(&std::env::args().collect::<Vec<_>>());
    std::process::exit(ret);
}
