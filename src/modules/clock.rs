use chrono::Local;
use gtk::prelude::*;
use gtk::Label;

use crate::module::Module;

pub struct Clock {
    label: Label,
    format: String,
}

impl Clock {
    pub fn new(format: &str) -> Self {
        let label = gtk::Label::new(None);
        let format = format.to_string();

        Self { label, format }
    }
}

impl Module for Clock {
    fn update(&mut self) {
        self.label
            .set_markup(&Local::now().format(&self.format).to_string());
    }

    fn get_label(&self) -> &Label {
        &self.label
    }
}
