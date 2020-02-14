use gtk::prelude::*;
use gtk::Label;
use i3ipc::I3Connection;
use i3ipc::I3EventListener;
use i3ipc::Subscription;

use crate::module::Module;

pub struct Sway {
    label: Label,

    connection: I3Connection,
}

impl Sway {
    pub fn new() -> Self {
        let label = gtk::Label::new(None);
        label.set_widget_name("sway");
        let connection = I3Connection::connect().unwrap();

        Self { label, connection }
    }

    pub fn get_listener() -> I3EventListener {
        let mut listener = I3EventListener::connect().unwrap();
        let subs = [Subscription::Workspace];
        listener.subscribe(&subs).unwrap();

        listener
    }
}

impl Module for Sway {
    fn update(&mut self) {
        let workspaces: Vec<String> = self
            .connection
            .get_workspaces()
            .unwrap()
            .workspaces
            .into_iter()
            .map(|workspace| workspace.name)
            .collect();

        self.label.set_markup(&workspaces.join(" "));
    }

    fn get_label(&self) -> &Label {
        &self.label
    }
}
