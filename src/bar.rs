use gtk::prelude::*;
use gtk::{Box as GtkBox, BoxExt, Orientation};
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

        let gtk_box = GtkBox::new(Orientation::Horizontal, 0);
        let gtk_box_left = GtkBox::new(Orientation::Horizontal, 0);
        let gtk_box_center = GtkBox::new(Orientation::Horizontal, 0);
        let gtk_box_right = GtkBox::new(Orientation::Horizontal, 0);

        gtk_box.pack_start(&gtk_box_left, false, false, 0);
        gtk_box.set_center_widget(Some(&gtk_box_center));
        gtk_box.pack_end(&gtk_box_right, false, false, 0);

        let clock = Box::new(Clock::new());

        gtk_box_center.add(&clock.get_label());

        window.add(&gtk_box);
        window.set_border_width(12);
        window.show_all();

        Bar {
            modules_left: Default::default(),
            modules_center: vec![clock],
            modules_right: Default::default(),
        }
    }
}
