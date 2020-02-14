use std::collections::HashMap;

use battery::Manager;
use gtk::prelude::*;
use gtk::Label;
use strfmt::{strfmt_map, Formatter};

use crate::module::Module;

pub struct Battery {
    label: Label,
    format: String,

    battery: battery::Battery,
    manager: Manager,
}

impl Battery {
    pub fn new(format: &str) -> Self {
        let label = gtk::Label::new(None);
        label.set_widget_name("battery");
        let format = format.to_string();
        let manager = Manager::new().unwrap();
        let battery = manager.batteries().unwrap().next().unwrap().unwrap();

        Self {
            label,
            format,
            battery,
            manager,
        }
    }
}

impl Module for Battery {
    fn update(&mut self) {
        self.manager.refresh(&mut self.battery).unwrap();
        let percent = self.battery.state_of_charge().value as f32 * 100.0;

        let mut vars = HashMap::new();
        vars.insert("percent".to_string(), percent);

        let f = |mut fmt: Formatter| fmt.f32(*vars.get(fmt.key).unwrap());

        self.label
            .set_markup(&strfmt_map(&self.format, &f).unwrap());
    }

    fn get_label(&self) -> &Label {
        &self.label
    }
}
