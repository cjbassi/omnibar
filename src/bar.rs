use gtk::prelude::*;
use gtk_layer_shell_rs as gtk_layer_shell;

use crate::module::Module;
use crate::modules::Clock;

pub struct Bar {
    modules_left: Vec<Box<dyn Module>>,
    modules_center: Vec<Box<dyn Module>>,
    modules_right: Vec<Box<dyn Module>>,
}

impl Bar {
    pub fn new(application: &gtk::Application) -> Self {
        // Create a normal GTK window however you like
        let window = gtk::ApplicationWindow::new(application);
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        // Before the window is first realized, set it up to be a layer surface
        gtk_layer_shell::init_for_window(&window);

        // Order below normal windows
        gtk_layer_shell::set_layer(&window, gtk_layer_shell::Layer::Bottom);

        // Push other windows out of the way
        gtk_layer_shell::auto_exclusive_zone_enable(&window);

        // Anchors are if the window is pinned to each edge of the output
        gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Left, true);
        gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Right, true);
        gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Top, false);
        gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Bottom, true);

        let clock = Box::new(Clock::new());

        window.add(&clock.get_label());
        window.set_border_width(12);
        window.show_all();

        Bar {
            modules_left: Default::default(),
            modules_center: vec![clock],
            modules_right: Default::default(),
        }
    }
}
