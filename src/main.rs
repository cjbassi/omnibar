mod bar;
mod module;
mod modules;

use std::env::args;

use gio::prelude::*;

const NAME: &str = env!("CARGO_PKG_NAME");
const AUTHOR: &str = "cjbassi";

fn build_ui(application: &gtk::Application) {
    bar::Bar::new(application);
}

fn main() {
    let gtk_name = format!("com.github.{}.{}", AUTHOR, NAME);

    let application = gtk::Application::new(Some(&gtk_name), Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
