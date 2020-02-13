use gtk::prelude::*;
use gtk::Label;

use crate::module::Module;

pub struct Custom {
    label: Label,

    name: String,
    text: String,
}

impl Custom {
    pub fn new(name: &str, text: &str) -> Self {
        let label = gtk::Label::new(None);
        let name = name.to_string();
        let text = text.to_string();

        Self { label, name, text }
    }
}

impl Module for Custom {
    fn update(&mut self) {
        self.label.set_markup(&self.text);
    }

    fn get_label(&self) -> &Label {
        &self.label
    }
}
