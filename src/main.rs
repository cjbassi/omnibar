mod bar;
mod module;
mod modules;

use std::env::args;

use gio::prelude::*;
use gtk::prelude::*;
use platform_dirs::{AppDirs, AppUI};

const NAME: &str = env!("CARGO_PKG_NAME");
const AUTHOR: &str = "cjbassi";

fn build_ui(application: &gtk::Application) {
    let app_dirs = AppDirs::new(Some("waybar"), AppUI::CommandLine).unwrap();
    let style_file = app_dirs.config_dir.join("style.css");

    // Create a normal GTK window however you like
    let window = gtk::ApplicationWindow::new(application);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let screen = window.get_screen().unwrap();
    let css_provider = gtk::CssProvider::new();
    css_provider
        .load_from_path(style_file.to_str().unwrap())
        .unwrap();
    gtk::StyleContext::add_provider_for_screen(
        &screen,
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER,
    );

    bar::Bar::new(window);
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
