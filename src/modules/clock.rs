use gtk::prelude::*;
use gtk::Label;

use crate::module::Module;

pub struct Clock {
    label: Label,
}

impl Clock {
    pub fn new() -> Self {
        let label = gtk::Label::new(None);

        Self { label }
    }
}

impl Module for Clock {
    fn update(&mut self) {
        self.label
            .set_markup("<span font_desc=\"20.0\">GTK Layer Shell example!</span>");
    }

    fn get_label(&self) -> &Label {
        &self.label
    }
}
