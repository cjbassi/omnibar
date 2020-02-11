use gtk::prelude::*;
use gtk::Label;

use crate::module::Module;

pub struct Clock {
    // label: Label,
}

impl Clock {
    pub fn new() -> Self {
        Self {}
    }
}

impl Module for Clock {
    fn update(&mut self) {
        unimplemented!()
    }

    fn get_label(&self) -> Label {
        let label = gtk::Label::new(Some(""));
        label.set_markup("<span font_desc=\"20.0\">GTK Layer Shell example!</span>");
        label
    }
}
