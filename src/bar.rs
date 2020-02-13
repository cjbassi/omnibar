use std::thread;

use gtk::prelude::*;
use gtk::{Box as GtkBox, BoxExt, Orientation};
use gtk_layer_shell_rs as gtk_layer_shell;

use crate::module::Module;
use crate::modules::*;

pub struct Bar {}

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

        let mut battery = Battery::new("{percent:2.0}");
        let mut clock1 = Clock::new("%a");
        let mut cpu = Cpu::new("{percent:2.0}");
        let mut disk = Disk::new("{percent:2.0}", "/");
        let mut memory = Memory::new("{percent:2.0}");
        let mut pulseaudio = PulseAudio::new("{percent:2.0}");
        let mut sway = Sway::new();

        battery.update();
        clock1.update();
        cpu.update();
        disk.update();
        memory.update();
        pulseaudio.update();
        sway.update();

        gtk_box_left.pack_start(sway.get_label(), false, false, 0);
        gtk_box_center.pack_start(clock1.get_label(), false, false, 0);
        gtk_box_right.pack_end(disk.get_label(), false, false, 0);
        gtk_box_right.pack_end(battery.get_label(), false, false, 0);
        gtk_box_right.pack_end(cpu.get_label(), false, false, 0);
        gtk_box_right.pack_end(memory.get_label(), false, false, 0);
        gtk_box_right.pack_end(pulseaudio.get_label(), false, false, 0);

        let tick = move || {
            battery.update();
            clock1.update();
            cpu.update();
            disk.update();
            memory.update();
            pulseaudio.update();

            glib::Continue(true)
        };

        gtk::timeout_add_seconds(1, tick);

        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

        thread::spawn(move || {
            for _event in Sway::get_listener().listen() {
                tx.send(()).expect("Couldn't send data to channel");
            }
        });

        rx.attach(None, move |_| {
            sway.update();

            glib::Continue(true)
        });

        window.add(&gtk_box);
        window.set_border_width(12);
        window.show_all();

        Bar {}
    }
}
