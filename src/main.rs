mod bar;
mod module;
mod modules;

use std::env::args;

use gio::prelude::*;

const NAME: &str = env!("CARGO_PKG_NAME");
const AUTHOR: &str = "cjbassi";

fn activate(application: &gtk::Application) {
    bar::Bar::new(application);
}

fn main() {
    let application = gtk::Application::new(
        Some(&format!("com.{}.{}", AUTHOR, NAME)),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        activate(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
